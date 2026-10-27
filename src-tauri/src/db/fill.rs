//! Completes a stored document from a duplicate record imported later, so re-exporting
//! from Scopus with References checked adds the reference lists instead of being skipped.
//! Only empty fields are filled; nothing already stored is overwritten.

use super::insert::title_key;
use super::papers::NewPaper;
use crate::error::AppResult;
use rusqlite::{params, Connection, OptionalExtension};

/// The stored copy of a record: same source id, DOI, or title and year.
pub fn existing(conn: &Connection, p: &NewPaper) -> AppResult<Option<i64>> {
    Ok(conn
        .query_row(
            "SELECT id FROM papers WHERE ext_id = ?1 OR (doi IS NOT NULL AND doi = ?2)
                OR (title_key = ?3 AND title_key <> '' AND year IS ?4)
             ORDER BY (ext_id = ?1) DESC LIMIT 1",
            params![p.ext_id, p.doi, title_key(&p.title), p.year],
            |r| r.get(0),
        )
        .optional()?)
}

/// Adds what document `id` lacks from `p`. Returns true when anything was added.
pub fn fill(conn: &Connection, id: i64, p: &NewPaper) -> AppResult<bool> {
    let mut added = 0;
    let has_refs: bool = conn.query_row("SELECT EXISTS (SELECT 1 FROM paper_refs WHERE paper_id = ?1)", params![id], |r| r.get(0))?;
    if !has_refs && !p.refs.is_empty() {
        let mut add = conn.prepare_cached("INSERT INTO paper_refs (paper_id, position, raw) VALUES (?1, ?2, ?3)")?;
        for (pos, raw) in p.refs.iter().enumerate() {
            add.execute(params![id, pos as i64, raw])?;
        }
        conn.execute("UPDATE papers SET ref_count = MAX(ref_count, ?2) WHERE id = ?1", params![id, p.refs.len() as i64])?;
        added += 1;
    }
    let mut cite = conn.prepare_cached("INSERT OR IGNORE INTO paper_cites (paper_id, cited) VALUES (?1, ?2)")?;
    for c in &p.cites {
        added += cite.execute(params![id, c])?;
    }
    // Each column is set only while it is empty, so these never overwrite stored values.
    let empty_fill: [(&str, Option<&str>); 5] = [
        ("UPDATE papers SET issn = ?2 WHERE id = ?1 AND COALESCE(issn, '') = ''", p.issn.as_deref()),
        ("UPDATE papers SET abstract = ?2 WHERE id = ?1 AND COALESCE(abstract, '') = ''", p.abstract_text.as_deref()),
        ("UPDATE papers SET keywords = ?2 WHERE id = ?1 AND COALESCE(keywords, '') = ''", p.keywords.as_deref()),
        ("UPDATE papers SET keywords_plus = ?2 WHERE id = ?1 AND COALESCE(keywords_plus, '') = ''", p.keywords_plus.as_deref()),
        ("UPDATE papers SET doi = ?2 WHERE id = ?1 AND doi IS NULL", p.doi.as_deref()),
    ];
    for (sql, value) in empty_fill {
        if let Some(v) = value.filter(|v| !v.trim().is_empty()) {
            added += conn.execute(sql, params![id, v])?;
        }
    }
    Ok(added > 0)
}

#[cfg(test)]
mod tests {
    use crate::db::insert::insert_and_fill;
    use crate::db::{open_memory, papers::NewPaper};

    fn paper(id: &str, refs: &[&str], abs: Option<&str>) -> NewPaper {
        NewPaper {
            ext_id: id.into(),
            doi: Some("10.1/x".into()),
            title: "Deep learning for TB".into(),
            year: Some(2024),
            abstract_text: abs.map(Into::into),
            keywords: Some("tb".into()),
            refs: refs.iter().map(|r| r.to_string()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn reimport_with_references_completes_the_stored_copy() {
        let mut conn = open_memory();
        assert_eq!(insert_and_fill(&mut conn, &[paper("scopus:1", &[], Some("first abstract"))], "Scopus").unwrap(), (1, 0, 0));
        let again = [paper("scopus:1", &["QIN ZZ, 2019, SCI REP", "LAKHANI P, 2017, RADIOLOGY"], Some("other abstract"))];
        assert_eq!(insert_and_fill(&mut conn, &again, "Scopus").unwrap(), (0, 1, 1));
        let (docs, refs, count, abs): (i64, i64, i64, String) = conn
            .query_row("SELECT (SELECT COUNT(*) FROM papers), (SELECT COUNT(*) FROM paper_refs), ref_count, abstract FROM papers", [], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
            })
            .unwrap();
        assert_eq!((docs, refs, count), (1, 2, 2), "no duplicate document; the references were added");
        assert_eq!(abs, "first abstract", "stored values are never overwritten");
        // A third import changes nothing: the reference list is already there.
        assert_eq!(insert_and_fill(&mut conn, &again, "Scopus").unwrap(), (0, 1, 0));
    }

    #[test]
    fn a_match_by_title_and_year_from_another_source_is_completed_too() {
        let mut conn = open_memory();
        let mut first = paper("openalex:W1", &[], None);
        first.doi = None;
        insert_and_fill(&mut conn, &[first], "OpenAlex").unwrap();
        let mut wos = paper("wos:1", &["HE K, 2016, PROC CVPR"], Some("abstract"));
        wos.doi = None;
        assert_eq!(insert_and_fill(&mut conn, &[wos], "WoS").unwrap(), (0, 1, 1));
    }
}
