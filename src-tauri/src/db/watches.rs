//! Saved searches (topic monitor) and the new documents each run found.

use crate::error::AppResult;
use crate::search::Source;
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;

pub const MAX_WATCHES: i64 = 50;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Watch {
    pub id: i64,
    pub query: String,
    pub sources: Vec<Source>,
    pub doc_type: Option<String>,
    pub year_from: i32,
    pub max_results: u32,
    pub created_at: String,
    pub last_run_at: Option<String>,
    /// Documents the last run added.
    pub last_new: i64,
    /// Documents added by every run so far that are still in the collection.
    pub total_new: i64,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    pub id: i64,
    pub title: String,
    pub authors: String,
    pub year: Option<i32>,
    pub source_name: Option<String>,
    pub origin: String,
    pub found_at: String,
}

pub fn count(conn: &Connection) -> AppResult<i64> {
    Ok(conn.query_row("SELECT COUNT(*) FROM watches", [], |r| r.get(0))?)
}

pub fn create(conn: &Connection, query: &str, sources: &[Source], doc_type: Option<&str>, year_from: i32, max_results: u32) -> AppResult<i64> {
    let sources = serde_json::to_string(sources).map_err(|_| crate::error::AppError::Internal)?;
    conn.execute(
        "INSERT INTO watches (query, sources, doc_type, year_from, max_results) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![query, sources, doc_type, year_from, max_results],
    )?;
    Ok(conn.last_insert_rowid())
}

const SELECT: &str = "SELECT w.id, w.query, w.sources, w.doc_type, w.year_from, w.max_results, w.created_at, w.last_run_at, w.last_new,
    (SELECT COUNT(*) FROM watch_hits h WHERE h.watch_id = w.id) FROM watches w";

fn row(r: &rusqlite::Row) -> rusqlite::Result<Watch> {
    let sources: String = r.get(2)?;
    Ok(Watch {
        id: r.get(0)?,
        query: r.get(1)?,
        // A stored list that no longer parses is shown without sources rather than failing the page.
        sources: serde_json::from_str(&sources).unwrap_or_default(),
        doc_type: r.get(3)?,
        year_from: r.get(4)?,
        max_results: r.get(5)?,
        created_at: r.get(6)?,
        last_run_at: r.get(7)?,
        last_new: r.get(8)?,
        total_new: r.get(9)?,
    })
}

pub fn list(conn: &Connection) -> AppResult<Vec<Watch>> {
    let mut stmt = conn.prepare(&format!("{SELECT} ORDER BY w.id DESC"))?;
    let rows = stmt.query_map([], row)?;
    Ok(rows.collect::<Result<_, _>>()?)
}

pub fn get(conn: &Connection, id: i64) -> AppResult<Option<Watch>> {
    Ok(conn.query_row(&format!("{SELECT} WHERE w.id = ?1"), params![id], row).optional()?)
}

pub fn delete(conn: &Connection, id: i64) -> AppResult<bool> {
    Ok(conn.execute("DELETE FROM watches WHERE id = ?1", params![id])? > 0)
}

/// Records a finished run and the documents it added.
pub fn record_run(conn: &mut Connection, id: i64, new_ids: &[i64]) -> AppResult<()> {
    let tx = conn.transaction()?;
    {
        let mut hit = tx.prepare_cached("INSERT OR IGNORE INTO watch_hits (watch_id, paper_id) VALUES (?1, ?2)")?;
        for paper in new_ids {
            hit.execute(params![id, paper])?;
        }
    }
    tx.execute("UPDATE watches SET last_run_at = datetime('now'), last_new = ?2 WHERE id = ?1", params![id, new_ids.len() as i64])?;
    tx.commit()?;
    Ok(())
}

/// Documents a watch added, newest first.
pub fn hits(conn: &Connection, id: i64, limit: u32) -> AppResult<Vec<Hit>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.title, p.authors, p.year, p.source_name, p.origin, h.found_at FROM watch_hits h
         JOIN papers p ON p.id = h.paper_id WHERE h.watch_id = ?1 ORDER BY h.found_at DESC, p.year DESC, p.id DESC LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![id, limit], |r| {
        Ok(Hit { id: r.get(0)?, title: r.get(1)?, authors: r.get(2)?, year: r.get(3)?, source_name: r.get(4)?, origin: r.get(5)?, found_at: r.get(6)? })
    })?;
    Ok(rows.collect::<Result<_, _>>()?)
}

#[cfg(test)]
#[path = "watches_tests.rs"]
mod tests;
