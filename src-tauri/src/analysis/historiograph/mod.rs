//! Historiograph (Garfield): the documents most cited inside the collection (local
//! citation score, LCS) and the direct citations among them, placed by year.

pub(crate) mod links;
pub(crate) mod index;
pub(crate) mod refmatch;
#[cfg(test)]
mod tests;

use super::conceptual::in_range;
use super::louvain;
use crate::error::AppResult;
use crate::report::names::surname;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HistParams {
    pub nodes: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistNode {
    pub id: i64,
    /// "Surname Year".
    pub label: String,
    pub title: String,
    pub year: i32,
    pub source: Option<String>,
    pub doi: Option<String>,
    /// Citations from documents in the collection.
    pub lcs: i64,
    /// Citations overall, as reported by the source.
    pub gcs: i64,
    pub cluster: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Historiograph {
    pub documents: i64,
    /// Matched citation links in the collection, and how they were found.
    pub links: usize,
    pub by_id: usize,
    pub by_doi: usize,
    pub by_name: usize,
    pub nodes: Vec<HistNode>,
    /// (citing, cited) as indices into `nodes`.
    pub edges: Vec<(usize, usize)>,
}

pub fn compute(conn: &Connection, p: &HistParams) -> AppResult<Historiograph> {
    let top = in_range(p.nodes, 10, 100, "jumlah dokumen")? as usize;
    let documents: i64 = conn.query_row("SELECT COUNT(*) FROM papers", [], |r| r.get(0))?;
    let found = links::find(conn)?;
    let mut lcs: HashMap<i64, i64> = HashMap::new();
    for &(_, cited) in &found.links {
        *lcs.entry(cited).or_default() += 1;
    }
    let mut stmt = conn.prepare(
        "SELECT p.title, p.year, p.source_name, p.doi, p.cited_by,
                COALESCE((SELECT a.name FROM paper_authors a WHERE a.paper_id = p.id ORDER BY a.position LIMIT 1), '')
         FROM papers p WHERE p.id = ?1 AND p.year IS NOT NULL",
    )?;
    let mut ranked: Vec<(i64, i64)> = lcs.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let mut nodes = Vec::new();
    for (id, score) in ranked {
        if nodes.len() >= top {
            break;
        }
        let row = stmt.query_row(params![id], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i32>(1)?, r.get(2)?, r.get(3)?, r.get::<_, i64>(4)?, r.get::<_, String>(5)?))
        });
        let Ok((title, year, source, doi, gcs, first)) = row else { continue };
        let who = if first.is_empty() { title.split_whitespace().next().unwrap_or("").to_string() } else { surname(&first) };
        nodes.push(HistNode { id, label: format!("{who} {year}"), title, year, source, doi, lcs: score, gcs, cluster: 0 });
    }
    nodes.sort_by(|a, b| a.year.cmp(&b.year).then(b.lcs.cmp(&a.lcs)).then(a.id.cmp(&b.id)));
    let index: HashMap<i64, usize> = nodes.iter().enumerate().map(|(i, n)| (n.id, i)).collect();
    let edges: Vec<(usize, usize)> = found.links.iter().filter_map(|(a, b)| Some((*index.get(a)?, *index.get(b)?))).collect();
    let weighted: Vec<(usize, usize, f64)> = edges.iter().map(|&(a, b)| (a.min(b), a.max(b), 1.0)).collect();
    for (node, c) in nodes.iter_mut().zip(louvain::communities(index.len(), &weighted)) {
        node.cluster = c;
    }
    Ok(Historiograph { documents, links: found.links.len(), by_id: found.by_id, by_doi: found.by_doi, by_name: found.by_name, nodes, edges })
}
