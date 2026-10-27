//! Merging author name variants ("Rahman M", "Muhammad Rahman") into one author, and
//! undoing it. Only the counting key changes; names stay as printed in each byline.

use super::insert::name_key;
use crate::error::{AppError, AppResult};
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;

const MAX_KEYS: usize = 20;
const MAX_KEY: usize = 200;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Merge {
    pub from_key: String,
    pub from_name: String,
    pub to_key: String,
    pub to_name: String,
}

fn label(conn: &Connection, key: &str) -> AppResult<Option<String>> {
    Ok(conn.query_row("SELECT MIN(name) FROM paper_authors WHERE name_key = ?1", params![key], |r| r.get(0))?)
}

/// Counts every key in `keys` as `target` from now on. Returns the author rows changed.
pub fn merge(conn: &mut Connection, keys: &[String], target: &str) -> AppResult<usize> {
    let mut unique = keys.to_vec();
    unique.sort();
    unique.dedup();
    let bad = |k: &String| k.is_empty() || k.chars().count() > MAX_KEY || k.chars().any(char::is_control);
    if unique.len() < 2 || unique.len() > MAX_KEYS || unique.len() != keys.len() || keys.iter().any(bad) || !unique.iter().any(|k| k == target) {
        return Err(AppError::Invalid(format!("pilih 2 sampai {MAX_KEYS} nama berbeda, termasuk nama tujuan")));
    }
    let tx = conn.transaction()?;
    let Some(to_name) = label(&tx, target)? else { return Err(AppError::Invalid("nama tujuan tidak ditemukan".into())) };
    let mut changed = 0;
    for key in unique.iter().filter(|k| *k != target) {
        let Some(from_name) = label(&tx, key)? else { return Err(AppError::Invalid("salah satu nama tidak ditemukan".into())) };
        tx.execute(
            "INSERT OR REPLACE INTO author_aliases (from_key, from_name, to_key, to_name) VALUES (?1, ?2, ?3, ?4)",
            params![key, from_name, target, to_name],
        )?;
        // Earlier merges into `key` now point straight at the target.
        tx.execute("UPDATE author_aliases SET to_key = ?1, to_name = ?2 WHERE to_key = ?3", params![target, to_name, key])?;
        changed += tx.execute("UPDATE paper_authors SET name_key = ?1 WHERE name_key = ?2", params![target, key])?;
    }
    tx.execute("DELETE FROM author_aliases WHERE from_key = to_key", [])?;
    tx.commit()?;
    Ok(changed)
}

/// Undoes the merge of `from_key`: author rows printed with that name count separately again.
pub fn split(conn: &mut Connection, from_key: &str) -> AppResult<usize> {
    let tx = conn.transaction()?;
    let target: Option<String> = tx.query_row("SELECT to_key FROM author_aliases WHERE from_key = ?1", params![from_key], |r| r.get(0)).optional()?;
    let Some(target) = target else { return Err(AppError::Invalid("penyatuan itu tidak ditemukan".into())) };
    tx.execute("DELETE FROM author_aliases WHERE from_key = ?1", params![from_key])?;
    let rows: Vec<(i64, i64, String)> = tx
        .prepare("SELECT paper_id, position, name FROM paper_authors WHERE name_key = ?1")?
        .query_map(params![target], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<Result<_, _>>()?;
    let mut changed = 0;
    for (paper, position, _) in rows.iter().filter(|(_, _, n)| name_key(n) == from_key) {
        changed += tx.execute("UPDATE paper_authors SET name_key = ?1 WHERE paper_id = ?2 AND position = ?3", params![from_key, paper, position])?;
    }
    tx.commit()?;
    Ok(changed)
}

pub fn list(conn: &Connection) -> AppResult<Vec<Merge>> {
    let mut stmt = conn.prepare("SELECT from_key, from_name, to_key, to_name FROM author_aliases ORDER BY to_name, from_name")?;
    let rows = stmt.query_map([], |r| Ok(Merge { from_key: r.get(0)?, from_name: r.get(1)?, to_key: r.get(2)?, to_name: r.get(3)? }))?;
    Ok(rows.collect::<Result<_, _>>()?)
}

#[cfg(test)]
#[path = "author_merge_tests.rs"]
mod tests;
