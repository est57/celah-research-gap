//! Every reviewed gap, including those no longer among the current candidates
//! (after a filter change or new imports), with its latest verification.

use crate::analysis::gaps;
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewEntry {
    pub key: String,
    /// "matrix", "emerging", "niche", "structural" or "explicit".
    pub kind: String,
    pub title: String,
    pub status: String,
    pub note: String,
    pub updated_at: String,
    /// Still among the candidates of the current (filtered) analysis.
    pub in_analysis: bool,
    pub score: Option<f64>,
    /// Latest verification: "open", "new" or "answered", with its date.
    pub verdict: Option<String>,
    pub checked_at: Option<String>,
}

/// "matrix:cough-sound:transformer" -> "cough sound x transformer", for reviews stored before titles were.
pub fn title_from_key(key: &str) -> String {
    let parts: Vec<String> = key.split(':').skip(1).map(|p| p.replace('-', " ")).collect();
    parts.join(" x ")
}

/// Reviews ordered by most recent change. `conn` may be the filtered analysis
/// connection: reviews and checks are never filtered, the candidates are.
pub fn list(conn: &Connection) -> AppResult<Vec<ReviewEntry>> {
    let current: HashMap<String, (String, f64)> = gaps::compute(conn)?.gaps.into_iter().map(|g| (g.key, (g.title, g.score))).collect();
    let mut stmt = conn.prepare(
        "SELECT r.gap_key, r.status, r.note, r.updated_at, r.title, c.matching, c.checked_at
         FROM main.gap_reviews r LEFT JOIN main.gap_checks c ON c.gap_key = r.gap_key
         ORDER BY r.updated_at DESC, r.gap_key",
    )?;
    let rows = stmt.query_map([], |r| {
        let key: String = r.get(0)?;
        let stored: String = r.get(4)?;
        let now = current.get(&key);
        let title = now.map(|(t, _)| t.clone()).filter(|_| stored.is_empty()).unwrap_or(stored);
        Ok(ReviewEntry {
            kind: key.split(':').next().unwrap_or("").to_string(),
            title: if title.is_empty() { title_from_key(&key) } else { title },
            status: r.get(1)?,
            note: r.get(2)?,
            updated_at: r.get(3)?,
            in_analysis: now.is_some(),
            score: now.map(|(_, s)| *s),
            verdict: r.get::<_, Option<i64>>(5)?.map(|m| super::checks::verdict(m).to_string()),
            checked_at: r.get(6)?,
            key,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{open_memory, reviews};

    #[test]
    fn lists_reviews_including_gaps_no_longer_found() {
        let conn = open_memory();
        reviews::set(&conn, "matrix:cough-sound:transformer", "accepted", "baca Pahar 2021", "").unwrap();
        reviews::set(&conn, "niche:drug-resistance", "rejected", "", "Tema drug resistance terpisah").unwrap();
        let list = list(&conn).unwrap();
        assert_eq!(list.len(), 2);
        let m = list.iter().find(|e| e.kind == "matrix").unwrap();
        assert_eq!((m.title.as_str(), m.status.as_str(), m.in_analysis), ("cough sound x transformer", "accepted", false));
        assert_eq!(m.note, "baca Pahar 2021");
        assert!(m.verdict.is_none());
        let n = list.iter().find(|e| e.kind == "niche").unwrap();
        assert_eq!(n.title, "Tema drug resistance terpisah");
    }
}
