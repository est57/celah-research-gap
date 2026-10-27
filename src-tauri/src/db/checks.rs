//! Stored results of gap verification runs. Only the latest run per gap is kept.

use crate::error::AppResult;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::collections::HashMap;

/// New papers mentioning every term at which a gap counts as possibly answered.
pub const ANSWERED_AT: i64 = 3;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckPaper {
    pub title: String,
    pub year: Option<i32>,
    pub doi: Option<String>,
    pub source: Option<String>,
    pub cited_by: i64,
    /// Title or abstract mentions every term of the gap.
    pub matches: bool,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckSummary {
    pub checked_at: String,
    pub query: String,
    pub found: i64,
    pub matching: i64,
    /// "open", "new" (some new studies to read) or "answered".
    pub verdict: &'static str,
    pub papers: Vec<CheckPaper>,
}

pub fn verdict(matching: i64) -> &'static str {
    match matching {
        0 => "open",
        m if m < ANSWERED_AT => "new",
        _ => "answered",
    }
}

/// Replaces the stored result for `key` in one transaction.
pub fn save(conn: &mut Connection, key: &str, query: &str, papers: &[CheckPaper]) -> AppResult<()> {
    let matching = papers.iter().filter(|p| p.matches).count() as i64;
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM gap_checks WHERE gap_key = ?1", params![key])?;
    tx.execute(
        "INSERT INTO gap_checks (gap_key, query, found, matching) VALUES (?1, ?2, ?3, ?4)",
        params![key, query, papers.len() as i64, matching],
    )?;
    {
        let mut add = tx.prepare_cached(
            "INSERT INTO gap_check_papers (gap_key, position, title, year, doi, source, cited_by, matches)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )?;
        for (i, p) in papers.iter().enumerate() {
            add.execute(params![key, i as i64, p.title, p.year, p.doi, p.source, p.cited_by, p.matches])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn all(conn: &Connection) -> AppResult<HashMap<String, CheckSummary>> {
    let mut out: HashMap<String, CheckSummary> = HashMap::new();
    let mut stmt = conn.prepare("SELECT gap_key, checked_at, query, found, matching FROM gap_checks")?;
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let matching: i64 = r.get(4)?;
        out.insert(r.get(0)?, CheckSummary {
            checked_at: r.get(1)?, query: r.get(2)?, found: r.get(3)?, matching, verdict: verdict(matching), papers: Vec::new(),
        });
    }
    let mut stmt = conn.prepare(
        "SELECT gap_key, title, year, doi, source, cited_by, matches FROM gap_check_papers ORDER BY gap_key, matches DESC, position",
    )?;
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let key: String = r.get(0)?;
        if let Some(summary) = out.get_mut(&key) {
            summary.papers.push(CheckPaper { title: r.get(1)?, year: r.get(2)?, doi: r.get(3)?, source: r.get(4)?, cited_by: r.get(5)?, matches: r.get(6)? });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::open_memory;

    fn paper(title: &str, matches: bool) -> CheckPaper {
        CheckPaper { title: title.into(), year: Some(2025), doi: None, source: None, cited_by: 0, matches }
    }

    #[test]
    fn keeps_only_the_latest_run() {
        let mut conn = open_memory();
        save(&mut conn, "matrix:a:b", "a b", &[paper("Old", true)]).unwrap();
        save(&mut conn, "matrix:a:b", "a b", &[paper("New 1", true), paper("New 2", false), paper("New 3", true), paper("New 4", true)]).unwrap();
        let s = &all(&conn).unwrap()["matrix:a:b"];
        assert_eq!((s.found, s.matching, s.verdict), (4, 3, "answered"));
        assert_eq!(s.papers.len(), 4);
        assert!(s.papers[0].matches, "matching papers are listed first");
        assert_eq!(verdict(0), "open");
        assert_eq!(verdict(1), "new");
    }
}
