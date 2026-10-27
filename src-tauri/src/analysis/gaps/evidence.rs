//! Loads document details for evidence references.

use super::model::{Evidence, EvidenceRef};
use crate::error::AppResult;
use rusqlite::{params_from_iter, Connection};
use std::collections::HashMap;

struct Info {
    title: String,
    year: Option<i32>,
    source: Option<String>,
    authors: String,
    cited_by: i64,
}

fn load(conn: &Connection, ids: &[i64]) -> AppResult<HashMap<i64, Info>> {
    let mut out = HashMap::new();
    // Placeholders are generated from the count only; values are bound as parameters.
    for chunk in ids.chunks(500) {
        let marks = vec!["?"; chunk.len()].join(",");
        let mut stmt = conn.prepare(&format!(
            "SELECT id, title, year, source_name, authors, cited_by FROM papers WHERE id IN ({marks})"
        ))?;
        let rows = stmt.query_map(params_from_iter(chunk.iter()), |r| {
            Ok((r.get::<_, i64>(0)?, Info { title: r.get(1)?, year: r.get(2)?, source: r.get(3)?, authors: r.get(4)?, cited_by: r.get(5)? }))
        })?;
        for row in rows {
            let (id, info) = row?;
            out.insert(id, info);
        }
    }
    Ok(out)
}

/// Resolves references for several gaps with one query per 500 documents.
pub fn resolve_all(conn: &Connection, refs: &[Vec<EvidenceRef>]) -> AppResult<Vec<Vec<Evidence>>> {
    let mut ids: Vec<i64> = refs.iter().flatten().map(|r| r.paper_id).collect();
    ids.sort_unstable();
    ids.dedup();
    let info = load(conn, &ids)?;
    Ok(refs
        .iter()
        .map(|list| {
            list.iter()
                .filter_map(|r| {
                    let i = info.get(&r.paper_id)?;
                    Some(Evidence {
                        paper_id: r.paper_id,
                        title: i.title.clone(),
                        year: i.year,
                        source: i.source.clone(),
                        authors: i.authors.clone(),
                        cited_by: i.cited_by,
                        role: r.role,
                        quote: r.quote.clone(),
                    })
                })
                .collect()
        })
        .collect())
}

/// Most cited documents among `docs` (indices into the keyword network), newest first on ties.
pub fn top_docs(docs: &[usize], paper_ids: &[i64], years: &[Option<i32>], cited: &HashMap<i64, i64>, n: usize) -> Vec<i64> {
    let mut list: Vec<usize> = docs.to_vec();
    list.sort_by(|&a, &b| {
        let (ca, cb) = (cited.get(&paper_ids[a]).copied().unwrap_or(0), cited.get(&paper_ids[b]).copied().unwrap_or(0));
        cb.cmp(&ca).then(years[b].cmp(&years[a]))
    });
    list.into_iter().take(n).map(|d| paper_ids[d]).collect()
}
