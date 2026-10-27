//! Most relevant affiliations: documents per institution (each document counts once
//! per institution), with country, citations, and cumulative production of the top ones.

use super::current_year;
use super::source_impact::Series;
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::{BTreeSet, HashMap};

const DYNAMICS_TOP: usize = 6;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliationRow {
    pub label: String,
    pub country: Option<String>,
    pub documents: usize,
    pub citations: i64,
    pub first_year: Option<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Affiliations {
    pub documents: i64,
    /// Documents with at least one known institution.
    pub with_affiliation: i64,
    pub rows: Vec<AffiliationRow>,
    pub years: Vec<i32>,
    pub dynamics: Vec<Series>,
}

#[derive(Default)]
struct Acc {
    names: HashMap<String, usize>,
    countries: HashMap<String, usize>,
    papers: Vec<(Option<i32>, i64)>,
}

fn most_common(map: &HashMap<String, usize>) -> Option<String> {
    map.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))).map(|(k, _)| k.clone())
}

pub fn compute(conn: &Connection) -> AppResult<Affiliations> {
    let now = current_year(conn)?;
    let (documents, with_affiliation): (i64, i64) = conn.query_row(
        "SELECT COUNT(*), COUNT(*) FILTER (WHERE EXISTS (SELECT 1 FROM paper_affiliations f WHERE f.paper_id = papers.id)) FROM papers",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    let mut stmt = conn.prepare("SELECT f.name_key, f.name, f.country, p.year, p.cited_by FROM paper_affiliations f JOIN papers p ON p.id = f.paper_id")?;
    let mut acc: HashMap<String, Acc> = HashMap::new();
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let e = acc.entry(r.get(0)?).or_default();
        *e.names.entry(r.get(1)?).or_default() += 1;
        if let Some(c) = r.get::<_, Option<String>>(2)? {
            *e.countries.entry(c).or_default() += 1;
        }
        e.papers.push((r.get(3)?, r.get(4)?));
    }
    let mut ranked: Vec<(String, Acc)> = acc.into_iter().collect();
    ranked.sort_by(|a, b| b.1.papers.len().cmp(&a.1.papers.len()).then(a.0.cmp(&b.0)));
    let years: Vec<i32> = ranked.iter().flat_map(|(_, a)| a.papers.iter().filter_map(|p| p.0)).filter(|y| *y <= now).collect::<BTreeSet<_>>().into_iter().collect();
    let out: Vec<AffiliationRow> = ranked
        .iter()
        .map(|(_, a)| AffiliationRow {
            label: most_common(&a.names).unwrap_or_default(),
            country: most_common(&a.countries),
            documents: a.papers.len(),
            citations: a.papers.iter().map(|p| p.1).sum(),
            first_year: a.papers.iter().filter_map(|p| p.0).min(),
        })
        .collect();
    let dynamics = ranked
        .iter()
        .zip(&out)
        .take(DYNAMICS_TOP)
        .map(|((_, a), row)| Series {
            label: row.label.clone(),
            cumulative: years.iter().map(|y| a.papers.iter().filter(|p| p.0.is_some_and(|py| py <= *y)).count() as i64).collect(),
        })
        .collect();
    Ok(Affiliations { documents, with_affiliation, rows: out, years, dynamics })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};
    use crate::geo::affil::from_text;

    fn paper(id: &str, year: i32, affs: &[&str]) -> NewPaper {
        let affiliations = crate::geo::affil::collect(affs.iter().map(|a| from_text(a, false)));
        NewPaper { ext_id: id.into(), title: id.into(), year: Some(year), cited_by: 10, affiliations, ..Default::default() }
    }

    #[test]
    fn ranks_institutions_across_spellings() {
        let mut conn = open_memory();
        let items = [
            paper("a", 2020, &["[Rahman, M] Univ Malaya, Kuala Lumpur, Malaysia", "Stanford Univ, Stanford, CA 94305 USA"]),
            paper("b", 2022, &["Dept CS, University of Malaya, Kuala Lumpur, Malaysia"]),
            paper("c", 2022, &[]),
        ];
        insert_many(&mut conn, &items, "Test").unwrap();
        let a = compute(&conn).unwrap();
        assert_eq!((a.documents, a.with_affiliation), (3, 2));
        assert_eq!((a.rows[0].documents, a.rows[0].country.as_deref(), a.rows[0].citations), (2, Some("MY"), 20));
        assert_eq!(a.years, vec![2020, 2022]);
        assert_eq!(a.dynamics[0].cumulative, vec![1, 2]);
    }
}
