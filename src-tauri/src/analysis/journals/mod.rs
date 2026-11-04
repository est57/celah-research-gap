//! Target journal recommendation: which journals in the (filtered) collection publish work
//! on the gaps the researcher accepted, recently and often.

mod matcher;
mod rank;

use crate::db::sjr::{self, SjrStatus};
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;

/// A gap whose terms are matched against the documents of each journal.
pub struct GapTerms {
    pub title: String,
    pub terms: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalPick {
    pub label: String,
    /// Documents of this journal in the collection.
    pub documents: usize,
    /// Documents that match at least one gap.
    pub matching: usize,
    /// Matching documents from the last three years of the collection.
    pub recent: usize,
    /// Total citations of the matching documents.
    pub citations: i64,
    /// "Q1".."Q4", or "" when the journal is not in the SJR list.
    pub quartile: String,
    pub sjr: Option<f64>,
    /// Titles of the gaps this journal's documents match.
    pub gaps: Vec<String>,
    pub score: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalTargets {
    pub status: SjrStatus,
    /// True when no gap was accepted and every non-rejected candidate was used instead.
    pub fallback: bool,
    /// Titles of the gaps used for matching.
    pub gaps: Vec<String>,
    pub documents: usize,
    /// First year counted as recent; None when no document has a year.
    pub recent_from: Option<i32>,
    pub journals: Vec<JournalPick>,
}

/// Uses the accepted gaps, or every candidate that was not rejected when none is accepted.
pub fn compute(conn: &Connection) -> AppResult<JournalTargets> {
    let list = crate::analysis::gaps::compute(conn)?;
    let to_terms = |g: &crate::analysis::gaps::model::Gap| GapTerms { title: g.title.clone(), terms: g.terms.clone() };
    let accepted: Vec<GapTerms> = list.gaps.iter().filter(|g| g.status == "accepted").map(to_terms).collect();
    let fallback = accepted.is_empty();
    let used = if fallback { list.gaps.iter().filter(|g| g.status != "rejected").map(to_terms).collect() } else { accepted };
    rank_for(conn, &used, fallback)
}

/// Ranks the journals for the given gaps.
pub fn rank_for(conn: &Connection, gaps: &[GapTerms], fallback: bool) -> AppResult<JournalTargets> {
    let status = sjr::status(conn)?;
    let ranked = rank::rank(conn, gaps)?;
    Ok(JournalTargets {
        status,
        fallback,
        gaps: gaps.iter().map(|g| g.title.clone()).collect(),
        documents: ranked.documents,
        recent_from: ranked.recent_from,
        journals: ranked.journals,
    })
}

#[cfg(test)]
mod tests;
