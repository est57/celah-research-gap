use super::terms;
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeCount {
    pub doc_type: String,
    pub count: i64,
}

/// The "Main information" table of bibliometrix.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MainInfo {
    pub year_min: Option<i32>,
    pub year_max: Option<i32>,
    pub documents: i64,
    pub sources: i64,
    pub growth_rate: Option<f64>,
    pub avg_age: Option<f64>,
    pub avg_citations: f64,
    pub references: i64,
    pub keywords_plus: usize,
    pub author_keywords: usize,
    pub authors: i64,
    pub single_author_docs: i64,
    pub authors_of_single: i64,
    pub coauthors_per_doc: Option<f64>,
    /// Share of documents with known countries that have authors from two or more countries.
    pub international_coauthorship: Option<f64>,
    pub doc_types: Vec<TypeCount>,
}

fn count(conn: &Connection, sql: &str) -> AppResult<i64> {
    Ok(conn.query_row(sql, [], |r| r.get(0))?)
}

pub fn compute(conn: &Connection, current_year: i32, growth_rate: Option<f64>) -> AppResult<MainInfo> {
    let (year_min, year_max, documents, sources, avg_citations, coauthors_per_doc, single, avg_year) = conn.query_row(
        "SELECT MIN(year), MAX(year), COUNT(*), COUNT(DISTINCT lower(source_name)), COALESCE(AVG(cited_by), 0),
                AVG(NULLIF(author_count, 0)), COALESCE(SUM(author_count = 1), 0), AVG(year)
         FROM papers",
        [],
        |r| {
            Ok((
                r.get::<_, Option<i32>>(0)?,
                r.get::<_, Option<i32>>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, i64>(3)?,
                r.get::<_, f64>(4)?,
                r.get::<_, Option<f64>>(5)?,
                r.get::<_, i64>(6)?,
                r.get::<_, Option<f64>>(7)?,
            ))
        },
    )?;

    let mut author_kw = HashSet::new();
    let mut plus_kw = HashSet::new();
    let mut stmt = conn.prepare("SELECT keywords, keywords_plus FROM papers")?;
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        author_kw.extend(terms(r.get::<_, Option<String>>(0)?.as_deref()));
        plus_kw.extend(terms(r.get::<_, Option<String>>(1)?.as_deref()));
    }

    let mut stmt = conn.prepare("SELECT COALESCE(doc_type, 'lainnya'), COUNT(*) FROM papers GROUP BY 1 ORDER BY 2 DESC")?;
    let doc_types = stmt
        .query_map([], |r| Ok(TypeCount { doc_type: r.get(0)?, count: r.get(1)? }))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(MainInfo {
        year_min,
        year_max,
        documents,
        sources,
        growth_rate,
        avg_age: avg_year.map(|y| f64::from(current_year) - y),
        avg_citations,
        references: count(conn, "SELECT COUNT(DISTINCT lower(raw)) FROM paper_refs")?,
        keywords_plus: plus_kw.len(),
        author_keywords: author_kw.len(),
        authors: count(conn, "SELECT COUNT(DISTINCT name_key) FROM paper_authors")?,
        single_author_docs: single,
        authors_of_single: count(
            conn,
            "SELECT COUNT(DISTINCT a.name_key) FROM paper_authors a
             JOIN papers p ON p.id = a.paper_id WHERE p.author_count = 1",
        )?,
        coauthors_per_doc,
        international_coauthorship: conn.query_row(
            "SELECT AVG(n > 1) FROM (SELECT COUNT(*) AS n FROM paper_countries GROUP BY paper_id)",
            [],
            |r| r.get(0),
        )?,
        doc_types,
    })
}
