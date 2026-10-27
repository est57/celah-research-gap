//! Finds likely duplicates that exact import dedupe misses: the same title in a
//! different year (preprint and published version) and nearly identical titles.

use super::record::DupPaper;
use std::collections::{HashMap, HashSet};

pub const SIMILAR: f64 = 0.85;
const MAX_YEAR_GAP_SAME: i32 = 3;
const MAX_YEAR_GAP_SIMILAR: i32 = 1;
/// Short generic titles ("Editorial", "Correction to ...") repeat across unrelated items.
const MIN_WORDS: usize = 4;

fn words(title: &str) -> Vec<String> {
    let lower: String = title.chars().map(|c| if c.is_alphanumeric() { c } else { ' ' }).flat_map(char::to_lowercase).collect();
    let mut out: Vec<String> = lower.split_whitespace().filter(|w| w.chars().count() >= 2).map(str::to_string).collect();
    out.sort();
    out.dedup();
    out
}

fn jaccard(a: &[String], b: &[String]) -> f64 {
    let set: HashSet<&String> = a.iter().collect();
    let shared = b.iter().filter(|w| set.contains(w)).count();
    let union = a.len() + b.len() - shared;
    if union == 0 { 0.0 } else { shared as f64 / union as f64 }
}

fn year_gap(a: &DupPaper, b: &DupPaper) -> i32 {
    match (a.year, b.year) {
        (Some(x), Some(y)) => (x - y).abs(),
        _ => 0,
    }
}

/// Two different non-preprint DOIs mean two published works, e.g. a conference and a journal version.
fn distinct_works(a: &DupPaper, b: &DupPaper) -> bool {
    matches!((&a.doi, &b.doi), (Some(x), Some(y)) if !x.eq_ignore_ascii_case(y) && !a.is_preprint() && !b.is_preprint())
}

/// Name keys differ in order across sources, so any shared name part of 3+ letters counts.
fn same_first_author(a: &DupPaper, b: &DupPaper) -> bool {
    if a.first_author.is_empty() || b.first_author.is_empty() {
        return true;
    }
    let parts: HashSet<&str> = a.first_author.split(' ').filter(|p| p.len() >= 3).collect();
    b.first_author.split(' ').any(|p| parts.contains(p))
}

/// (index a, index b, similarity, same title) for every candidate pair, a < b.
pub fn pairs(papers: &[DupPaper]) -> Vec<(usize, usize, f64, bool)> {
    let toks: Vec<Vec<String>> = papers.iter().map(|p| words(&p.title)).collect();
    let mut df: HashMap<&str, usize> = HashMap::new();
    for t in toks.iter().flatten() {
        *df.entry(t.as_str()).or_default() += 1;
    }
    // Pairs above the threshold share nearly all words, so they share one of each title's two rarest.
    let mut index: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, t) in toks.iter().enumerate().filter(|(_, t)| t.len() >= MIN_WORDS) {
        let mut rare: Vec<&str> = t.iter().map(String::as_str).collect();
        rare.sort_by_key(|w| (df[w], *w));
        for w in rare.into_iter().take(2) {
            index.entry(w).or_default().push(i);
        }
    }
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for group in index.values() {
        for (x, &i) in group.iter().enumerate() {
            for &j in &group[x + 1..] {
                let (a, b) = (i.min(j), i.max(j));
                if !seen.insert((a, b)) || distinct_works(&papers[a], &papers[b]) {
                    continue;
                }
                let (pa, pb) = (&papers[a], &papers[b]);
                let same = !pa.title_key.is_empty() && pa.title_key == pb.title_key;
                let sim = if same { 1.0 } else { jaccard(&toks[a], &toks[b]) };
                let ok = if same {
                    year_gap(pa, pb) <= MAX_YEAR_GAP_SAME
                } else {
                    sim >= SIMILAR && year_gap(pa, pb) <= MAX_YEAR_GAP_SIMILAR && same_first_author(pa, pb)
                };
                if ok {
                    out.push((a, b, sim, same));
                }
            }
        }
    }
    out
}
