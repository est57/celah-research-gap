//! Author productivity over time and local impact (h, g, m indices).

use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

/// One author's documents: (year, citations).
pub struct AuthorPapers {
    pub label: String,
    pub papers: Vec<(Option<i32>, i64)>,
}

/// Groups documents by normalized author name. Each paper counts once per author.
pub fn load(conn: &Connection) -> AppResult<HashMap<String, AuthorPapers>> {
    let mut stmt = conn.prepare(
        "SELECT a.name_key, MIN(a.name), p.year, p.cited_by FROM paper_authors a
         JOIN papers p ON p.id = a.paper_id GROUP BY a.name_key, p.id",
    )?;
    let mut out: HashMap<String, AuthorPapers> = HashMap::new();
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let (key, name): (String, String) = (r.get(0)?, r.get(1)?);
        let entry = out.entry(key).or_insert_with(|| AuthorPapers { label: name, papers: Vec::new() });
        entry.papers.push((r.get(2)?, r.get(3)?));
    }
    Ok(out)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Impact {
    pub label: String,
    pub h: usize,
    pub g: usize,
    pub m: Option<f64>,
    pub citations: i64,
    pub documents: usize,
    pub first_year: Option<i32>,
}

/// h: h papers with at least h citations. g: top g papers have at least g^2 citations together.
pub fn h_g(citations: &[i64]) -> (usize, usize) {
    let mut sorted: Vec<i64> = citations.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let h = sorted.iter().enumerate().take_while(|(i, c)| **c >= (*i as i64 + 1)).count();
    let mut sum = 0;
    let mut g = 0;
    for (i, c) in sorted.iter().enumerate() {
        sum += c;
        if sum >= ((i + 1) * (i + 1)) as i64 {
            g = i + 1;
        }
    }
    (h, g)
}

pub fn impact(a: &AuthorPapers, current_year: i32) -> Impact {
    let cites: Vec<i64> = a.papers.iter().map(|(_, c)| *c).collect();
    let (h, g) = h_g(&cites);
    let first_year = a.papers.iter().filter_map(|(y, _)| *y).min();
    Impact {
        label: a.label.clone(),
        h,
        g,
        m: first_year.map(|y| h as f64 / f64::from((current_year - y + 1).max(1))),
        citations: cites.iter().sum(),
        documents: a.papers.len(),
        first_year,
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YearCell {
    pub year: i32,
    pub documents: i64,
    /// Citations of that year's documents divided by the years since publication.
    pub tc_per_year: f64,
}

#[derive(Serialize)]
pub struct Timeline {
    pub label: String,
    pub documents: usize,
    pub years: Vec<YearCell>,
}

pub fn timeline(a: &AuthorPapers, current_year: i32) -> Timeline {
    let mut by_year: BTreeMap<i32, (i64, i64)> = BTreeMap::new();
    for (year, cites) in &a.papers {
        if let Some(y) = year {
            let e = by_year.entry(*y).or_default();
            e.0 += 1;
            e.1 += cites;
        }
    }
    let years = by_year
        .into_iter()
        .map(|(year, (documents, tc))| YearCell {
            year,
            documents,
            tc_per_year: tc as f64 / f64::from((current_year - year + 1).max(1)),
        })
        .collect();
    Timeline { label: a.label.clone(), documents: a.papers.len(), years }
}

#[cfg(test)]
mod tests {
    #[test]
    fn computes_h_and_g() {
        assert_eq!(super::h_g(&[10, 8, 5, 4, 3]), (4, 5));
        assert_eq!(super::h_g(&[0, 0]), (0, 0));
        assert_eq!(super::h_g(&[25, 0, 0]), (1, 3));
    }
}
