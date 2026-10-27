//! The collection as CSV: one row per document with every stored field, readable by
//! Excel and R. Cells that a spreadsheet would run as a formula are neutralized.

use crate::error::{AppError, AppResult};
use rusqlite::Connection;

const HEADER: [&str; 14] = [
    "id", "title", "authors", "year", "source", "doc_type", "doi", "cited_by", "author_keywords", "keywords_plus",
    "abstract", "countries", "institutions", "origin",
];

/// Spreadsheets treat a leading =, +, -, @, tab or carriage return as a formula
/// (CSV injection); a leading apostrophe makes the cell plain text.
pub fn safe_cell(value: &str) -> String {
    match value.chars().next() {
        Some('=' | '+' | '-' | '@' | '\t' | '\r') => format!("'{value}"),
        _ => value.to_string(),
    }
}

/// Every document `conn` can see (the whole collection, or the filtered set on the
/// analysis connection), oldest first. Starts with a UTF-8 byte order mark for Excel.
pub fn write(conn: &Connection) -> AppResult<(Vec<u8>, usize)> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.title,
                (SELECT group_concat(name, '; ') FROM (SELECT name FROM paper_authors a WHERE a.paper_id = p.id ORDER BY a.position)),
                p.year, p.source_name, p.doc_type, p.doi, p.cited_by, p.keywords, p.keywords_plus, p.abstract,
                (SELECT group_concat(code, '; ') FROM paper_countries c WHERE c.paper_id = p.id),
                (SELECT group_concat(name, '; ') FROM paper_affiliations f WHERE f.paper_id = p.id),
                p.origin
         FROM papers p ORDER BY p.year IS NULL, p.year, p.id",
    )?;
    let mut out = Vec::from(&b"\xEF\xBB\xBF"[..]);
    let mut writer = csv::Writer::from_writer(&mut out);
    writer.write_record(HEADER).map_err(|_| AppError::Internal)?;
    let mut rows = stmt.query([])?;
    let mut count = 0;
    while let Some(r) = rows.next()? {
        let text = |i: usize| -> rusqlite::Result<String> {
            Ok(match r.get_ref(i)? {
                rusqlite::types::ValueRef::Null => String::new(),
                rusqlite::types::ValueRef::Integer(n) => n.to_string(),
                rusqlite::types::ValueRef::Real(f) => f.to_string(),
                rusqlite::types::ValueRef::Text(t) | rusqlite::types::ValueRef::Blob(t) => String::from_utf8_lossy(t).into_owned(),
            })
        };
        let record: Vec<String> = (0..HEADER.len()).map(|i| text(i).map(|v| safe_cell(&v))).collect::<Result<_, _>>()?;
        writer.write_record(&record).map_err(|_| AppError::Internal)?;
        count += 1;
    }
    writer.flush().map_err(|_| AppError::Internal)?;
    drop(writer);
    Ok((out, count))
}
