//! Testing a researcher's own gap idea: how often two concepts appear in the
//! collection, alone and together, against what independence would predict.

mod docs;
#[cfg(test)]
mod tests;

use super::current_year;
use crate::error::{AppError, AppResult};
use crate::verify::tokens;
pub use docs::DocRef;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

const MAX_ALTERNATIVES: usize = 5;
const RECENT_YEARS: i32 = 3;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IdeaParams {
    /// Comma-separated ways of writing concept A, e.g. "vision transformer, ViT".
    pub a: String,
    pub b: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Idea {
    pub a: Vec<String>,
    pub b: Vec<String>,
    pub documents: usize,
    pub with_a: usize,
    pub with_b: usize,
    pub with_both: usize,
    /// Documents with both if the concepts were unrelated: with_a * with_b / documents.
    pub expected: f64,
    /// "kurang-data", "celah", "jarang" or "banyak".
    pub verdict: &'static str,
    /// Share of each concept's documents from the last complete years, and of the collection.
    pub recent_a: f64,
    pub recent_b: f64,
    pub recent_all: f64,
    pub years: Vec<i32>,
    /// Documents per year in `years`: [concept A, concept B, both].
    pub per_year: [Vec<usize>; 3],
    pub both_docs: Vec<DocRef>,
    pub a_only: Vec<DocRef>,
    pub b_only: Vec<DocRef>,
}

/// "vision transformer, ViT" -> ["vision transformer", "vit"]; 1 to 5 forms of 2 to 80 characters.
pub fn parse(raw: &str) -> AppResult<Vec<String>> {
    let forms: Vec<String> = raw.split(',').map(|f| f.trim().to_lowercase()).filter(|f| !f.is_empty()).collect();
    let ok = |f: &String| (2..=80).contains(&f.chars().count()) && !f.chars().any(char::is_control) && !tokens(f).is_empty();
    if forms.is_empty() || forms.len() > MAX_ALTERNATIVES || !forms.iter().all(ok) {
        return Err(AppError::Invalid(format!("isi tiap konsep dengan 1 sampai {MAX_ALTERNATIVES} sebutan, dipisah koma, masing-masing 2 sampai 80 karakter")));
    }
    Ok(forms)
}

/// "networks" -> "network": a trailing "s" is ignored so plurals match.
fn stem(w: &str) -> String {
    w.strip_suffix('s').filter(|s| s.len() >= 3).unwrap_or(w).to_string()
}

/// Words of a text, lowercased and stemmed, ready for phrase matching.
pub fn stems(text: &str) -> Vec<String> {
    tokens(text).iter().map(|w| stem(w)).collect()
}

/// Some form appears as a phrase: its words consecutive and in order in `text` (stems).
pub fn any_form(text: &[String], forms: &[String]) -> bool {
    forms.iter().any(|f| {
        let words = stems(f);
        !words.is_empty() && text.windows(words.len()).any(|w| w == words.as_slice())
    })
}

fn verdict(a: usize, b: usize, both: usize, expected: f64) -> &'static str {
    if a < 3 || b < 3 {
        "kurang-data"
    } else if both as f64 <= expected * 0.35 && both <= 3 {
        "celah"
    } else if both <= 3 || (both as f64) < expected * 0.7 {
        // A handful of documents is rare even when the expectation is just as small.
        "jarang"
    } else {
        "banyak"
    }
}

pub fn compute(conn: &Connection, p: &IdeaParams) -> AppResult<Idea> {
    let (a, b) = (parse(&p.a)?, parse(&p.b)?);
    let all = docs::load(conn)?;
    let now = current_year(conn)?;
    let flags: Vec<(bool, bool)> = all.iter().map(|d| (any_form(&d.tokens, &a), any_form(&d.tokens, &b))).collect();
    let count = |f: &dyn Fn(&(bool, bool)) -> bool| flags.iter().filter(|x| f(x)).count();
    let (with_a, with_b, with_both) = (count(&|x| x.0), count(&|x| x.1), count(&|x| x.0 && x.1));
    let n = all.len();
    let expected = if n > 0 { with_a as f64 * with_b as f64 / n as f64 } else { 0.0 };
    let recent = |pick: &dyn Fn(&(bool, bool)) -> bool| {
        let dated: Vec<i32> = all.iter().zip(&flags).filter(|(_, f)| pick(f)).filter_map(|(d, _)| d.year).collect();
        let hits = dated.iter().filter(|y| (now - RECENT_YEARS..now).contains(*y)).count();
        if dated.is_empty() { 0.0 } else { hits as f64 / dated.len() as f64 }
    };
    let (years, per_year) = docs::per_year(&all, &flags);
    Ok(Idea {
        verdict: verdict(with_a, with_b, with_both, expected),
        recent_a: recent(&|x| x.0),
        recent_b: recent(&|x| x.1),
        recent_all: recent(&|_| true),
        both_docs: docs::top(&all, &flags, |x| x.0 && x.1, 5),
        a_only: docs::top(&all, &flags, |x| x.0 && !x.1, 3),
        b_only: docs::top(&all, &flags, |x| x.1 && !x.0, 3),
        a,
        b,
        documents: n,
        with_a,
        with_b,
        with_both,
        expected,
        years,
        per_year,
    })
}
