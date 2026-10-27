//! Paper fields needed to compare and rank duplicate candidates.

use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DupPaper {
    pub id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub origin: String,
    pub source: Option<String>,
    pub doc_type: Option<String>,
    pub doi: Option<String>,
    pub authors: String,
    pub cited_by: i64,
    /// Labels of the optional fields this record has, e.g. "Abstrak".
    pub has: Vec<&'static str>,
    #[serde(skip)]
    pub title_key: String,
    #[serde(skip)]
    pub first_author: String,
}

/// Preprint servers have DOIs of their own, so a preprint DOI never rules out a match.
const PREPRINT_DOI: [&str; 5] = ["10.48550/", "10.1101/", "10.21203/", "10.2139/", "10.31219/"];

impl DupPaper {
    pub fn is_preprint(&self) -> bool {
        self.doc_type.as_deref() == Some("preprint")
            || self.origin == "arXiv"
            || self.doi.as_deref().is_some_and(|d| PREPRINT_DOI.iter().any(|p| d.starts_with(p)))
    }

    /// Higher is the better record to keep: published version first, then richer metadata.
    pub fn completeness(&self) -> (i64, i64) {
        let fields = self.has.len() as i64 + i64::from(self.has.contains(&"Referensi")) + i64::from(self.doi.is_some());
        (i64::from(!self.is_preprint()) * 10 + fields, self.cited_by)
    }
}

pub fn load(conn: &Connection) -> AppResult<Vec<DupPaper>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.title, p.title_key, p.year, p.doi, p.origin, p.source_name, p.doc_type, p.authors, p.cited_by,
                COALESCE(p.abstract, '') <> '',
                COALESCE(p.keywords, '') <> '' OR COALESCE(p.keywords_plus, '') <> '',
                EXISTS (SELECT 1 FROM paper_refs r WHERE r.paper_id = p.id),
                EXISTS (SELECT 1 FROM paper_countries c WHERE c.paper_id = p.id),
                COALESCE((SELECT a.name_key FROM paper_authors a WHERE a.paper_id = p.id ORDER BY a.position LIMIT 1), '')
         FROM papers p",
    )?;
    let rows = stmt.query_map([], |r| {
        let flags: [(bool, &'static str); 4] =
            [(r.get(10)?, "Abstrak"), (r.get(11)?, "Kata kunci"), (r.get(12)?, "Referensi"), (r.get(13)?, "Negara")];
        let authors: String = r.get(8)?;
        Ok(DupPaper {
            id: r.get(0)?,
            title: r.get(1)?,
            title_key: r.get(2)?,
            year: r.get(3)?,
            doi: r.get(4)?,
            origin: r.get(5)?,
            source: r.get(6)?,
            doc_type: r.get(7)?,
            authors: authors.chars().take(160).collect(),
            cited_by: r.get(9)?,
            has: flags.iter().filter(|f| f.0).map(|f| f.1).collect(),
            first_author: r.get(14)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}
