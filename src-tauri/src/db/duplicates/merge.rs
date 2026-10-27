//! Merges a duplicate into the record kept: empty fields of the kept record are
//! filled from the duplicate, then the duplicate is removed and its ids recorded
//! as aliases so a later import does not bring it back.

use super::check_ids;
use crate::error::{AppError, AppResult};
use rusqlite::{params, Connection, OptionalExtension};

fn has_rows(conn: &Connection, table: &str, id: i64) -> AppResult<bool> {
    // `table` is one of the fixed names below, never user input.
    Ok(conn.prepare(&format!("SELECT 1 FROM {table} WHERE paper_id = ?1 LIMIT 1"))?.exists(params![id])?)
}

pub fn merge(conn: &mut Connection, keep: i64, drop: i64) -> AppResult<()> {
    check_ids(keep, drop)?;
    let tx = conn.transaction()?;
    let found: Option<(String, Option<String>)> = tx
        .query_row("SELECT ext_id, doi FROM papers WHERE id = ?1", params![drop], |r| Ok((r.get(0)?, r.get(1)?)))
        .optional()?;
    let keep_exists = tx.prepare("SELECT 1 FROM papers WHERE id = ?1")?.exists(params![keep])?;
    let (Some((ext_id, doi)), true) = (found, keep_exists) else {
        return Err(AppError::Invalid("dokumen tidak ditemukan; muat ulang daftar duplikat".into()));
    };

    // DOIs are unique, so the duplicate's DOI is released before the kept record takes it.
    tx.execute("UPDATE papers SET doi = NULL WHERE id = ?1", params![drop])?;
    tx.execute(
        "UPDATE papers AS k SET
            doi = COALESCE(k.doi, ?3),
            abstract = COALESCE(NULLIF(k.abstract, ''), d.abstract),
            keywords = COALESCE(NULLIF(k.keywords, ''), d.keywords),
            keywords_plus = COALESCE(NULLIF(k.keywords_plus, ''), d.keywords_plus),
            source_name = COALESCE(NULLIF(k.source_name, ''), d.source_name),
            year = COALESCE(k.year, d.year),
            cited_by = MAX(k.cited_by, d.cited_by),
            ref_count = MAX(k.ref_count, d.ref_count),
            has_affiliation = MAX(k.has_affiliation, d.has_affiliation)
         FROM papers AS d WHERE k.id = ?1 AND d.id = ?2",
        params![keep, drop, doi],
    )?;
    if !has_rows(&tx, "paper_refs", keep)? {
        tx.execute("UPDATE paper_refs SET paper_id = ?1 WHERE paper_id = ?2", params![keep, drop])?;
    }
    if !has_rows(&tx, "paper_authors", keep)? {
        tx.execute("UPDATE paper_authors SET paper_id = ?1 WHERE paper_id = ?2", params![keep, drop])?;
        tx.execute(
            "UPDATE papers AS k SET authors = d.authors, author_count = d.author_count
             FROM papers AS d WHERE k.id = ?1 AND d.id = ?2",
            params![keep, drop],
        )?;
    }
    tx.execute(
        "INSERT OR IGNORE INTO paper_countries (paper_id, code, corresponding)
         SELECT ?1, code, corresponding FROM paper_countries WHERE paper_id = ?2",
        params![keep, drop],
    )?;
    tx.execute(
        "INSERT OR IGNORE INTO paper_cites (paper_id, cited) SELECT ?1, cited FROM paper_cites WHERE paper_id = ?2",
        params![keep, drop],
    )?;
    tx.execute(
        "INSERT OR IGNORE INTO paper_affiliations (paper_id, name_key, name, country)
         SELECT ?1, name_key, name, country FROM paper_affiliations WHERE paper_id = ?2",
        params![keep, drop],
    )?;
    tx.execute("UPDATE paper_aliases SET paper_id = ?1 WHERE paper_id = ?2", params![keep, drop])?;
    tx.execute("INSERT OR IGNORE INTO paper_aliases (ext_id, doi, paper_id) VALUES (?1, ?2, ?3)", params![ext_id, doi, keep])?;

    // Child rows are removed explicitly so the result does not depend on the foreign_keys pragma.
    for table in ["paper_refs", "paper_authors", "paper_countries", "paper_cites", "paper_affiliations"] {
        tx.execute(&format!("DELETE FROM {table} WHERE paper_id = ?1"), params![drop])?;
    }
    tx.execute("DELETE FROM dup_dismissed WHERE a = ?1 OR b = ?1", params![drop])?;
    tx.execute("DELETE FROM papers WHERE id = ?1", params![drop])?;
    tx.commit()?;
    Ok(())
}
