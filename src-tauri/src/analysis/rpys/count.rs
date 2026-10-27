//! Reads the collection's text references and counts citing documents per cited work.

use super::parse::{self, Parsed};
use crate::error::AppResult;
use rusqlite::Connection;
use std::collections::{HashMap, HashSet};

/// Long enough to keep a DOI written at the end of a reference.
const MAX_EXAMPLE: usize = 400;

pub struct Tally {
    pub year: u16,
    pub label: String,
    pub example: String,
    pub documents: usize,
}

pub struct Counts {
    pub refs: HashMap<String, Tally>,
    pub citing: usize,
    pub dated: usize,
    pub undated: usize,
}

pub fn count(conn: &Connection, newest: u16) -> AppResult<Counts> {
    let mut parsed: Vec<(i64, String, Parsed)> = Vec::new();
    let mut citing = HashSet::new();
    let mut undated = 0;
    let mut stmt = conn.prepare("SELECT paper_id, raw FROM paper_refs")?;
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let (paper, raw): (i64, String) = (r.get(0)?, r.get(1)?);
        citing.insert(paper);
        match parse::reference(&raw, newest) {
            Some(p) => parsed.push((paper, raw, p)),
            None => undated += 1,
        }
    }

    // Surname-year keys that belong to exactly one DOI.
    let mut alias: HashMap<&str, Option<&str>> = HashMap::new();
    for (_, _, p) in &parsed {
        if let Some(a) = &p.alias {
            let slot = alias.entry(a.as_str()).or_insert(Some(p.key.as_str()));
            if *slot != Some(p.key.as_str()) {
                *slot = None;
            }
        }
    }
    let resolve = |p: &Parsed| -> String {
        alias.get(p.key.as_str()).copied().flatten().map_or_else(|| p.key.clone(), str::to_string)
    };

    let mut refs: HashMap<String, Tally> = HashMap::new();
    let mut seen: HashSet<(i64, String)> = HashSet::new();
    let mut dated = 0;
    for (paper, raw, p) in &parsed {
        let key = resolve(p);
        if !seen.insert((*paper, key.clone())) {
            continue;
        }
        dated += 1;
        let t = refs.entry(key).or_insert_with(|| Tally {
            year: p.year,
            label: p.label.clone(),
            example: raw.chars().take(MAX_EXAMPLE).collect(),
            documents: 0,
        });
        t.documents += 1;
    }
    Ok(Counts { refs, citing: citing.len(), dated, undated })
}
