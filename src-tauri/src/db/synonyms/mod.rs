//! Keyword synonyms ("CNN" = "convolutional neural network"): stored as a mapping and
//! applied to keyword strings when the analysis connection builds its tables.

pub mod suggest;
#[cfg(test)]
mod tests;

use crate::error::{AppError, AppResult};
use rusqlite::{params, Connection};
use serde::Serialize;
use std::collections::HashMap;

const MAX_KEYS: usize = 30;
const MAX_LABEL: usize = 120;

/// from key -> target label.
pub type Map = HashMap<String, String>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Synonym {
    pub from_key: String,
    pub from_label: String,
    pub to_key: String,
    pub to_label: String,
}

pub fn key(label: &str) -> String {
    label.trim().to_lowercase()
}

pub fn load(conn: &Connection) -> AppResult<Map> {
    let mut stmt = conn.prepare("SELECT from_key, to_label FROM main.keyword_synonyms")?;
    let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
    Ok(rows.collect::<Result<_, _>>()?)
}

pub fn list(conn: &Connection) -> AppResult<Vec<Synonym>> {
    let mut stmt = conn.prepare("SELECT from_key, from_label, to_key, to_label FROM keyword_synonyms ORDER BY to_label, from_label")?;
    let rows = stmt.query_map([], |r| Ok(Synonym { from_key: r.get(0)?, from_label: r.get(1)?, to_key: r.get(2)?, to_label: r.get(3)? }))?;
    Ok(rows.collect::<Result<_, _>>()?)
}

/// "CNN; cnns; Deep learning" with {cnns -> CNN} -> "CNN; Deep learning": mapped, without repeats.
pub fn rewrite(raw: &str, map: &Map) -> String {
    let mut seen: Vec<String> = Vec::new();
    let mut out: Vec<String> = Vec::new();
    for term in raw.split(';').map(str::trim).filter(|t| !t.is_empty()) {
        let label = map.get(&key(term)).map_or(term, String::as_str);
        if !seen.contains(&key(label)) {
            seen.push(key(label));
            out.push(label.to_string());
        }
    }
    out.join("; ")
}

/// Every label in `labels` counts as `target` from now on. Returns how many were mapped.
pub fn merge(conn: &mut Connection, labels: &[String], target: &str) -> AppResult<usize> {
    let bad = |l: &String| l.trim().is_empty() || l.chars().count() > MAX_LABEL || l.chars().any(char::is_control) || l.contains(';');
    let mut keys: Vec<String> = labels.iter().map(|l| key(l)).collect();
    keys.sort();
    keys.dedup();
    if keys.len() < 2 || keys.len() > MAX_KEYS || keys.len() != labels.len() || labels.iter().any(bad) || !keys.contains(&key(target)) {
        return Err(AppError::Invalid(crate::tr!(format!("pilih 2 sampai {MAX_KEYS} kata kunci berbeda, termasuk kata kunci tujuan"), format!("choose 2 to {MAX_KEYS} different keywords, including the target keyword"))));
    }
    let (to_key, to_label) = (key(target), target.trim().to_string());
    let tx = conn.transaction()?;
    let mut mapped = 0;
    for label in labels.iter().filter(|l| key(l) != to_key) {
        tx.execute(
            "INSERT OR REPLACE INTO keyword_synonyms (from_key, from_label, to_key, to_label) VALUES (?1, ?2, ?3, ?4)",
            params![key(label), label.trim(), to_key, to_label],
        )?;
        tx.execute("UPDATE keyword_synonyms SET to_key = ?1, to_label = ?2 WHERE to_key = ?3", params![to_key, to_label, key(label)])?;
        mapped += 1;
    }
    tx.execute("DELETE FROM keyword_synonyms WHERE from_key = ?1", params![to_key])?;
    tx.commit()?;
    Ok(mapped)
}

pub fn remove(conn: &Connection, from_key: &str) -> AppResult<()> {
    if conn.execute("DELETE FROM keyword_synonyms WHERE from_key = ?1", params![from_key])? == 0 {
        return Err(AppError::Invalid(crate::tr!("sinonim itu tidak ditemukan", "that synonym was not found").into()));
    }
    Ok(())
}
