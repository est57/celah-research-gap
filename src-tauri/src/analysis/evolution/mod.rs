//! Thematic evolution: a thematic map per period and the flow of themes between periods.

mod links;
mod periods;
#[cfg(test)]
mod tests;

use super::conceptual::{field_choice, in_range};
use super::cooccur;
use super::current_year;
use super::thematic::{self, Theme};
use crate::error::AppResult;
pub use links::ThemeLink;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

/// Largest themes kept per period; smaller ones are fragments of a few keywords.
const MAX_THEMES: usize = 10;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvolutionParams {
    /// "auto", "author" or "plus".
    pub field: String,
    /// Keywords per period.
    pub words: u32,
    /// Minimum theme frequency per 1,000 documents of the period.
    pub min_cluster_freq: u32,
    pub labels: u32,
    /// Cut years; empty for an automatic split.
    pub cuts: Vec<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub from: i32,
    pub to: i32,
    pub documents: usize,
    pub themes: Vec<Theme>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Evolution {
    pub field: &'static str,
    /// Cut years actually used (the automatic ones when none were given).
    pub cuts: Vec<i32>,
    pub periods: Vec<Period>,
    pub links: Vec<ThemeLink>,
}

pub fn compute(conn: &Connection, p: &EvolutionParams) -> AppResult<Evolution> {
    let words = in_range(p.words, 20, 1000, "jumlah kata")? as usize;
    let min_freq = in_range(p.min_cluster_freq, 0, 100, "frekuensi minimum tema")?;
    let labels = in_range(p.labels, 1, 5, "label per tema")? as usize;
    periods::validate(&p.cuts)?;
    let field = field_choice(conn, &p.field)?;
    let years = periods::years(conn)?;
    let cuts = if p.cuts.is_empty() { periods::auto_cuts(&years) } else { p.cuts.clone() };
    let year = current_year(conn)?;

    let mut nets = Vec::new();
    let mut out = Vec::new();
    for (from, to) in periods::ranges(&years, &cuts) {
        let net = cooccur::build_range(conn, field, words, Some((from, to)))?;
        let mut themes = thematic::compute(&net, min_freq, labels, year).themes;
        // Themes come sorted by frequency with ids 0.., so truncating keeps ids aligned.
        themes.truncate(MAX_THEMES);
        out.push(Period { from, to, documents: net.years.len(), themes });
        nets.push(net);
    }
    let links = (1..out.len())
        .flat_map(|i| links::between(i - 1, (&nets[i - 1], &out[i - 1].themes), (&nets[i], &out[i].themes)))
        .collect();
    let used = out.iter().take(out.len().saturating_sub(1)).map(|p| p.to).collect();
    Ok(Evolution { field: field.label(), cuts: used, periods: out, links })
}
