//! Removing documents from the collection. Every table that refers to a paper
//! cascades (references, authors, countries, institutions, cites, aliases, checks).

use crate::error::{AppError, AppResult};
use rusqlite::{params, Connection};

/// One request removes at most this many selected documents.
pub const MAX_IDS: usize = 10_000;
const MAX_ORIGIN: usize = 100;

/// Removes the given documents; unknown ids are ignored. Returns how many were removed.
pub fn by_ids(conn: &mut Connection, ids: &[i64]) -> AppResult<usize> {
    if ids.is_empty() || ids.len() > MAX_IDS || ids.iter().any(|&id| id <= 0) {
        return Err(AppError::Invalid(crate::tr!(format!("pilih 1 sampai {MAX_IDS} dokumen"), format!("choose 1 to {MAX_IDS} documents"))));
    }
    let tx = conn.transaction()?;
    let mut removed = 0;
    {
        let mut stmt = tx.prepare_cached("DELETE FROM papers WHERE id = ?1")?;
        for id in ids {
            removed += stmt.execute(params![id])?;
        }
    }
    tx.commit()?;
    Ok(removed)
}

/// Removes every document imported from `origin` (e.g. "Crossref").
pub fn by_origin(conn: &Connection, origin: &str) -> AppResult<usize> {
    if origin.is_empty() || origin.chars().count() > MAX_ORIGIN || origin.chars().any(char::is_control) {
        return Err(AppError::Invalid(crate::tr!("sumber impor tidak sah", "invalid import source").into()));
    }
    let removed = conn.execute("DELETE FROM papers WHERE origin = ?1", params![origin])?;
    if removed == 0 {
        return Err(AppError::Invalid(crate::tr!("tidak ada dokumen dari sumber impor itu", "no documents from that import source").into()));
    }
    Ok(removed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    fn paper(id: &str) -> NewPaper {
        NewPaper {
            ext_id: id.into(),
            title: format!("Title {id}"),
            author_names: vec!["Rahman M".into()],
            refs: vec!["R".into()],
            countries: vec!["ID".into()],
            ..Default::default()
        }
    }

    #[test]
    fn removes_documents_and_everything_attached() {
        let mut conn = open_memory();
        insert_many(&mut conn, &[paper("a"), paper("b")], "OpenAlex").unwrap();
        insert_many(&mut conn, &[paper("c")], "Crossref").unwrap();
        let count = |c: &Connection, t: &str| c.query_row(&format!("SELECT COUNT(*) FROM {t}"), [], |r| r.get::<_, i64>(0)).unwrap();
        let a: i64 = conn.query_row("SELECT id FROM papers WHERE ext_id = 'a'", [], |r| r.get(0)).unwrap();

        assert_eq!(by_ids(&mut conn, &[a, 99_999]).unwrap(), 1, "unknown ids are ignored");
        assert_eq!((count(&conn, "papers"), count(&conn, "paper_refs"), count(&conn, "paper_authors")), (2, 2, 2));
        assert_eq!(by_origin(&conn, "Crossref").unwrap(), 1);
        assert_eq!(count(&conn, "paper_countries"), 1);
        assert!(by_origin(&conn, "Scopus").is_err());
        assert!(by_ids(&mut conn, &[]).is_err());
        assert!(by_ids(&mut conn, &[-1]).is_err());
        assert!(by_origin(&conn, "x\u{0}").is_err());
        assert!(by_origin(&conn, "OpenAlex' OR '1'='1").is_err(), "bound as a value, so it matches nothing");
        assert_eq!(count(&conn, "papers"), 1);
    }
}
