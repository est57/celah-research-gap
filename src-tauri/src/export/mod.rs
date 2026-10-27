//! Exporting the collection (CSV, BibTeX, RIS) and backing up the database file.

pub mod csv_out;
pub mod image;
pub mod restore;
#[cfg(test)]
mod tests;

use crate::error::{AppError, AppResult};
use crate::report::bibtex;
use crate::report::cite::{self, Work};
use rusqlite::{params, Connection};
use std::path::Path;

/// Ids per lookup, well under SQLite's bound-parameter limit.
const CHUNK: usize = 900;

/// Works for every document `conn` can see, oldest first.
fn works(conn: &Connection) -> AppResult<Vec<Work>> {
    let mut stmt = conn.prepare("SELECT id FROM papers ORDER BY year IS NULL, year, id")?;
    let ids: Vec<i64> = stmt.query_map([], |r| r.get(0))?.collect::<Result<_, _>>()?;
    let mut out = Vec::with_capacity(ids.len());
    for chunk in ids.chunks(CHUNK) {
        let mut found = cite::load(conn, chunk)?;
        out.extend(chunk.iter().filter_map(|id| found.remove(id)));
    }
    Ok(out)
}

/// The export in `format` ("csv", "bibtex" or "ris"), its file extension and the document count.
pub fn build(conn: &Connection, format: &str) -> AppResult<(Vec<u8>, &'static str, usize)> {
    Ok(match format {
        "csv" => {
            let (bytes, n) = csv_out::write(conn)?;
            (bytes, "csv", n)
        }
        "bibtex" | "ris" => {
            let list = works(conn)?;
            let text = if format == "bibtex" { bibtex::bibtex(&list) } else { bibtex::ris(&list) };
            (text.into_bytes(), if format == "bibtex" { "bib" } else { "ris" }, list.len())
        }
        _ => return Err(AppError::Invalid("format ekspor tidak dikenal".into())),
    })
}

/// A consistent copy of the whole database at `target` (VACUUM INTO), written next to it
/// first and then moved into place, so a failed backup never leaves a half-written file.
pub fn backup(conn: &Connection, target: &Path) -> AppResult<()> {
    let tmp = target.with_extension("celah-tmp");
    let _ = std::fs::remove_file(&tmp);
    let tmp_text = tmp.to_str().ok_or_else(|| AppError::Invalid("lokasi file tidak didukung".into()))?;
    conn.execute("VACUUM main INTO ?1", params![tmp_text])?;
    std::fs::rename(&tmp, target).map_err(|_| {
        let _ = std::fs::remove_file(&tmp);
        AppError::Invalid("cadangan tidak bisa disimpan di lokasi itu".into())
    })
}
