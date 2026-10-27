//! Bookkeeping for completing reference lists from Crossref: documents with a DOI and no
//! reference list that were never looked up, and storing what came back.

use crate::error::AppResult;
use crate::openalex::refs::filter_safe_doi;
use rusqlite::{params, Connection};

const PENDING: &str = "FROM papers p WHERE p.doi IS NOT NULL
     AND NOT EXISTS (SELECT 1 FROM paper_refs r WHERE r.paper_id = p.id)
     AND NOT EXISTS (SELECT 1 FROM crossref_checked c WHERE c.paper_id = p.id)";

pub fn remaining(conn: &Connection) -> AppResult<i64> {
    Ok(conn.query_row(&format!("SELECT COUNT(*) {PENDING}"), [], |r| r.get(0))?)
}

/// Up to `limit` (paper id, DOI) pairs still to look up. DOIs that cannot go into a
/// Crossref filter are marked as checked so they are not offered again.
pub fn pending(conn: &Connection, limit: i64) -> AppResult<Vec<(i64, String)>> {
    let mut stmt = conn.prepare(&format!("SELECT p.id, lower(p.doi) {PENDING} ORDER BY p.id LIMIT ?1"))?;
    let rows: Vec<(i64, String)> = stmt.query_map(params![limit], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<Result<_, _>>()?;
    let (ok, bad): (Vec<_>, Vec<_>) = rows.into_iter().partition(|(_, d)| filter_safe_doi(d));
    for (id, _) in bad {
        mark_checked(conn, id)?;
    }
    Ok(ok)
}

pub fn mark_checked(conn: &Connection, id: i64) -> AppResult<()> {
    conn.execute("INSERT OR IGNORE INTO crossref_checked (paper_id) VALUES (?1)", params![id])?;
    Ok(())
}

/// Stores `refs` for a document that still has none. Returns true when references were added.
pub fn store(conn: &mut Connection, id: i64, refs: &[String]) -> AppResult<bool> {
    let tx = conn.transaction()?;
    let has: bool = tx.query_row("SELECT EXISTS (SELECT 1 FROM paper_refs WHERE paper_id = ?1)", params![id], |r| r.get(0))?;
    let added = !has && !refs.is_empty();
    if added {
        let mut add = tx.prepare_cached("INSERT INTO paper_refs (paper_id, position, raw) VALUES (?1, ?2, ?3)")?;
        for (pos, raw) in refs.iter().enumerate() {
            add.execute(params![id, pos as i64, raw])?;
        }
        drop(add);
        tx.execute("UPDATE papers SET ref_count = MAX(ref_count, ?2) WHERE id = ?1", params![id, refs.len() as i64])?;
    }
    tx.execute("INSERT OR IGNORE INTO crossref_checked (paper_id) VALUES (?1)", params![id])?;
    tx.commit()?;
    Ok(added)
}

#[cfg(test)]
mod tests {
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    #[test]
    fn offers_documents_once_and_never_overwrites_references() {
        let mut conn = open_memory();
        let p = |id: &str, doi: &str, refs: &[&str]| NewPaper {
            ext_id: id.into(), title: id.into(), doi: Some(doi.into()), refs: refs.iter().map(|r| r.to_string()).collect(), ..Default::default()
        };
        insert_many(&mut conn, &[p("a", "10.1/a", &[]), p("b", "10.1/b", &["HE K, 2016, CVPR"]), p("c", "10.1/c,x", &[])], "Test").unwrap();
        let todo = super::pending(&conn, 10).unwrap();
        assert_eq!(todo.iter().map(|t| t.1.as_str()).collect::<Vec<_>>(), vec!["10.1/a"], "b has references; c is unsafe");
        assert_eq!(super::remaining(&conn).unwrap(), 1);
        let a = todo[0].0;
        assert!(super::store(&mut conn, a, &["QIN ZZ, 2019, SCI REP".into()]).unwrap());
        assert!(!super::store(&mut conn, a, &["OTHER, 2020".into()]).unwrap(), "existing references stay");
        assert_eq!(super::remaining(&conn).unwrap(), 0);
        let n: i64 = conn.query_row("SELECT COUNT(*) FROM paper_refs WHERE paper_id = ?1", [a], |r| r.get(0)).unwrap();
        assert_eq!(n, 1);
    }
}
