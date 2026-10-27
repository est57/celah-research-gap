use crate::error::{AppError, AppResult};
use rusqlite::{params, Connection};
use std::collections::HashMap;

pub const STATUSES: [&str; 3] = ["accepted", "check", "rejected"];
const MAX_KEY: usize = 300;
const MAX_NOTE: usize = 2_000;
const MAX_TITLE: usize = 300;

/// gap key -> (status, note)
pub fn all(conn: &Connection) -> AppResult<HashMap<String, (String, String)>> {
    let mut stmt = conn.prepare("SELECT gap_key, status, note FROM gap_reviews")?;
    let rows = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, (r.get(1)?, r.get(2)?))))?
        .collect::<Result<HashMap<_, _>, _>>()?;
    Ok(rows)
}

/// Saves a decision with the gap's current title. An empty status removes the review.
pub fn set(conn: &Connection, key: &str, status: &str, note: &str, title: &str) -> AppResult<()> {
    // Keys are slugs of keywords, which may hold non-ASCII letters ("tuberculose-pédiatrique").
    let key_ok = !key.is_empty()
        && key.len() <= MAX_KEY
        && key.chars().all(|c| c.is_alphanumeric() || matches!(c, '-' | ':' | '_'));
    if !key_ok {
        return Err(AppError::Invalid("kunci celah tidak sah".into()));
    }
    if note.chars().count() > MAX_NOTE || note.chars().any(|c| c.is_control() && c != '\n') {
        return Err(AppError::Invalid("catatan terlalu panjang atau berisi karakter tidak sah".into()));
    }
    if title.chars().count() > MAX_TITLE || title.chars().any(char::is_control) {
        return Err(AppError::Invalid("judul celah terlalu panjang atau berisi karakter tidak sah".into()));
    }
    if status.is_empty() {
        conn.execute("DELETE FROM gap_reviews WHERE gap_key = ?1", params![key])?;
        return Ok(());
    }
    if !STATUSES.contains(&status) {
        return Err(AppError::Invalid("status review tidak dikenal".into()));
    }
    conn.execute(
        "INSERT INTO gap_reviews (gap_key, status, note, title) VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(gap_key) DO UPDATE SET status = excluded.status, note = excluded.note, updated_at = datetime('now'),
             title = CASE WHEN excluded.title <> '' THEN excluded.title ELSE gap_reviews.title END",
        params![key, status, note, title],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::open_memory;

    #[test]
    fn stores_updates_and_validates() {
        let conn = open_memory();
        super::set(&conn, "matrix:cough-sound:transformer", "accepted", "cek data lokal", "Judul").unwrap();
        super::set(&conn, "matrix:cough-sound:transformer", "check", "", "").unwrap();
        super::set(&conn, "explicit:tuberculose-pédiatrique", "check", "", "").unwrap();
        let all = super::all(&conn).unwrap();
        assert_eq!(all["matrix:cough-sound:transformer"].0, "check");
        let title: String = conn.query_row("SELECT title FROM gap_reviews WHERE gap_key = 'matrix:cough-sound:transformer'", [], |r| r.get(0)).unwrap();
        assert_eq!(title, "Judul", "an empty title keeps the stored one");
        assert!(super::set(&conn, "x'; DROP TABLE papers;--", "accepted", "", "").is_err());
        assert!(super::set(&conn, "matrix:a", "approved", "", "").is_err());
        assert!(super::set(&conn, "matrix:a", "accepted", "", "a\u{0}b").is_err());
        super::set(&conn, "matrix:cough-sound:transformer", "", "", "").unwrap();
        super::set(&conn, "explicit:tuberculose-pédiatrique", "", "", "").unwrap();
        assert!(super::all(&conn).unwrap().is_empty());
    }
}
