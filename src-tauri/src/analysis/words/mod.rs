//! Word analysis: the most frequent keywords, title or abstract words, and how often the
//! top ones appear per year (bibliometrix Most Frequent Words and Words' Frequency over Time).

mod stop;
pub(crate) mod text;
#[cfg(test)]
mod tests;

use super::conceptual::in_range;
use super::terms_with_label;
use crate::error::{AppError, AppResult};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Words drawn in the per-year chart.
const SERIES: usize = 8;

#[derive(Deserialize)]
pub struct WordParams {
    /// "author_keywords", "keywords_plus", "title" or "abstract".
    pub field: String,
    /// Words per phrase for titles and abstracts (1 to 3); keywords ignore it.
    pub ngram: u32,
    pub top: u32,
}

#[derive(Serialize)]
pub struct WordCount {
    pub label: String,
    /// Documents that contain the word.
    pub documents: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WordSeries {
    pub label: String,
    /// Documents per year in `years`.
    pub per_year: Vec<usize>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Words {
    pub field: &'static str,
    pub ngram: usize,
    pub documents: usize,
    /// Documents with the field filled in.
    pub with_field: usize,
    pub distinct: usize,
    pub words: Vec<WordCount>,
    pub years: Vec<i32>,
    pub series: Vec<WordSeries>,
}

/// (fixed SQL, label, is free text). The SQL never contains caller input.
fn field(raw: &str) -> AppResult<(&'static str, &'static str, bool)> {
    Ok(match raw {
        "author_keywords" => ("SELECT year, keywords FROM papers WHERE COALESCE(keywords, '') <> ''", "Kata kunci penulis", false),
        "keywords_plus" => ("SELECT year, keywords_plus FROM papers WHERE COALESCE(keywords_plus, '') <> ''", "Keywords Plus", false),
        "title" => ("SELECT year, title FROM papers WHERE COALESCE(title, '') <> ''", "Kata judul", true),
        "abstract" => ("SELECT year, abstract FROM papers WHERE COALESCE(abstract, '') <> ''", "Kata abstrak", true),
        _ => return Err(AppError::Invalid("bidang kata tidak dikenal".into())),
    })
}

#[derive(Default)]
struct Tally {
    documents: usize,
    /// Spellings seen for this key; the most common one is shown.
    labels: HashMap<String, usize>,
}

pub fn compute(conn: &Connection, p: &WordParams) -> AppResult<Words> {
    let top = in_range(p.top, 10, 100, "jumlah kata")? as usize;
    let (sql, label, free_text) = field(&p.field)?;
    let ngram = if free_text { in_range(p.ngram, 1, 3, "panjang frasa")? as usize } else { 1 };
    let documents: i64 = conn.query_row("SELECT COUNT(*) FROM papers", [], |r| r.get(0))?;

    // Each document as (year, distinct keys), plus a tally per key.
    let mut docs: Vec<(Option<i32>, Vec<String>)> = Vec::new();
    let mut tally: HashMap<String, Tally> = HashMap::new();
    let mut stmt = conn.prepare(sql)?;
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let (year, raw): (Option<i32>, String) = (r.get(0)?, r.get(1)?);
        let items: Vec<(String, String)> = if free_text {
            text::grams(&raw, ngram).into_iter().map(|g| (text::key(&g), g)).collect()
        } else {
            terms_with_label(Some(&raw)).collect()
        };
        let mut keys: Vec<String> = Vec::new();
        for (key, shown) in items {
            if keys.contains(&key) {
                continue;
            }
            let t = tally.entry(key.clone()).or_default();
            t.documents += 1;
            *t.labels.entry(shown).or_default() += 1;
            keys.push(key);
        }
        docs.push((year, keys));
    }

    let mut ranked: Vec<(&String, &Tally)> = tally.iter().collect();
    ranked.sort_by(|a, b| b.1.documents.cmp(&a.1.documents).then(a.0.cmp(b.0)));
    ranked.truncate(top);
    let shown = |t: &Tally| t.labels.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))).map(|(l, _)| l.clone()).unwrap_or_default();
    let words: Vec<WordCount> = ranked.iter().map(|(_, t)| WordCount { label: shown(t), documents: t.documents }).collect();

    let dated = docs.iter().filter_map(|d| d.0);
    let years: Vec<i32> = match (dated.clone().min(), dated.max()) {
        (Some(lo), Some(hi)) => (lo..=hi).collect(),
        _ => vec![],
    };
    let mut series: Vec<WordSeries> = ranked
        .iter()
        .take(SERIES)
        .map(|(_, t)| WordSeries { label: shown(t), per_year: vec![0; years.len()] })
        .collect();
    let index: HashMap<&String, usize> = ranked.iter().take(SERIES).enumerate().map(|(i, (k, _))| (*k, i)).collect();
    for (year, keys) in &docs {
        let Some(y) = year else { continue };
        let at = (y - years[0]) as usize;
        for k in keys {
            if let Some(&i) = index.get(k) {
                series[i].per_year[at] += 1;
            }
        }
    }

    Ok(Words { field: label, ngram, documents: documents as usize, with_field: docs.len(), distinct: tally.len(), words, years, series })
}
