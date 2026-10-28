//! Search box in the title bar: documents, authors and sources whose names contain
//! the typed text, across the whole collection.

use super::papers::like_pattern;
use crate::error::{AppError, AppResult};
use rusqlite::{params, Connection};
use serde::Serialize;

const PER_GROUP: i64 = 5;

#[derive(Serialize)]
pub struct DocHit {
    pub id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub authors: String,
}

#[derive(Serialize)]
pub struct NameHit {
    pub label: String,
    pub documents: i64,
}

#[derive(Serialize)]
pub struct QuickResults {
    pub documents: Vec<DocHit>,
    pub authors: Vec<NameHit>,
    pub sources: Vec<NameHit>,
}

/// 2 to 100 printable characters.
pub fn validate(raw: &str) -> AppResult<String> {
    let q = raw.trim();
    let n = q.chars().count();
    if !(2..=100).contains(&n) || q.chars().any(char::is_control) {
        return Err(AppError::Invalid(crate::tr!("teks pencarian harus 2 sampai 100 karakter", "search text must be 2 to 100 characters").into()));
    }
    Ok(q.to_string())
}

fn names(conn: &Connection, sql: &str, pattern: &str) -> AppResult<Vec<NameHit>> {
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map(params![pattern, PER_GROUP], |r| Ok(NameHit { label: r.get(0)?, documents: r.get(1)? }))?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn search(conn: &Connection, raw: &str) -> AppResult<QuickResults> {
    let q = validate(raw)?;
    let pattern = like_pattern(&q);
    let mut stmt = conn.prepare(
        "SELECT id, title, year, authors FROM papers WHERE title LIKE ?1 ESCAPE '\\'
         ORDER BY cited_by DESC, id LIMIT ?2",
    )?;
    let documents = stmt
        .query_map(params![pattern, PER_GROUP], |r| Ok(DocHit { id: r.get(0)?, title: r.get(1)?, year: r.get(2)?, authors: r.get(3)? }))?
        .collect::<Result<Vec<_>, _>>()?;
    // Every SQL text here is a fixed literal; only the pattern and the limit are bound.
    let authors = names(conn,
        "SELECT MIN(name), COUNT(DISTINCT paper_id) FROM paper_authors WHERE name LIKE ?1 ESCAPE '\\'
         GROUP BY name_key ORDER BY 2 DESC, 1 LIMIT ?2", &pattern)?;
    let sources = names(conn,
        "SELECT MIN(trim(source_name)), COUNT(*) FROM papers WHERE source_name LIKE ?1 ESCAPE '\\'
         GROUP BY lower(trim(source_name)) ORDER BY 2 DESC, 1 LIMIT ?2", &pattern)?;
    Ok(QuickResults { documents, authors, sources })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    #[test]
    fn finds_documents_authors_and_sources() {
        let mut conn = open_memory();
        let p = |id: &str, title: &str, cited: i64| NewPaper {
            ext_id: id.into(), title: title.into(), cited_by: cited, source_name: Some("Tuberculosis Journal".into()),
            author_names: vec!["Rahman M".into()], ..Default::default()
        };
        insert_many(&mut conn, &[p("a", "Deep learning for TB", 3), p("b", "TB screening 100% recall", 9)], "Test").unwrap();
        let r = search(&conn, "tb").unwrap();
        assert_eq!(r.documents.len(), 2);
        assert_eq!(r.documents[0].title, "TB screening 100% recall", "most cited first");
        assert_eq!(search(&conn, "100%").unwrap().documents.len(), 1, "% is a literal, not a wildcard");
        assert_eq!(search(&conn, "rahm").unwrap().authors[0].documents, 2);
        assert_eq!(search(&conn, "tubercul").unwrap().sources[0].documents, 2);
        assert!(search(&conn, "x").is_err());
        assert!(search(&conn, "a\u{0}b").is_err());
    }
}
