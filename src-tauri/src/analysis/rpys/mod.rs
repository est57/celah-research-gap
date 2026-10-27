//! Reference Publication Year Spectroscopy (RPYS): how often each publication year is
//! cited by the collection. Peaks above the 5-year median point at the historical
//! roots of the field (Marx, Bornmann, Barth and Leydesdorff 2014).

pub(crate) mod count;
pub(crate) mod parse;
mod series;
#[cfg(test)]
mod tests;

use crate::error::{AppError, AppResult};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const MAX_PEAKS: usize = 10;
const REFS_PER_PEAK: usize = 5;

#[derive(Deserialize)]
pub struct RpysParams {
    /// First and last cited year; 0 picks them from the data.
    pub from: u32,
    pub to: u32,
}

#[derive(Serialize)]
pub struct PeakRef {
    /// "Surname Year".
    pub label: String,
    /// The reference as one citing document wrote it.
    pub example: String,
    /// Documents in the collection that cite it.
    pub documents: usize,
}

#[derive(Serialize)]
pub struct Peak {
    pub year: u16,
    pub citations: usize,
    pub deviation: i64,
    pub refs: Vec<PeakRef>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rpys {
    pub documents: usize,
    /// Documents with at least one text reference.
    pub citing: usize,
    /// Distinct (document, reference) pairs with a publication year.
    pub dated: usize,
    pub undated: usize,
    pub from: u16,
    pub to: u16,
    pub years: Vec<u16>,
    pub citations: Vec<usize>,
    pub deviation: Vec<i64>,
    pub peaks: Vec<Peak>,
}

fn bound(v: u32, what: &str) -> AppResult<Option<u16>> {
    match v {
        0 => Ok(None),
        1800..=2100 => Ok(Some(v as u16)),
        _ => Err(AppError::Invalid(format!("{what} harus 1800 sampai 2100, atau 0 untuk otomatis"))),
    }
}

pub fn compute(conn: &Connection, p: &RpysParams) -> AppResult<Rpys> {
    let (from, to) = (bound(p.from, "tahun awal")?, bound(p.to, "tahun akhir")?);
    if let (Some(a), Some(b)) = (from, to) {
        if a > b {
            return Err(AppError::Invalid("tahun awal tidak boleh setelah tahun akhir".into()));
        }
    }
    let documents: i64 = conn.query_row("SELECT COUNT(*) FROM papers", [], |r| r.get(0))?;
    let newest: Option<i64> = conn.query_row("SELECT MAX(year) FROM papers", [], |r| r.get(0))?;
    let newest = newest.map_or(2100, |y| y.clamp(1800, 2100) as u16);

    let count::Counts { refs, citing, dated, undated } = count::count(conn, newest)?;

    let mut per_year: BTreeMap<u16, usize> = BTreeMap::new();
    for t in refs.values() {
        *per_year.entry(t.year).or_default() += t.documents;
    }
    let spread: Vec<(u16, usize)> = per_year.iter().map(|(&y, &c)| (y, c)).collect();
    // The oldest 1% of citations is a long, sparse tail that flattens the chart.
    let from = from.or_else(|| series::quantile_year(&spread, 0.01)).unwrap_or(newest);
    let to = to.unwrap_or(newest).max(from);
    let years: Vec<u16> = (from..=to).collect();
    let citations: Vec<usize> = years.iter().map(|y| per_year.get(y).copied().unwrap_or(0)).collect();
    let deviation = series::deviation(&citations);

    let peaks = series::peaks(&deviation, MAX_PEAKS)
        .into_iter()
        .map(|i| {
            let mut top: Vec<&count::Tally> = refs.values().filter(|t| t.year == years[i]).collect();
            top.sort_by(|a, b| b.documents.cmp(&a.documents).then(a.label.cmp(&b.label)));
            let refs = top.into_iter().take(REFS_PER_PEAK)
                .map(|t| PeakRef { label: t.label.clone(), example: t.example.clone(), documents: t.documents })
                .collect();
            Peak { year: years[i], citations: citations[i], deviation: deviation[i], refs }
        })
        .collect();

    Ok(Rpys { documents: documents as usize, citing, dated, undated, from, to, years, citations, deviation, peaks })
}
