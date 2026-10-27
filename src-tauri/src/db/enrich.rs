//! Bookkeeping for completing cited works and institutions from OpenAlex: which
//! documents still need a lookup, and storing what came back.

use crate::error::AppResult;
use crate::openalex::refs::{filter_safe_doi, short_id, RefRecord};
use rusqlite::{params, Connection};
use std::collections::HashMap;

const PENDING: &str = "FROM papers p WHERE NOT EXISTS (SELECT 1 FROM refs_checked r WHERE r.paper_id = p.id)
     AND (NOT EXISTS (SELECT 1 FROM paper_cites c WHERE c.paper_id = p.id)
          OR NOT EXISTS (SELECT 1 FROM paper_affiliations f WHERE f.paper_id = p.id))
     AND (p.ext_id LIKE 'openalex:%' OR p.doi IS NOT NULL)";

#[derive(Default)]
pub struct Pending {
    /// OpenAlex id ("W123") -> paper id.
    pub by_id: HashMap<String, i64>,
    /// DOI -> paper id, for documents from other sources.
    pub by_doi: HashMap<String, i64>,
}

pub fn remaining(conn: &Connection) -> AppResult<i64> {
    Ok(conn.query_row(&format!("SELECT COUNT(*) {PENDING}"), [], |r| r.get(0))?)
}

/// Up to `limit` documents without cited works that were never looked up.
pub fn pending(conn: &Connection, limit: i64) -> AppResult<Pending> {
    let mut stmt = conn.prepare(&format!("SELECT p.id, p.ext_id, p.doi {PENDING} ORDER BY p.id LIMIT ?1"))?;
    let rows = stmt.query_map(params![limit], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?, r.get::<_, Option<String>>(2)?)))?;
    let mut out = Pending::default();
    let mut unusable = Vec::new();
    for row in rows {
        let (id, ext, doi) = row?;
        if let Some(w) = ext.strip_prefix("openalex:").and_then(short_id) {
            out.by_id.insert(w, id);
        } else if let Some(d) = doi.filter(|d| filter_safe_doi(d)) {
            out.by_doi.insert(d, id);
        } else {
            unusable.push(id);
        }
    }
    mark_checked(conn, &unusable)?;
    Ok(out)
}

fn mark_checked(conn: &Connection, ids: &[i64]) -> AppResult<()> {
    let mut stmt = conn.prepare_cached("INSERT OR IGNORE INTO refs_checked (paper_id) VALUES (?1)")?;
    for id in ids {
        stmt.execute(params![id])?;
    }
    Ok(())
}

/// Stores cited works for the requested documents and marks them checked. A document
/// found by DOI also gets its OpenAlex id as an alias, so citations to that id match it
/// and the same OpenAlex record is not imported again. Returns documents that gained references.
pub fn save(conn: &mut Connection, requested: &Pending, records: &[RefRecord]) -> AppResult<usize> {
    let tx = conn.transaction()?;
    let mut gained = 0;
    {
        let mut cite = tx.prepare_cached("INSERT OR IGNORE INTO paper_cites (paper_id, cited) VALUES (?1, ?2)")?;
        let mut affil = tx.prepare_cached(
            "INSERT OR IGNORE INTO paper_affiliations (paper_id, name_key, name, country) VALUES (?1, ?2, ?3, ?4)",
        )?;
        let mut alias = tx.prepare_cached(
            "INSERT OR IGNORE INTO paper_aliases (ext_id, doi, paper_id)
             SELECT ?1, ?2, ?3 WHERE NOT EXISTS (SELECT 1 FROM papers WHERE ext_id = ?1)",
        )?;
        for rec in records {
            let via_doi = rec.doi.as_ref().and_then(|d| requested.by_doi.get(d));
            let Some(&paper) = requested.by_id.get(&rec.id).or(via_doi) else { continue };
            if via_doi.is_some() && !requested.by_id.contains_key(&rec.id) {
                alias.execute(params![format!("openalex:{}", rec.id), rec.doi, paper])?;
            }
            for c in &rec.cites {
                cite.execute(params![paper, c])?;
            }
            for a in &rec.affiliations {
                affil.execute(params![paper, a.key, a.name, a.country])?;
            }
            gained += usize::from(!rec.cites.is_empty());
        }
    }
    let ids: Vec<i64> = requested.by_id.values().chain(requested.by_doi.values()).copied().collect();
    mark_checked(&tx, &ids)?;
    tx.commit()?;
    Ok(gained)
}

#[cfg(test)]
#[path = "enrich_tests.rs"]
mod tests;
