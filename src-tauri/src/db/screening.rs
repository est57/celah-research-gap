//! Title and abstract screening: the researcher marks each document Include or Exclude
//! (with a reason). Excluded documents leave every analysis, like the collection filter.

use super::papers::like_pattern;
use crate::error::{AppError, AppResult};
use rusqlite::{params, Connection};
use serde::Serialize;

/// Reason codes for an exclusion; the frontend shows a label for each.
pub const REASONS: [&str; 6] = ["topic", "type", "language", "access", "design", "other"];
pub const MAX_NOTE: usize = 200;

/// Condition on `papers` that keeps every document not excluded at screening.
pub const NOT_EXCLUDED: &str = "papers.id NOT IN (SELECT paper_id FROM main.screening WHERE decision = 'exclude')";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenRow {
    pub id: i64,
    pub title: String,
    pub authors: String,
    pub year: Option<i32>,
    pub source_name: Option<String>,
    pub abstract_text: Option<String>,
    pub keywords: Option<String>,
    /// "include", "exclude", or "" when not screened yet.
    pub decision: String,
    pub reason: String,
    pub note: String,
}

#[derive(Serialize, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScreenCounts {
    pub unscreened: i64,
    pub included: i64,
    pub excluded: i64,
}

pub fn has_exclusions(conn: &Connection) -> AppResult<bool> {
    Ok(conn.query_row("SELECT EXISTS (SELECT 1 FROM main.screening WHERE decision = 'exclude')", [], |r| r.get(0))?)
}

pub fn counts(conn: &Connection) -> AppResult<ScreenCounts> {
    Ok(conn.query_row(
        "SELECT SUM(s.decision IS NULL), SUM(s.decision = 'include'), SUM(s.decision = 'exclude')
         FROM main.papers p LEFT JOIN main.screening s ON s.paper_id = p.id",
        [],
        |r| Ok(ScreenCounts { unscreened: r.get::<_, Option<i64>>(0)?.unwrap_or(0), included: r.get::<_, Option<i64>>(1)?.unwrap_or(0), excluded: r.get::<_, Option<i64>>(2)?.unwrap_or(0) }),
    )?)
}

/// One page of documents in `status` ("all", "none", "include", "exclude"), newest first.
pub fn page(conn: &Connection, status: &str, query: Option<&str>, limit: u32, offset: u32) -> AppResult<(Vec<ScreenRow>, i64)> {
    let status_sql = match status {
        "all" => "1",
        "none" => "s.decision IS NULL",
        "include" => "s.decision = 'include'",
        "exclude" => "s.decision = 'exclude'",
        _ => return Err(AppError::Invalid(crate::tr!("status skrining tidak dikenal", "unknown screening status").into())),
    };
    // `status_sql` is one of the fixed literals above; user text is only bound.
    let from = format!(
        "FROM main.papers p LEFT JOIN main.screening s ON s.paper_id = p.id
         WHERE {status_sql} AND (?1 IS NULL OR p.title LIKE ?1 ESCAPE '\\' OR p.abstract LIKE ?1 ESCAPE '\\' OR p.keywords LIKE ?1 ESCAPE '\\')"
    );
    let pattern = query.map(like_pattern);
    let total: i64 = conn.query_row(&format!("SELECT COUNT(*) {from}"), params![pattern], |r| r.get(0))?;
    let mut stmt = conn.prepare(&format!(
        "SELECT p.id, p.title, p.authors, p.year, p.source_name, substr(p.abstract, 1, 3000), p.keywords,
                COALESCE(s.decision, ''), COALESCE(s.reason, ''), COALESCE(s.note, '')
         {from} ORDER BY p.year DESC, p.id DESC LIMIT ?2 OFFSET ?3"
    ))?;
    let rows = stmt
        .query_map(params![pattern, limit, offset], |r| {
            Ok(ScreenRow {
                id: r.get(0)?,
                title: r.get(1)?,
                authors: r.get(2)?,
                year: r.get(3)?,
                source_name: r.get(4)?,
                abstract_text: r.get(5)?,
                keywords: r.get(6)?,
                decision: r.get(7)?,
                reason: r.get(8)?,
                note: r.get(9)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok((rows, total))
}

/// Sets (or with `decision` None clears) the decision of every id. Unknown ids are ignored.
pub fn set(conn: &mut Connection, ids: &[i64], decision: Option<&str>, reason: &str, note: &str) -> AppResult<usize> {
    let tx = conn.transaction()?;
    let mut changed = 0;
    {
        let mut clear = tx.prepare_cached("DELETE FROM screening WHERE paper_id = ?1")?;
        let mut upsert = tx.prepare_cached(
            "INSERT INTO screening (paper_id, decision, reason, note) SELECT id, ?2, ?3, ?4 FROM papers WHERE id = ?1
             ON CONFLICT(paper_id) DO UPDATE SET decision = excluded.decision, reason = excluded.reason,
                note = excluded.note, decided_at = datetime('now')",
        )?;
        for id in ids {
            changed += match decision {
                None => clear.execute(params![id])?,
                Some(d) => upsert.execute(params![id, d, reason, note])?,
            };
        }
    }
    tx.commit()?;
    Ok(changed)
}

#[cfg(test)]
#[path = "screening_tests.rs"]
mod tests;
