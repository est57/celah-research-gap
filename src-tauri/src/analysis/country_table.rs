//! Country table: production, citations and collaboration per country, and the
//! cumulative production of the top countries (bibliometrix Countries' Production
//! over Time and Most Cited Countries).

use super::current_year;
use super::source_impact::Series;
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};

const DYNAMICS_TOP: usize = 6;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryRow {
    /// ISO alpha-2 code.
    pub code: String,
    /// Documents with at least one author from the country.
    pub documents: usize,
    pub citations: i64,
    pub avg_citations: f64,
    /// Documents whose corresponding author is from the country.
    pub corresponding: usize,
    /// Of those, single- and multiple-country publications.
    pub scp: usize,
    pub mcp: usize,
    pub first_year: Option<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryTable {
    pub documents: i64,
    pub with_country: usize,
    pub rows: Vec<CountryRow>,
    pub years: Vec<i32>,
    pub dynamics: Vec<Series>,
}

struct Paper {
    year: Option<i32>,
    cited_by: i64,
    codes: Vec<String>,
    corresponding: Option<String>,
}

fn load(conn: &Connection) -> AppResult<Vec<Paper>> {
    let mut stmt = conn.prepare(
        "SELECT c.paper_id, c.code, c.corresponding, p.year, p.cited_by
         FROM paper_countries c JOIN papers p ON p.id = c.paper_id ORDER BY c.paper_id, c.code",
    )?;
    let mut by_paper: BTreeMap<i64, Paper> = BTreeMap::new();
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let (id, code, corr): (i64, String, bool) = (r.get(0)?, r.get(1)?, r.get(2)?);
        let p = by_paper.entry(id).or_insert(Paper { year: r.get(3)?, cited_by: r.get(4)?, codes: Vec::new(), corresponding: None });
        if corr {
            p.corresponding = Some(code.clone());
        }
        p.codes.push(code);
    }
    Ok(by_paper.into_values().collect())
}

pub fn compute(conn: &Connection) -> AppResult<CountryTable> {
    let now = current_year(conn)?;
    let documents: i64 = conn.query_row("SELECT COUNT(*) FROM papers", [], |r| r.get(0))?;
    let papers = load(conn)?;
    let mut by_code: HashMap<&str, Vec<&Paper>> = HashMap::new();
    for p in &papers {
        for c in &p.codes {
            by_code.entry(c.as_str()).or_default().push(p);
        }
    }
    let mut rows: Vec<CountryRow> = by_code
        .iter()
        .map(|(&code, list)| {
            let citations: i64 = list.iter().map(|p| p.cited_by).sum();
            let led: Vec<&&Paper> = list.iter().filter(|p| p.corresponding.as_deref() == Some(code)).collect();
            let mcp = led.iter().filter(|p| p.codes.len() > 1).count();
            CountryRow {
                code: code.to_string(),
                documents: list.len(),
                citations,
                avg_citations: citations as f64 / list.len() as f64,
                corresponding: led.len(),
                scp: led.len() - mcp,
                mcp,
                first_year: list.iter().filter_map(|p| p.year).min(),
            }
        })
        .collect();
    rows.sort_by(|a, b| b.documents.cmp(&a.documents).then(b.citations.cmp(&a.citations)).then(a.code.cmp(&b.code)));

    let years: Vec<i32> = papers.iter().filter_map(|p| p.year).filter(|y| *y <= now).collect::<BTreeSet<_>>().into_iter().collect();
    let dynamics = rows
        .iter()
        .take(DYNAMICS_TOP)
        .map(|row| {
            let list = &by_code[row.code.as_str()];
            Series {
                label: row.code.clone(),
                cumulative: years.iter().map(|y| list.iter().filter(|p| p.year.is_some_and(|py| py <= *y)).count() as i64).collect(),
            }
        })
        .collect();
    Ok(CountryTable { documents, with_country: papers.len(), rows, years, dynamics })
}

#[cfg(test)]
mod tests {
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    fn paper(id: &str, year: i32, cited: i64, countries: &[&str]) -> NewPaper {
        NewPaper {
            ext_id: id.into(),
            title: id.into(),
            year: Some(year),
            cited_by: cited,
            countries: countries.iter().map(|c| c.to_string()).collect(),
            corresponding_country: countries.first().map(|c| c.to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn counts_documents_citations_and_collaboration_per_country() {
        let mut conn = open_memory();
        let items = [
            paper("a", 2020, 10, &["MY", "US"]),
            paper("b", 2022, 4, &["MY"]),
            paper("c", 2022, 6, &["US"]),
            paper("d", 2023, 0, &[]),
        ];
        insert_many(&mut conn, &items, "Test").unwrap();
        let t = super::compute(&conn).unwrap();
        assert_eq!((t.documents, t.with_country), (4, 3));
        let my = t.rows.iter().find(|r| r.code == "MY").unwrap();
        assert_eq!((my.documents, my.citations, my.avg_citations), (2, 14, 7.0));
        assert_eq!((my.corresponding, my.scp, my.mcp, my.first_year), (2, 1, 1, Some(2020)));
        let us = t.rows.iter().find(|r| r.code == "US").unwrap();
        assert_eq!((us.documents, us.corresponding, us.citations), (2, 1, 16));
        assert_eq!(t.rows[0].code, "US", "ties on documents go to more citations");
        assert_eq!(t.years, vec![2020, 2022]);
        assert_eq!(t.dynamics[1].cumulative, vec![1, 2]);
    }
}
