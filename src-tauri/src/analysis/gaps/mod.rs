//! Gap Explorer: gap candidates from several detectors, each backed by documents.

mod evidence;
mod ctx;
mod explicit;
mod fronts;
mod ideas;
pub mod model;
mod pairs;
mod structural;
mod themes;

use super::cooccur;
use super::current_year;
use crate::db::checks::{self, ANSWERED_AT};
use crate::db::reviews;
use crate::error::AppResult;
pub use ctx::Ctx;
use model::{confidence, score, Draft, Gap, Kind};
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;

const WORDS: usize = 250;
const MAX_GAPS: usize = 30;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GapList {
    pub documents: usize,
    pub field: &'static str,
    pub gaps: Vec<Gap>,
}

/// Weights for Kebaruan (novelty), Bukti (evidence) and Momentum, per kind.
fn weights(kind: Kind) -> [f64; 3] {
    match kind {
        Kind::Explicit => [0.3, 0.45, 0.25],
        Kind::Emerging => [0.35, 0.25, 0.4],
        Kind::Idea => [0.5, 0.3, 0.2],
        Kind::Front => [0.45, 0.3, 0.25],
        _ => [0.5, 0.25, 0.25],
    }
}

pub fn compute(conn: &Connection) -> AppResult<GapList> {
    let field = cooccur::best_field(conn)?;
    let net = cooccur::build(conn, field, WORDS)?;
    let mut stmt = conn.prepare("SELECT id, cited_by FROM papers")?;
    let cited: HashMap<i64, i64> = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<Result<_, _>>()?;
    let ctx = Ctx::new(net, current_year(conn)?, cited);

    let mut drafts: Vec<Draft> = Vec::new();
    if !ctx.net.labels.is_empty() {
        drafts.extend(pairs::detect(&ctx));
        drafts.extend(themes::detect(&ctx));
        drafts.extend(structural::detect(&ctx));
        drafts.extend(explicit::detect(conn, &ctx)?);
    }
    drafts.extend(fronts::detect(conn, &ctx)?);
    drafts.extend(ideas::detect(conn)?);
    drafts.dedup_by(|a, b| a.key == b.key);
    // The collection's most frequent keyword anchors verification searches to its topic,
    // so "feature extraction" does not pull in papers from unrelated fields.
    if let Some(anchor) = ctx.net.labels.first().cloned() {
        for d in &mut drafts {
            let low = anchor.to_lowercase();
            if !d.terms.iter().any(|t| t.to_lowercase().contains(&low) || low.contains(&t.to_lowercase())) {
                d.terms.push(anchor.clone());
            }
        }
    }

    let refs: Vec<_> = drafts.iter().map(|d| d.evidence.clone()).collect();
    let resolved = evidence::resolve_all(conn, &refs)?;
    let reviews = reviews::all(conn)?;
    let mut checks = checks::all(conn)?;
    let mut gaps: Vec<Gap> = drafts
        .into_iter()
        .zip(resolved)
        // Every gap must be backed by at least one document.
        .filter(|(_, ev)| !ev.is_empty())
        .map(|(mut d, ev)| {
            let check = checks.remove(&d.key);
            let mut s = score(&d.parts, &weights(d.kind));
            // Recent papers that already cover every term lower the score, by up to half.
            if let Some(c) = &check {
                let unanswered = 1.0 - (c.matching as f64 / ANSWERED_AT as f64).min(1.0);
                d.parts.push(model::part("Belum dijawab", unanswered));
                s *= 0.5 + 0.5 * unanswered;
            }
            let s = (s * 100.0).round() / 100.0;
            let (status, note) = reviews.get(&d.key).cloned().unwrap_or_default();
            Gap {
                confidence: confidence(s, ev.len()),
                key: d.key, kind: d.kind, title: d.title, statement: d.statement, terms: d.terms, score: s,
                parts: d.parts, signals: d.signals, evidence: ev, questions: d.questions, status, note, check,
            }
        })
        .collect();
    gaps.sort_by(|a, b| b.score.total_cmp(&a.score).then(a.key.cmp(&b.key)));
    // Saved ideas always stay; detected candidates fill the rest of the list.
    let (mine, mut found): (Vec<Gap>, Vec<Gap>) = gaps.into_iter().partition(|g| g.kind == Kind::Idea);
    found.truncate(MAX_GAPS);
    let mut gaps: Vec<Gap> = mine.into_iter().chain(found).collect();
    gaps.sort_by(|a, b| b.score.total_cmp(&a.score).then(a.key.cmp(&b.key)));
    Ok(GapList { documents: ctx.net.years.len(), field: field.label(), gaps })
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod fronts_tests;
