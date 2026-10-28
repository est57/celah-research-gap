//! Saved gap ideas: two concepts as the researcher wrote them, keyed like other gaps.

use crate::analysis::gaps::model::slug;
use crate::analysis::idea::parse;
use crate::error::{AppError, AppResult};
use rusqlite::{params, Connection};

pub struct SavedIdea {
    pub key: String,
    pub a: String,
    pub b: String,
}

/// Stores the idea (forms are validated first) and returns its gap key.
pub fn save(conn: &Connection, a: &str, b: &str) -> AppResult<String> {
    let (fa, fb) = (parse(a)?, parse(b)?);
    let key = format!("idea:{}:{}", slug(&fa[0]), slug(&fb[0]));
    conn.execute(
        "INSERT INTO gap_ideas (gap_key, concept_a, concept_b) VALUES (?1, ?2, ?3)
         ON CONFLICT(gap_key) DO UPDATE SET concept_a = excluded.concept_a, concept_b = excluded.concept_b",
        params![key, fa.join(", "), fb.join(", ")],
    )?;
    Ok(key)
}

pub fn list(conn: &Connection) -> AppResult<Vec<SavedIdea>> {
    let mut stmt = conn.prepare("SELECT gap_key, concept_a, concept_b FROM main.gap_ideas ORDER BY created_at, gap_key")?;
    let rows = stmt.query_map([], |r| Ok(SavedIdea { key: r.get(0)?, a: r.get(1)?, b: r.get(2)? }))?;
    Ok(rows.collect::<Result<_, _>>()?)
}

/// Removes a saved idea; its review, if any, stays in the review history.
pub fn remove(conn: &Connection, key: &str) -> AppResult<()> {
    if conn.execute("DELETE FROM gap_ideas WHERE gap_key = ?1", params![key])? == 0 {
        return Err(AppError::Invalid(crate::tr!("ide itu tidak ditemukan", "that idea was not found").into()));
    }
    Ok(())
}
