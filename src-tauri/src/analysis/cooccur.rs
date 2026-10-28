//! Keyword co-occurrence network over the most frequent keywords.

use super::terms_with_label;
use crate::error::AppResult;
use rusqlite::Connection;
use std::collections::HashMap;

/// Keywords considered per document, to keep pair counts bounded.
const MAX_TERMS_PER_DOC: usize = 60;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Field {
    AuthorKeywords,
    KeywordsPlus,
}

impl Field {
    pub fn column(self) -> &'static str {
        match self {
            Field::AuthorKeywords => "keywords",
            Field::KeywordsPlus => "keywords_plus",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Field::AuthorKeywords => crate::tr!("Kata kunci penulis", "Author keywords"),
            Field::KeywordsPlus => "Keywords Plus",
        }
    }
}

pub struct Net {
    pub labels: Vec<String>,
    /// Normalized key of each keyword, parallel to `labels`; stable across periods.
    pub keys: Vec<String>,
    pub freq: Vec<i64>,
    /// Documents (indices into `years`) that contain each keyword.
    pub docs: Vec<Vec<usize>>,
    pub years: Vec<Option<i32>>,
    /// Database id of each document, parallel to `years`.
    pub paper_ids: Vec<i64>,
    /// Co-occurrence counts for keyword pairs (i < j).
    pub pairs: HashMap<(usize, usize), i64>,
}

/// The field with more filled-in documents, preferring author keywords on a tie.
pub fn best_field(conn: &Connection) -> AppResult<Field> {
    let (de, id): (i64, i64) = conn.query_row(
        "SELECT COUNT(NULLIF(keywords, '')), COUNT(NULLIF(keywords_plus, '')) FROM papers",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    Ok(if de >= id { Field::AuthorKeywords } else { Field::KeywordsPlus })
}

pub fn build(conn: &Connection, field: Field, top: usize) -> AppResult<Net> {
    build_range(conn, field, top, None)
}

/// As `build`, limited to documents published within `years` (inclusive) when given.
pub fn build_range(conn: &Connection, field: Field, top: usize, years: Option<(i32, i32)>) -> AppResult<Net> {
    // The column name comes from the Field enum, never from user input; years are bound parameters.
    let (from, to) = years.unwrap_or((i32::MIN, i32::MAX));
    let filter = if years.is_some() { "WHERE year BETWEEN ?1 AND ?2" } else { "WHERE ?1 <= ?2" };
    let mut stmt = conn.prepare(&format!("SELECT year, {}, id FROM papers {filter}", field.column()))?;
    let mut years = Vec::new();
    let mut paper_ids = Vec::new();
    let mut doc_terms: Vec<Vec<String>> = Vec::new();
    let mut count: HashMap<String, i64> = HashMap::new();
    let mut label: HashMap<String, String> = HashMap::new();
    let mut rows = stmt.query(rusqlite::params![from, to])?;
    while let Some(r) = rows.next()? {
        years.push(r.get::<_, Option<i32>>(0)?);
        paper_ids.push(r.get::<_, i64>(2)?);
        let mut keys: Vec<String> = Vec::new();
        for (key, original) in terms_with_label(r.get::<_, Option<String>>(1)?.as_deref()) {
            if !keys.contains(&key) && keys.len() < MAX_TERMS_PER_DOC {
                *count.entry(key.clone()).or_default() += 1;
                label.entry(key.clone()).or_insert(original);
                keys.push(key);
            }
        }
        doc_terms.push(keys);
    }

    let mut ranked: Vec<(&String, &i64)> = count.iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    ranked.truncate(top);
    let index: HashMap<&str, usize> = ranked.iter().enumerate().map(|(i, (k, _))| (k.as_str(), i)).collect();

    let mut docs = vec![Vec::new(); ranked.len()];
    let mut pairs = HashMap::new();
    for (d, keys) in doc_terms.iter().enumerate() {
        let mut ids: Vec<usize> = keys.iter().filter_map(|k| index.get(k.as_str()).copied()).collect();
        ids.sort_unstable();
        for (a, &i) in ids.iter().enumerate() {
            docs[i].push(d);
            for &j in &ids[a + 1..] {
                *pairs.entry((i, j)).or_insert(0) += 1;
            }
        }
    }
    Ok(Net {
        labels: ranked.iter().map(|(k, _)| label[*k].clone()).collect(),
        keys: ranked.iter().map(|(k, _)| (*k).clone()).collect(),
        freq: ranked.iter().map(|(_, c)| **c).collect(),
        docs,
        years,
        paper_ids,
        pairs,
    })
}

impl Net {
    /// Callon's equivalence index e_ij = c_ij^2 / (c_i * c_j), in 0..1.
    pub fn equivalence(&self) -> Vec<(usize, usize, f64)> {
        self.pairs
            .iter()
            .map(|(&(i, j), &c)| (i, j, (c * c) as f64 / (self.freq[i] * self.freq[j]) as f64))
            .collect()
    }
}
