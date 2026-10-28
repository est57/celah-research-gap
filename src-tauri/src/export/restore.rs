//! Restoring the collection from a backup file. The file is checked first (integrity,
//! expected tables, schema version), then its pages are copied into the live database
//! with SQLite's backup API, so the app does not need to restart.

use crate::db;
use crate::error::{AppError, AppResult};
use rusqlite::{Connection, OpenFlags, MAIN_DB};
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupInfo {
    pub documents: i64,
    pub reviews: i64,
    /// Schema version of the backup; older ones are migrated after restoring.
    pub schema: i64,
}

fn invalid(msg: &str) -> AppError {
    AppError::Invalid(msg.into())
}

/// Opens `path` read-only and checks it is an intact Celah database this version can use.
pub fn inspect(path: &Path) -> AppResult<BackupInfo> {
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX)
        .map_err(|_| invalid(crate::tr!("file tidak bisa dibuka sebagai database", "the file cannot be opened as a database")))?;
    let check: String = conn
        .query_row("PRAGMA integrity_check", [], |r| r.get(0))
        .map_err(|_| invalid(crate::tr!("file ini bukan database SQLite", "this file is not a SQLite database")))?;
    if check != "ok" {
        return Err(invalid(crate::tr!("file cadangan rusak (pemeriksaan integritas gagal)", "the backup file is damaged (integrity check failed)")));
    }
    let schema: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    if schema > db::SCHEMA_VERSION {
        return Err(invalid(crate::tr!("cadangan ini dibuat oleh versi Celah yang lebih baru; perbarui aplikasi dulu", "this backup was made by a newer version of Celah; update the app first")));
    }
    let has = |table: &str| -> AppResult<bool> {
        Ok(conn.prepare("SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1")?.exists([table])?)
    };
    if schema < 2 || !has("papers")? {
        return Err(invalid(crate::tr!("file ini bukan cadangan Celah", "this file is not a Celah backup")));
    }
    let documents = conn.query_row("SELECT COUNT(*) FROM papers", [], |r| r.get(0))?;
    let reviews = if has("gap_reviews")? { conn.query_row("SELECT COUNT(*) FROM gap_reviews", [], |r| r.get(0))? } else { 0 };
    Ok(BackupInfo { documents, reviews, schema })
}

/// Replaces the live database with `from`, then brings it to the current schema.
/// Callers save a backup of the current data first.
pub fn restore(conn: &mut Connection, from: &Path) -> AppResult<()> {
    inspect(from)?;
    conn.restore(MAIN_DB, from, None::<fn(rusqlite::backup::Progress)>)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    db::migrate(conn)
}
