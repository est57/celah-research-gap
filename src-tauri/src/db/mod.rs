pub mod analysis_conn;
pub mod author_merge;
pub mod author_suggest;
pub mod checks;
pub mod duplicates;
pub mod enrich;
pub mod filter;
pub mod filter_options;
pub mod ideas;
pub mod fill;
pub mod insert;
pub mod papers;
pub mod quality;
pub mod quick_search;
pub mod remove;
pub mod review_history;
pub mod reviews;
mod schema;
mod schema_late;
pub mod stats;
pub mod sjr;
pub mod synonyms;
pub mod text_refs;
pub mod workflow;

use crate::error::AppResult;
use rusqlite::Connection;
use std::path::Path;

pub fn open(path: &Path) -> AppResult<Connection> {
    let mut conn = Connection::open(path)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    migrate(&mut conn)?;
    Ok(conn)
}

/// Schema version of a fully migrated database.
pub const SCHEMA_VERSION: i64 = schema::MIGRATIONS.len() as i64;

/// Applies pending migrations, each in its own transaction.
pub fn migrate(conn: &mut Connection) -> AppResult<()> {
    let current: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    for (i, sql) in schema::MIGRATIONS.iter().enumerate().skip(current.max(0) as usize) {
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", (i + 1) as i64)?;
        tx.commit()?;
    }
    Ok(())
}

#[cfg(test)]
pub fn open_memory() -> Connection {
    let mut conn = Connection::open_in_memory().expect("in-memory db");
    // Same as the real database, so cascading deletes are exercised by tests.
    conn.pragma_update(None, "foreign_keys", "ON").expect("foreign keys");
    migrate(&mut conn).expect("migrations");
    conn
}
