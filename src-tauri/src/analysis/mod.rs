//! Descriptive bibliometric analysis computed from the local database.

pub mod annual;
pub mod affiliations;
pub mod author_table;
pub mod authors;
pub mod cited;
pub mod cited_entities;
pub mod conceptual;
pub mod countries;
pub mod country_table;
mod cooccur;
pub(crate) mod coupling;
pub mod coupling_map;
pub mod evolution;
pub mod factorial;
pub mod gaps;
pub mod historiograph;
pub mod idea;
mod louvain;
pub mod lotka;
pub mod main_info;
pub mod quartiles;
pub(crate) mod network;
pub(crate) mod refkey;
pub mod rpys;
pub mod source_impact;
pub mod sources;
pub mod top_refs;
pub mod structure;
pub mod sources_authors;
pub(crate) mod thematic;
pub mod three_field;
mod trends;
pub mod words;

use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;

/// Items per column in the three-field plot.
const THREE_FIELD_TOP: usize = 8;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Overview {
    pub current_year: i32,
    pub info: main_info::MainInfo,
    pub annual: Vec<annual::YearPoint>,
    pub three_field: three_field::ThreeField,
}

/// Keyword strings are stored as "a; b; c". Yields (lowercase key, original label).
pub fn terms_with_label(raw: Option<&str>) -> impl Iterator<Item = (String, String)> + '_ {
    raw.unwrap_or("")
        .split(';')
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(|t| (t.to_lowercase(), t.to_string()))
}

pub fn terms(raw: Option<&str>) -> impl Iterator<Item = String> + '_ {
    terms_with_label(raw).map(|(key, _)| key)
}

/// Current calendar year, taken from SQLite so no date crate is needed.
pub fn current_year(conn: &Connection) -> AppResult<i32> {
    Ok(conn.query_row("SELECT CAST(strftime('%Y', 'now') AS INTEGER)", [], |r| r.get(0))?)
}

pub fn overview(conn: &Connection) -> AppResult<Overview> {
    let current_year = current_year(conn)?;
    let annual = annual::compute(conn, current_year)?;
    let info = main_info::compute(conn, current_year, annual::growth_rate(&annual))?;
    let three_field = three_field::compute(conn, THREE_FIELD_TOP)?;
    Ok(Overview { current_year, info, annual, three_field })
}

#[cfg(test)]
mod tests;
