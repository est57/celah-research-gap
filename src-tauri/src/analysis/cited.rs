//! Most cited documents, globally (citations reported by the source) and locally
//! (citations from documents in the collection), with year-normalized citations.

use super::conceptual::in_range;
use super::current_year;
use super::historiograph::links;
use crate::error::{AppError, AppResult};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CitedParams {
    /// "global", "local" or "normalized".
    pub sort: String,
    pub limit: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CitedDoc {
    pub id: i64,
    pub title: String,
    pub authors: String,
    pub year: Option<i32>,
    pub source: Option<String>,
    pub doi: Option<String>,
    pub global: i64,
    pub local: i64,
    pub per_year: f64,
    /// Global citations divided by the mean of documents from the same year (1 = average).
    pub normalized: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cited {
    pub documents: usize,
    /// Citation links found inside the collection; 0 means local counts are unavailable.
    pub local_links: usize,
    pub docs: Vec<CitedDoc>,
}

pub fn compute(conn: &Connection, p: &CitedParams) -> AppResult<Cited> {
    let limit = in_range(p.limit, 10, 200, "jumlah dokumen")? as usize;
    if !["global", "local", "normalized"].contains(&p.sort.as_str()) {
        return Err(AppError::Invalid("urutan tidak dikenal".into()));
    }
    let found = links::find(conn)?;
    let mut local: HashMap<i64, i64> = HashMap::new();
    for &(_, cited) in &found.links {
        *local.entry(cited).or_default() += 1;
    }
    let now = current_year(conn)?;
    let mut stmt = conn.prepare("SELECT id, title, authors, year, source_name, doi, cited_by FROM papers")?;
    let mut docs = stmt
        .query_map([], |r| {
            let (id, year, global): (i64, Option<i32>, i64) = (r.get(0)?, r.get(3)?, r.get(6)?);
            Ok(CitedDoc {
                id,
                title: r.get(1)?,
                authors: r.get(2)?,
                year,
                source: r.get(4)?,
                doi: r.get(5)?,
                global,
                local: local.get(&id).copied().unwrap_or(0),
                per_year: year.map_or(0.0, |y| global as f64 / f64::from((now - y + 1).max(1))),
                normalized: 0.0,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut by_year: HashMap<Option<i32>, (i64, i64)> = HashMap::new();
    for d in &docs {
        let e = by_year.entry(d.year).or_default();
        e.0 += d.global;
        e.1 += 1;
    }
    for d in &mut docs {
        let (sum, n) = by_year[&d.year];
        let mean = sum as f64 / n as f64;
        d.normalized = if mean > 0.0 { d.global as f64 / mean } else { 0.0 };
    }
    let documents = docs.len();
    match p.sort.as_str() {
        "local" => docs.sort_by(|a, b| b.local.cmp(&a.local).then(b.global.cmp(&a.global))),
        "normalized" => docs.sort_by(|a, b| b.normalized.total_cmp(&a.normalized).then(b.global.cmp(&a.global))),
        _ => docs.sort_by(|a, b| b.global.cmp(&a.global).then(b.local.cmp(&a.local))),
    }
    docs.truncate(limit);
    Ok(Cited { documents, local_links: found.links.len(), docs })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    fn paper(ext: &str, year: i32, cited_by: i64, cites: &[&str]) -> NewPaper {
        NewPaper { ext_id: ext.into(), title: ext.into(), year: Some(year), cited_by, cites: cites.iter().map(|c| c.to_string()).collect(), ..Default::default() }
    }

    #[test]
    fn ranks_globally_locally_and_by_year() {
        let mut conn = open_memory();
        let items = [
            paper("openalex:W1", 2018, 900, &[]),
            paper("openalex:W2", 2018, 600, &[]),
            paper("openalex:W3", 2024, 60, &["openalex:W2"]),
            paper("openalex:W4", 2024, 20, &["openalex:W2", "openalex:W3"]),
        ];
        insert_many(&mut conn, &items, "Test").unwrap();
        let run = |sort: &str| compute(&conn, &CitedParams { sort: sort.into(), limit: 10 }).unwrap();
        assert_eq!(run("global").docs[0].title, "openalex:W1");
        let local = run("local");
        assert_eq!((local.docs[0].title.as_str(), local.docs[0].local, local.local_links), ("openalex:W2", 2, 3));
        let norm = run("normalized");
        assert_eq!(norm.docs[0].title, "openalex:W3", "60 against a 2024 mean of 40 (1.5) beats 900 against a 2018 mean of 750 (1.2)");
        assert!((norm.docs[0].normalized - 1.5).abs() < 1e-9);
        assert!(compute(&conn, &CitedParams { sort: "x".into(), limit: 10 }).is_err());
        assert!(compute(&conn, &CitedParams { sort: "global".into(), limit: 5 }).is_err());
    }
}
