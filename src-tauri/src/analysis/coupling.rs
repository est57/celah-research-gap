//! Bibliographic coupling (Kessler 1963): documents, sources or authors are linked
//! when the documents behind them cite the same works. Works are matched by DOI, by
//! OpenAlex id, or by the surname-year key used for co-citation.

use super::historiograph::refmatch::dois;
use super::refkey::ref_key;
use super::network::{self, Network};
use crate::error::AppResult;
use crate::report::names::surname;
use rusqlite::Connection;
use std::collections::{BTreeMap, HashMap, HashSet};

#[cfg(test)]
#[path = "coupling_tests.rs"]
mod tests;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level {
    Documents,
    Sources,
    Authors,
}

impl Level {
    pub fn label(self) -> &'static str {
        match self {
            Level::Documents => crate::tr!("Dokumen", "Documents"),
            Level::Sources => crate::tr!("Sumber", "Sources"),
            Level::Authors => crate::tr!("Penulis", "Authors"),
        }
    }
}

/// Cited works per citing document, each work once.
fn cited(conn: &Connection) -> AppResult<BTreeMap<String, HashSet<i64>>> {
    let mut by_work: BTreeMap<String, HashSet<i64>> = BTreeMap::new();
    let mut stmt = conn.prepare("SELECT paper_id, raw FROM paper_refs")?;
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let (paper, raw): (i64, String) = (r.get(0)?, r.get(1)?);
        let key = match dois(&raw).first() {
            Some(d) => Some(format!("doi:{d}")),
            None => ref_key(&raw).map(|(k, _)| k),
        };
        if let Some(k) = key {
            by_work.entry(k).or_default().insert(paper);
        }
    }
    let mut stmt = conn.prepare("SELECT paper_id, cited FROM paper_cites")?;
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let (paper, work): (i64, String) = (r.get(0)?, r.get(1)?);
        by_work.entry(work.to_lowercase()).or_default().insert(paper);
    }
    // A work cited by one document couples nothing.
    by_work.retain(|_, papers| papers.len() >= 2);
    Ok(by_work)
}

/// (key, label) items each document stands for at this level.
fn entities(conn: &Connection, level: Level) -> AppResult<HashMap<i64, Vec<(String, String)>>> {
    let mut out: HashMap<i64, Vec<(String, String)>> = HashMap::new();
    match level {
        Level::Documents => {
            let mut stmt = conn.prepare(
                "SELECT p.id, p.title, p.year,
                        COALESCE((SELECT a.name FROM paper_authors a WHERE a.paper_id = p.id ORDER BY a.position LIMIT 1), '')
                 FROM papers p",
            )?;
            let mut rows = stmt.query([])?;
            while let Some(r) = rows.next()? {
                let (id, title, year, first): (i64, String, Option<i32>, String) = (r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?);
                let who = if first.is_empty() { title.split_whitespace().next().unwrap_or("").to_string() } else { surname(&first) };
                let when = year.map_or_else(|| "t.t.".to_string(), |y| y.to_string());
                out.entry(id).or_default().push((format!("d{id}"), format!("{who} {when}")));
            }
        }
        Level::Sources => {
            let mut stmt = conn.prepare(
                "SELECT id, lower(trim(source_name)), trim(source_name) FROM papers WHERE trim(COALESCE(source_name, '')) <> ''",
            )?;
            let mut rows = stmt.query([])?;
            while let Some(r) = rows.next()? {
                out.entry(r.get(0)?).or_default().push((r.get(1)?, r.get(2)?));
            }
        }
        Level::Authors => {
            let mut stmt = conn.prepare("SELECT paper_id, name_key, name FROM paper_authors")?;
            let mut rows = stmt.query([])?;
            while let Some(r) = rows.next()? {
                out.entry(r.get(0)?).or_default().push((r.get(1)?, r.get(2)?));
            }
        }
    }
    Ok(out)
}

/// The coupling network and the number of documents that share a cited work with another.
pub fn compute(conn: &Connection, level: Level, top: usize) -> AppResult<(usize, Network)> {
    let works = cited(conn)?;
    let items = entities(conn, level)?;
    let mut labels: HashMap<String, String> = HashMap::new();
    let mut coupled: HashSet<i64> = HashSet::new();
    let mut groups: Vec<Vec<String>> = Vec::with_capacity(works.len());
    for papers in works.values() {
        let mut keys = Vec::new();
        for p in papers {
            coupled.insert(*p);
            for (k, l) in items.get(p).into_iter().flatten() {
                labels.entry(k.clone()).or_insert_with(|| l.clone());
                keys.push(k.clone());
            }
        }
        groups.push(keys);
    }
    Ok((coupled.len(), network::build(&groups, &labels, top)))
}
