//! Values the filter can choose from, with document counts, over the whole collection.

use super::filter::{self, CollectionFilter};
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;

/// Publication sources listed; the rest are reachable through the search box in the UI.
const MAX_SOURCES: i64 = 300;

#[derive(Serialize)]
pub struct Choice {
    pub value: String,
    pub count: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterOptions {
    pub total: i64,
    pub year_min: Option<i32>,
    pub year_max: Option<i32>,
    pub doc_types: Vec<Choice>,
    pub origins: Vec<Choice>,
    pub sources: Vec<Choice>,
    pub filter: CollectionFilter,
    pub matching: i64,
}

fn choices(conn: &Connection, expr: &str, limit: i64) -> AppResult<Vec<Choice>> {
    // `expr` is one of the fixed column expressions below, never user input.
    let mut stmt = conn.prepare(&format!("SELECT {expr}, COUNT(*) FROM main.papers GROUP BY 1 ORDER BY 2 DESC, 1 LIMIT ?1"))?;
    let rows = stmt.query_map([limit], |r| Ok(Choice { value: r.get(0)?, count: r.get(1)? }))?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn options(conn: &Connection) -> AppResult<FilterOptions> {
    let (total, year_min, year_max) =
        conn.query_row("SELECT COUNT(*), MIN(year), MAX(year) FROM main.papers", [], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
    let current = filter::load(conn)?;
    Ok(FilterOptions {
        total,
        year_min,
        year_max,
        doc_types: choices(conn, "COALESCE(doc_type, '')", 50)?,
        origins: choices(conn, "origin", 50)?,
        sources: choices(conn, "COALESCE(source_name, '')", MAX_SOURCES)?,
        matching: filter::matching(conn, &current)?,
        filter: current,
    })
}
