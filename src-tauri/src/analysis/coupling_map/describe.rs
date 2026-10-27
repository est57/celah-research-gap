//! Per-document facts used to describe coupling clusters: citations normalized by
//! publication year, keywords, and a short title line.

use super::super::terms_with_label;
use super::super::words::text;
use crate::error::AppResult;
use rusqlite::Connection;
use std::collections::HashMap;

pub struct DocInfo {
    pub title: String,
    pub year: Option<i32>,
    pub cited_by: i64,
    /// Citations divided by the mean of documents from the same year (1.0 = average).
    pub normalized: f64,
    /// (key, label) of author keywords, else Keywords Plus, else two-word phrases of
    /// the title and abstract (Crossref and arXiv records carry no keywords).
    pub keywords: Vec<(String, String)>,
}

pub fn load(conn: &Connection) -> AppResult<HashMap<i64, DocInfo>> {
    let mut stmt = conn.prepare("SELECT id, title, year, cited_by, COALESCE(NULLIF(keywords, ''), NULLIF(keywords_plus, '')), COALESCE(abstract, '') FROM papers")?;
    let mut docs: HashMap<i64, DocInfo> = HashMap::new();
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let (title, raw, abs): (String, Option<String>, String) = (r.get(1)?, r.get(4)?, r.get(5)?);
        let keywords = match raw {
            Some(k) => terms_with_label(Some(&k)).map(|(key, l)| (text::key(&key), l)).collect(),
            None => text::grams(&format!("{title}. {abs}"), 2).into_iter().map(|g| (text::key(&g), g)).collect(),
        };
        docs.insert(r.get(0)?, DocInfo { title, year: r.get(2)?, cited_by: r.get(3)?, normalized: 0.0, keywords });
    }
    let mut per_year: HashMap<Option<i32>, (i64, i64)> = HashMap::new();
    for d in docs.values() {
        let e = per_year.entry(d.year).or_default();
        e.0 += d.cited_by;
        e.1 += 1;
    }
    for d in docs.values_mut() {
        let (sum, n) = per_year[&d.year];
        let mean = sum as f64 / n as f64;
        d.normalized = if mean > 0.0 { d.cited_by as f64 / mean } else { 1.0 };
    }
    Ok(docs)
}

/// The `n` keywords most specific to each cluster: count in the cluster times
/// ln((clusters + 1) / clusters using the keyword), a smoothed tf-idf over clusters,
/// so a phrase shared by every cluster ranks below one that sets a cluster apart.
pub fn labels(clusters: &[Vec<&DocInfo>], n: usize) -> Vec<Vec<String>> {
    let mut spread: HashMap<&str, usize> = HashMap::new();
    let mut counts: Vec<HashMap<&str, (usize, &str)>> = Vec::with_capacity(clusters.len());
    for docs in clusters {
        let mut c: HashMap<&str, (usize, &str)> = HashMap::new();
        for d in docs {
            for (k, l) in &d.keywords {
                c.entry(k.as_str()).or_insert((0, l.as_str())).0 += 1;
            }
        }
        for k in c.keys() {
            *spread.entry(k).or_default() += 1;
        }
        counts.push(c);
    }
    let total = clusters.len() as f64 + 1.0;
    counts
        .into_iter()
        .map(|c| {
            let mut scored: Vec<(f64, &str)> = c
                .iter()
                .map(|(k, &(count, label))| (count as f64 * (total / spread[k] as f64).ln(), label))
                .collect();
            scored.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(b.1)));
            scored.into_iter().take(n).map(|(_, l)| l.to_string()).collect()
        })
        .collect()
}
