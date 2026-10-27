//! Sources' local impact (h, g, m indices, citations, Bradford zone) and the
//! cumulative production of the top sources over time, as in bibliometrix.

use super::authors::h_g;
use super::{current_year, sources};
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::{BTreeSet, HashMap};

const DYNAMICS_TOP: usize = 6;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceRow {
    pub label: String,
    pub documents: usize,
    pub citations: i64,
    pub h: usize,
    pub g: usize,
    pub m: Option<f64>,
    pub first_year: Option<i32>,
    /// Bradford zone, 1 = core.
    pub zone: u8,
}

#[derive(Serialize)]
pub struct Series {
    pub label: String,
    /// Documents up to and including each year in `years`.
    pub cumulative: Vec<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcesDetail {
    pub current_year: i32,
    pub sources: Vec<SourceRow>,
    pub years: Vec<i32>,
    pub dynamics: Vec<Series>,
}

pub fn compute(conn: &Connection) -> AppResult<SourcesDetail> {
    let now = current_year(conn)?;
    let ranked = sources::ranked_sources(conn)?;
    let zones: HashMap<String, u8> = sources::bradford_all(&ranked).into_iter().collect();
    let mut stmt = conn.prepare(
        "SELECT lower(trim(source_name)), year, cited_by FROM papers WHERE trim(COALESCE(source_name, '')) <> ''",
    )?;
    let mut by_source: HashMap<String, Vec<(Option<i32>, i64)>> = HashMap::new();
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        by_source.entry(r.get(0)?).or_default().push((r.get(1)?, r.get(2)?));
    }
    let mut out: Vec<SourceRow> = ranked
        .iter()
        .filter_map(|s| {
            let papers = by_source.get(&s.label.trim().to_lowercase())?;
            let cites: Vec<i64> = papers.iter().map(|p| p.1).collect();
            let (h, g) = h_g(&cites);
            let first_year = papers.iter().filter_map(|p| p.0).min();
            Some(SourceRow {
                label: s.label.clone(),
                documents: papers.len(),
                citations: cites.iter().sum(),
                h,
                g,
                m: first_year.map(|y| h as f64 / f64::from((now - y + 1).max(1))),
                first_year,
                zone: zones.get(&s.label).copied().unwrap_or(3),
            })
        })
        .collect();
    out.sort_by(|a, b| b.h.cmp(&a.h).then(b.citations.cmp(&a.citations)).then(a.label.cmp(&b.label)));

    let years: Vec<i32> = by_source.values().flatten().filter_map(|p| p.0).filter(|y| *y <= now).collect::<BTreeSet<_>>().into_iter().collect();
    let dynamics = ranked
        .iter()
        .take(DYNAMICS_TOP)
        .map(|s| {
            let papers = by_source.get(&s.label.trim().to_lowercase()).map(Vec::as_slice).unwrap_or(&[]);
            let cumulative = years.iter().map(|y| papers.iter().filter(|p| p.0.is_some_and(|py| py <= *y)).count() as i64).collect();
            Series { label: s.label.clone(), cumulative }
        })
        .collect();
    Ok(SourcesDetail { current_year: now, sources: out, years, dynamics })
}
