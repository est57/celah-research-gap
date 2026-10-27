use crate::error::AppResult;
use rusqlite::{params, Connection};
use serde::Serialize;

/// A paper ready to be stored. Strings are already sanitized and bounded.
#[derive(Default)]
pub struct NewPaper {
    /// Source-prefixed id, e.g. "openalex:W123", "wos:WOS:000123", "scopus:2-s2.0-85".
    pub ext_id: String,
    pub doi: Option<String>,
    pub title: String,
    pub year: Option<i32>,
    pub source_name: Option<String>,
    pub doc_type: Option<String>,
    pub cited_by: i64,
    /// Display form: "A; B; +n".
    pub authors: String,
    pub author_count: i64,
    /// Every author name, in byline order (stored in paper_authors).
    pub author_names: Vec<String>,
    /// Distinct affiliation countries (ISO alpha-2), first-seen order.
    pub countries: Vec<String>,
    /// Corresponding author's country, or the first author's when unknown.
    pub corresponding_country: Option<String>,
    pub has_affiliation: bool,
    pub abstract_text: Option<String>,
    /// Author keywords (DE).
    pub keywords: Option<String>,
    /// Indexer keywords (ID): Keywords Plus or Scopus index keywords.
    pub keywords_plus: Option<String>,
    /// Cited references (CR), one raw string each.
    pub refs: Vec<String>,
    /// Source ids of cited works, e.g. "openalex:W123" (OpenAlex referenced_works).
    pub cites: Vec<String>,
    /// Distinct institutions of the authors.
    pub affiliations: Vec<crate::geo::affil::Affiliation>,
    pub ref_count: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaperRow {
    pub id: i64,
    pub title: String,
    pub authors: String,
    pub year: Option<i32>,
    pub source_name: Option<String>,
    pub doc_type: Option<String>,
    pub cited_by: i64,
    pub origin: String,
}

/// Escapes LIKE wildcards so user text is matched literally.
pub fn like_pattern(text: &str) -> String {
    let escaped = text.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_");
    format!("%{escaped}%")
}

const WHERE_FILTER: &str = "(?1 IS NULL OR title LIKE ?1 ESCAPE '\\'
    OR authors LIKE ?1 ESCAPE '\\' OR keywords LIKE ?1 ESCAPE '\\')";

pub fn list(conn: &Connection, filter: Option<&str>, limit: u32, offset: u32) -> AppResult<(Vec<PaperRow>, i64)> {
    let pattern = filter.map(like_pattern);
    let total: i64 = conn.query_row(
        &format!("SELECT COUNT(*) FROM papers WHERE {WHERE_FILTER}"),
        params![pattern],
        |r| r.get(0),
    )?;
    let mut stmt = conn.prepare(&format!(
        "SELECT id, title, authors, year, source_name, doc_type, cited_by, origin FROM papers
         WHERE {WHERE_FILTER} ORDER BY cited_by DESC, id ASC LIMIT ?2 OFFSET ?3"
    ))?;
    let rows = stmt
        .query_map(params![pattern, limit, offset], |r| {
            Ok(PaperRow {
                id: r.get(0)?,
                title: r.get(1)?,
                authors: r.get(2)?,
                year: r.get(3)?,
                source_name: r.get(4)?,
                doc_type: r.get(5)?,
                cited_by: r.get(6)?,
                origin: r.get(7)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok((rows, total))
}

#[cfg(test)]
mod tests;
