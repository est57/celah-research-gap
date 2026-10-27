//! Progress through the research workflow, counted from the open project: what was
//! collected, what still needs cleaning, and how far gap review and verification got.

use super::{author_merge, author_suggest, duplicates, synonyms};
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Workflow {
    pub documents: i64,
    pub origins: i64,
    pub with_abstract: i64,
    pub with_keywords: i64,
    pub duplicates: usize,
    pub synonym_suggestions: usize,
    pub synonyms: usize,
    pub name_suggestions: usize,
    pub name_merges: usize,
    /// Filled in from the analysis connection by the caller.
    pub gaps: usize,
    /// Of the most cited references (up to 50, filled in by the caller), those missing from the collection.
    pub core_listed: usize,
    pub core_missing: usize,
    pub reviewed: i64,
    pub accepted: i64,
    /// Accepted gaps with a verification result.
    pub verified: i64,
}

fn count(conn: &Connection, sql: &str) -> AppResult<i64> {
    Ok(conn.query_row(sql, [], |r| r.get(0))?)
}

pub fn status(conn: &Connection) -> AppResult<Workflow> {
    // Every SQL text below is a fixed literal.
    Ok(Workflow {
        documents: count(conn, "SELECT COUNT(*) FROM papers")?,
        origins: count(conn, "SELECT COUNT(DISTINCT origin) FROM papers")?,
        with_abstract: count(conn, "SELECT COUNT(*) FROM papers WHERE COALESCE(abstract, '') <> ''")?,
        with_keywords: count(conn, "SELECT COUNT(*) FROM papers WHERE COALESCE(keywords, '') <> '' OR COALESCE(keywords_plus, '') <> ''")?,
        duplicates: duplicates::list(conn)?.pairs.len(),
        synonym_suggestions: synonyms::suggest::suggest(conn)?.len(),
        synonyms: synonyms::list(conn)?.len(),
        name_suggestions: author_suggest::suggest(conn)?.len(),
        name_merges: author_merge::list(conn)?.len(),
        gaps: 0,
        core_listed: 0,
        core_missing: 0,
        reviewed: count(conn, "SELECT COUNT(*) FROM gap_reviews")?,
        accepted: count(conn, "SELECT COUNT(*) FROM gap_reviews WHERE status = 'accepted'")?,
        verified: count(conn, "SELECT COUNT(*) FROM gap_reviews r JOIN gap_checks c ON c.gap_key = r.gap_key WHERE r.status = 'accepted'")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper, reviews};

    #[test]
    fn counts_progress() {
        let mut conn = open_memory();
        let p = |id: &str, title: &str, year: i32| NewPaper { ext_id: id.into(), title: title.into(), year: Some(year), abstract_text: Some("x".into()), ..Default::default() };
        let items = [p("a", "Audio spectrogram transformers for tuberculosis cough screening", 2023), p("b", "Audio spectrogram transformers for tuberculosis cough screening", 2024), p("c", "Other study", 2022)];
        insert_many(&mut conn, &items, "Test").unwrap();
        reviews::set(&conn, "matrix:a:b", "accepted", "", "").unwrap();
        reviews::set(&conn, "matrix:c:d", "rejected", "", "").unwrap();
        let w = status(&conn).unwrap();
        assert_eq!((w.documents, w.origins, w.with_abstract, w.with_keywords), (3, 1, 3, 0));
        assert_eq!(w.duplicates, 1, "same title in two years");
        assert_eq!((w.reviewed, w.accepted, w.verified), (2, 1, 0));
    }
}
