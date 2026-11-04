//! Figures for the PRISMA 2020 flow diagram, over the whole collection:
//! identified (import log) -> duplicates and other removals -> screened -> excluded at
//! screening -> left out by the analysis filter (eligibility criteria) -> included.

use super::filter;
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    /// Import origin or exclusion reason code; "" for records merged before logging began.
    pub label: String,
    pub n: i64,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Prisma {
    pub sources: Vec<Count>,
    pub identified: i64,
    /// Skipped at import as already stored, plus records merged as duplicates later.
    pub duplicates: i64,
    /// Removed from the collection by the researcher (for example a whole source).
    pub removed_other: i64,
    pub screened: i64,
    pub excluded: i64,
    pub reasons: Vec<Count>,
    /// Included documents not screened yet (they still count as included).
    pub unscreened: i64,
    pub filter_active: bool,
    /// Not excluded at screening but outside the analysis filter.
    pub filtered_out: i64,
    pub included: i64,
}

fn counts(conn: &Connection, sql: &str) -> AppResult<Vec<Count>> {
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map([], |r| Ok(Count { label: r.get(0)?, n: r.get(1)? }))?;
    Ok(rows.collect::<Result<_, _>>()?)
}

pub fn compute(conn: &Connection) -> AppResult<Prisma> {
    let sources = counts(conn, "SELECT origin, SUM(fetched) FROM main.import_log GROUP BY origin HAVING SUM(fetched) > 0 ORDER BY origin = '', SUM(fetched) DESC, origin")?;
    let identified: i64 = sources.iter().map(|c| c.n).sum();
    let logged: i64 = conn.query_row("SELECT COALESCE(SUM(duplicates), 0) FROM main.import_log", [], |r| r.get(0))?;
    let merged: i64 = conn.query_row("SELECT COUNT(*) FROM main.paper_aliases", [], |r| r.get(0))?;
    let screened: i64 = conn.query_row("SELECT COUNT(*) FROM main.papers", [], |r| r.get(0))?;
    let duplicates = logged + merged;
    let reasons = counts(conn, "SELECT reason, COUNT(*) FROM main.screening WHERE decision = 'exclude' GROUP BY reason ORDER BY COUNT(*) DESC, reason")?;
    let excluded: i64 = reasons.iter().map(|c| c.n).sum();
    let f = filter::load(conn)?;
    let included = filter::matching(conn, &f)?;
    let (cond, values) = f.condition();
    let unscreened: i64 = conn.query_row(
        &format!("SELECT COUNT(*) FROM main.papers WHERE {cond} AND papers.id NOT IN (SELECT paper_id FROM main.screening)"),
        rusqlite::params_from_iter(values),
        |r| r.get(0),
    )?;
    Ok(Prisma {
        sources,
        identified,
        duplicates,
        removed_other: (identified - duplicates - screened).max(0),
        screened,
        excluded,
        reasons,
        unscreened,
        filter_active: f.is_active(),
        filtered_out: screened - excluded - included,
        included,
    })
}

#[cfg(test)]
#[path = "prisma_tests.rs"]
mod tests;
