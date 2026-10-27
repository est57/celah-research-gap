//! Documents of the (filtered) collection as searchable text, and helpers to list them.

use crate::error::AppResult;
use super::stems;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::BTreeMap;

pub struct Doc {
    pub id: i64,
    pub title: String,
    pub authors: String,
    pub year: Option<i32>,
    pub cited_by: i64,
    /// Stemmed words of the title, abstract and both keyword fields.
    pub tokens: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocRef {
    pub id: i64,
    pub title: String,
    pub authors: String,
    pub year: Option<i32>,
    pub cited_by: i64,
}

pub fn load(conn: &Connection) -> AppResult<Vec<Doc>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, authors, year, cited_by,
                title || ' ' || COALESCE(abstract, '') || ' ' || COALESCE(keywords, '') || ' ' || COALESCE(keywords_plus, '')
         FROM papers",
    )?;
    let rows = stmt.query_map([], |r| {
        let text: String = r.get(5)?;
        Ok(Doc { id: r.get(0)?, title: r.get(1)?, authors: r.get(2)?, year: r.get(3)?, cited_by: r.get(4)?, tokens: stems(&text) })
    })?;
    Ok(rows.collect::<Result<_, _>>()?)
}

/// Most cited documents whose flags pass `keep`.
pub fn top(all: &[Doc], flags: &[(bool, bool)], keep: impl Fn(&(bool, bool)) -> bool, n: usize) -> Vec<DocRef> {
    let mut picked: Vec<&Doc> = all.iter().zip(flags).filter(|(_, f)| keep(f)).map(|(d, _)| d).collect();
    picked.sort_by(|a, b| b.cited_by.cmp(&a.cited_by).then(a.id.cmp(&b.id)));
    picked
        .into_iter()
        .take(n)
        .map(|d| DocRef { id: d.id, title: d.title.clone(), authors: d.authors.clone(), year: d.year, cited_by: d.cited_by })
        .collect()
}

/// Years with any matching document, and per year the counts for A, B and both.
pub fn per_year(all: &[Doc], flags: &[(bool, bool)]) -> (Vec<i32>, [Vec<usize>; 3]) {
    let mut by: BTreeMap<i32, [usize; 3]> = BTreeMap::new();
    for (d, f) in all.iter().zip(flags) {
        let Some(y) = d.year else { continue };
        if f.0 || f.1 {
            let e = by.entry(y).or_default();
            e[0] += usize::from(f.0);
            e[1] += usize::from(f.1);
            e[2] += usize::from(f.0 && f.1);
        }
    }
    let years: Vec<i32> = by.keys().copied().collect();
    let col = |i: usize| by.values().map(|v| v[i]).collect();
    (years, [col(0), col(1), col(2)])
}
