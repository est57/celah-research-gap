//! Scores and ranks the journals of the collection for a set of gaps.
//!
//! score = (matching + recent) x quartile bonus
//! - matching: documents of the journal that match at least one gap;
//! - recent: those of them from the last three years of the collection, so a recent
//!   matching document counts twice;
//! - quartile bonus: 1.2 for Q1, 1.1 for Q2, 1.0 otherwise (or without an SJR list).
//!
//! Journals without a matching document are left out.

use super::{matcher, GapTerms, JournalPick};
use crate::db::sjr::{QUARTILE_SQL, SJR_SQL};
use crate::error::AppResult;
use rusqlite::Connection;
use std::collections::{BTreeSet, HashMap};

const TOP: usize = 30;
/// The newest year of the collection and the two before it.
const RECENT_YEARS: i32 = 3;

pub struct Ranked {
    pub documents: usize,
    pub recent_from: Option<i32>,
    pub journals: Vec<JournalPick>,
}

struct Doc {
    key: String,
    label: String,
    year: Option<i32>,
    cited: i64,
    quartile: String,
    sjr: Option<f64>,
}

pub fn bonus(quartile: &str) -> f64 {
    match quartile {
        "Q1" => 1.2,
        "Q2" => 1.1,
        _ => 1.0,
    }
}

pub fn rank(conn: &Connection, gaps: &[GapTerms]) -> AppResult<Ranked> {
    // Only constant SQL fragments are formatted in; no value reaches the query text.
    let sql = format!(
        "SELECT lower(trim(COALESCE(source_name, ''))), trim(COALESCE(source_name, '')), year, cited_by,
                lower(COALESCE(title, '') || ' | ' || COALESCE(abstract, '') || ' | ' || COALESCE(keywords, '') || ' | ' || COALESCE(keywords_plus, '')),
                {QUARTILE_SQL}, {SJR_SQL}
         FROM papers"
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;
    let (mut docs, mut texts) = (Vec::new(), Vec::new());
    while let Some(r) = rows.next()? {
        docs.push(Doc { key: r.get(0)?, label: r.get(1)?, year: r.get(2)?, cited: r.get(3)?, quartile: r.get(5)?, sjr: r.get(6)? });
        texts.push(r.get::<_, String>(4)?);
    }
    let recent_from = docs.iter().filter_map(|d| d.year).max().map(|y| y - (RECENT_YEARS - 1));
    let matched = matcher::matches(&texts, gaps);

    let mut by_source: HashMap<String, (JournalPick, BTreeSet<usize>)> = HashMap::new();
    for (d, hits) in docs.iter().zip(&matched) {
        if d.key.is_empty() {
            continue;
        }
        let (j, set) = by_source.entry(d.key.clone()).or_insert_with(|| {
            let pick = JournalPick { label: d.label.clone(), documents: 0, matching: 0, recent: 0, citations: 0, quartile: d.quartile.clone(), sjr: d.sjr, gaps: Vec::new(), score: 0.0 };
            (pick, BTreeSet::new())
        });
        j.documents += 1;
        if j.quartile.is_empty() && !d.quartile.is_empty() {
            (j.quartile, j.sjr) = (d.quartile.clone(), d.sjr);
        }
        if hits.is_empty() {
            continue;
        }
        j.matching += 1;
        j.citations += d.cited;
        if matches!((d.year, recent_from), (Some(y), Some(from)) if y >= from) {
            j.recent += 1;
        }
        set.extend(hits.iter().copied());
    }

    let mut journals: Vec<JournalPick> = by_source
        .into_values()
        .filter(|(j, _)| j.matching > 0)
        .map(|(mut j, set)| {
            j.gaps = set.into_iter().map(|g| gaps[g].title.clone()).collect();
            j.score = (((j.matching + j.recent) as f64 * bonus(&j.quartile)) * 100.0).round() / 100.0;
            j
        })
        .collect();
    journals.sort_by(|a, b| {
        b.score.total_cmp(&a.score).then(b.recent.cmp(&a.recent)).then(b.citations.cmp(&a.citations)).then(a.label.cmp(&b.label))
    });
    journals.truncate(TOP);
    Ok(Ranked { documents: docs.len(), recent_from, journals })
}
