//! Every author with productivity and local impact: documents, fractionalized
//! documents (each document split equally among its authors), citations, h, g, m.

use super::authors::h_g;
use super::current_year;
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;

/// Upper bound on rows sent to the UI; larger collections keep the most productive.
const MAX_ROWS: usize = 20_000;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorRow {
    pub label: String,
    pub documents: usize,
    pub fractional: f64,
    pub citations: i64,
    pub h: usize,
    pub g: usize,
    pub m: Option<f64>,
    pub first_year: Option<i32>,
    pub last_year: Option<i32>,
}

struct Acc {
    label: String,
    fractional: f64,
    papers: Vec<(Option<i32>, i64)>,
}

pub fn compute(conn: &Connection) -> AppResult<Vec<AuthorRow>> {
    let now = current_year(conn)?;
    let mut stmt = conn.prepare(
        "SELECT a.name_key, MIN(a.name), p.year, p.cited_by,
                (SELECT COUNT(DISTINCT b.name_key) FROM paper_authors b WHERE b.paper_id = p.id)
         FROM paper_authors a JOIN papers p ON p.id = a.paper_id GROUP BY a.name_key, p.id",
    )?;
    let mut acc: HashMap<String, Acc> = HashMap::new();
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let (key, name, n): (String, String, i64) = (r.get(0)?, r.get(1)?, r.get(4)?);
        let e = acc.entry(key).or_insert_with(|| Acc { label: name, fractional: 0.0, papers: Vec::new() });
        e.fractional += 1.0 / n.max(1) as f64;
        e.papers.push((r.get(2)?, r.get(3)?));
    }
    let mut out: Vec<AuthorRow> = acc
        .into_values()
        .map(|a| {
            let cites: Vec<i64> = a.papers.iter().map(|p| p.1).collect();
            let (h, g) = h_g(&cites);
            let years = a.papers.iter().filter_map(|p| p.0);
            let first_year = years.clone().min();
            AuthorRow {
                label: a.label,
                documents: a.papers.len(),
                fractional: a.fractional,
                citations: cites.iter().sum(),
                h,
                g,
                m: first_year.map(|y| h as f64 / f64::from((now - y + 1).max(1))),
                first_year,
                last_year: years.max(),
            }
        })
        .collect();
    out.sort_by(|a, b| b.documents.cmp(&a.documents).then(b.citations.cmp(&a.citations)).then(a.label.cmp(&b.label)));
    out.truncate(MAX_ROWS);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    fn paper(id: &str, year: i32, cited: i64, authors: &[&str], source: &str) -> NewPaper {
        NewPaper {
            ext_id: id.into(),
            title: id.into(),
            year: Some(year),
            cited_by: cited,
            source_name: Some(source.into()),
            author_names: authors.iter().map(|a| a.to_string()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn counts_fractional_documents_and_source_impact() {
        let mut conn = open_memory();
        let items = [
            paper("a", 2020, 10, &["Chen L", "Rahman M"], "PLOS ONE"),
            paper("b", 2021, 5, &["Chen L"], "plos one "),
            paper("c", 2022, 1, &["Chen L", "Okafor C", "Silva R", "Lee J"], "IEEE Access"),
        ];
        insert_many(&mut conn, &items, "Test").unwrap();
        let authors = compute(&conn).unwrap();
        let chen = &authors[0];
        assert_eq!((chen.label.as_str(), chen.documents, chen.h, chen.citations), ("Chen L", 3, 2, 16));
        assert!((chen.fractional - (0.5 + 1.0 + 0.25)).abs() < 1e-9);
        assert_eq!((chen.first_year, chen.last_year), (Some(2020), Some(2022)));

        let s = super::super::source_impact::compute(&conn).unwrap();
        let plos = s.sources.iter().find(|r| r.label.to_lowercase().starts_with("plos")).unwrap();
        assert_eq!((plos.documents, plos.citations, plos.h, plos.zone), (2, 15, 2, 1));
        assert_eq!(s.years, vec![2020, 2021, 2022]);
        assert_eq!(s.dynamics[0].cumulative, vec![1, 2, 2]);
    }
}
