//! Documents that contribute most to the first two axes of the map.

use super::mca::Mca;
use super::FactorCluster;
use crate::error::AppResult;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::collections::HashMap;

const TOP: usize = 10;
/// Documents with a single keyword sit exactly on it and say little about the map.
const MIN_TERMS: usize = 2;

#[derive(Serialize)]
pub struct FactorDoc {
    pub id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub x: f64,
    pub y: f64,
    pub cluster: usize,
}

/// Rows have equal mass, so a document's contribution grows with its squared distance from the origin.
pub fn top(conn: &Connection, m: &Mca, terms: &HashMap<usize, Vec<usize>>, paper_ids: &[i64], clusters: &[FactorCluster]) -> AppResult<Vec<FactorDoc>> {
    let mut placed: Vec<(usize, (f64, f64))> = terms.iter().filter(|(_, t)| t.len() >= MIN_TERMS).map(|(&d, t)| (d, m.document(t))).collect();
    placed.sort_by(|a, b| (b.1 .0.powi(2) + b.1 .1.powi(2)).total_cmp(&(a.1 .0.powi(2) + a.1 .1.powi(2))).then(a.0.cmp(&b.0)));
    let mut stmt = conn.prepare("SELECT title, year FROM papers WHERE id = ?1")?;
    let mut out = Vec::new();
    for (d, (x, y)) in placed.into_iter().take(TOP) {
        let id = paper_ids[d];
        let (title, year) = stmt.query_row(params![id], |r| Ok((r.get(0)?, r.get(1)?)))?;
        let cluster = clusters
            .iter()
            .min_by(|a, b| ((a.x - x).powi(2) + (a.y - y).powi(2)).total_cmp(&((b.x - x).powi(2) + (b.y - y).powi(2))))
            .map_or(0, |c| c.id);
        out.push(FactorDoc { id, title, year, x, y, cluster });
    }
    Ok(out)
}
