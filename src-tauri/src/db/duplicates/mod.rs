//! Duplicate candidates in the collection, merging them, and dismissing false matches.

mod detect;
mod merge;
mod record;
#[cfg(test)]
mod tests;

use crate::error::{AppError, AppResult};
pub use merge::merge;
pub use record::DupPaper;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::collections::HashSet;

const MAX_PAIRS: usize = 300;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DupPair {
    pub a: DupPaper,
    pub b: DupPaper,
    /// "same_title" (same title, different year) or "similar_title".
    pub reason: &'static str,
    pub similarity: f64,
    /// Suggested record to keep: the published version with the richer metadata.
    pub keep: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateList {
    pub documents: usize,
    pub pairs: Vec<DupPair>,
    pub truncated: bool,
}

fn dismissed(conn: &Connection) -> AppResult<HashSet<(i64, i64)>> {
    let mut stmt = conn.prepare("SELECT a, b FROM dup_dismissed")?;
    let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
    Ok(rows.collect::<Result<HashSet<_>, _>>()?)
}

pub fn list(conn: &Connection) -> AppResult<DuplicateList> {
    let papers = record::load(conn)?;
    let skip = dismissed(conn)?;
    let mut found: Vec<(usize, usize, f64, bool)> = detect::pairs(&papers)
        .into_iter()
        .filter(|&(a, b, ..)| !skip.contains(&ordered(papers[a].id, papers[b].id)))
        .collect();
    found.sort_by(|x, y| y.3.cmp(&x.3).then(y.2.total_cmp(&x.2)).then(papers[x.0].id.cmp(&papers[y.0].id)));
    let truncated = found.len() > MAX_PAIRS;
    let pairs = found
        .into_iter()
        .take(MAX_PAIRS)
        .map(|(a, b, sim, same)| {
            let (pa, pb) = (&papers[a], &papers[b]);
            let keep = if pb.completeness() > pa.completeness() { pb.id } else { pa.id };
            DupPair { a: pa.clone(), b: pb.clone(), reason: if same { "same_title" } else { "similar_title" }, similarity: sim, keep }
        })
        .collect();
    Ok(DuplicateList { documents: papers.len(), pairs, truncated })
}

fn ordered(a: i64, b: i64) -> (i64, i64) {
    (a.min(b), a.max(b))
}

pub fn check_ids(a: i64, b: i64) -> AppResult<()> {
    if a <= 0 || b <= 0 || a == b {
        return Err(AppError::Invalid("pasangan dokumen tidak sah".into()));
    }
    Ok(())
}

/// Marks a pair as not duplicates so it is no longer suggested.
pub fn dismiss(conn: &Connection, a: i64, b: i64) -> AppResult<()> {
    check_ids(a, b)?;
    let (x, y) = ordered(a, b);
    let changed = conn.execute(
        "INSERT OR IGNORE INTO dup_dismissed (a, b)
         SELECT ?1, ?2 WHERE (SELECT COUNT(*) FROM papers WHERE id IN (?1, ?2)) = 2",
        params![x, y],
    )?;
    if changed == 0 && !conn.prepare("SELECT 1 FROM dup_dismissed WHERE a = ?1 AND b = ?2")?.exists(params![x, y])? {
        return Err(AppError::Invalid("dokumen tidak ditemukan".into()));
    }
    Ok(())
}
