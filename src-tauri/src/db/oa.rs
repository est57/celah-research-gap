//! Stored open-access lookups: which documents still need one, and the links found.

use crate::error::AppResult;
use crate::unpaywall::OaLink;
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OaStatus {
    pub with_doi: i64,
    pub checked: i64,
    pub open: i64,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DocLinks {
    pub id: i64,
    /// An open-access copy is known (PDF or landing page).
    pub open: bool,
    pub pdf: bool,
    pub doi: bool,
}

/// (id, DOI) of documents with a DOI and no lookup yet, newest first.
pub fn pending(conn: &Connection, limit: i64) -> AppResult<Vec<(i64, String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, doi FROM papers WHERE doi IS NOT NULL AND doi <> '' AND id NOT IN (SELECT paper_id FROM paper_oa)
         ORDER BY year DESC, id DESC LIMIT ?1",
    )?;
    let rows = stmt.query_map(params![limit], |r| Ok((r.get(0)?, r.get(1)?)))?;
    Ok(rows.collect::<Result<_, _>>()?)
}

pub fn store(conn: &Connection, id: i64, link: &OaLink) -> AppResult<()> {
    conn.execute(
        "INSERT INTO paper_oa (paper_id, is_oa, url, pdf_url, host, license) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(paper_id) DO UPDATE SET is_oa = excluded.is_oa, url = excluded.url, pdf_url = excluded.pdf_url,
            host = excluded.host, license = excluded.license, checked_at = datetime('now')",
        params![id, link.is_oa, link.url, link.pdf_url, link.host, link.license],
    )?;
    Ok(())
}

pub fn status(conn: &Connection) -> AppResult<OaStatus> {
    Ok(conn.query_row(
        "SELECT (SELECT COUNT(*) FROM papers WHERE doi IS NOT NULL AND doi <> ''),
                (SELECT COUNT(*) FROM paper_oa),
                (SELECT COUNT(*) FROM paper_oa WHERE url IS NOT NULL OR pdf_url IS NOT NULL)",
        [],
        |r| Ok(OaStatus { with_doi: r.get(0)?, checked: r.get(1)?, open: r.get(2)? }),
    )?)
}

/// What can be opened for each of `ids`; unknown ids are left out.
pub fn links(conn: &Connection, ids: &[i64]) -> AppResult<Vec<DocLinks>> {
    let mut stmt = conn.prepare_cached(
        "SELECT p.id, o.url IS NOT NULL OR o.pdf_url IS NOT NULL, o.pdf_url IS NOT NULL, p.doi IS NOT NULL AND p.doi <> ''
         FROM papers p LEFT JOIN paper_oa o ON o.paper_id = p.id WHERE p.id = ?1",
    )?;
    let mut out = Vec::new();
    for id in ids {
        let row = stmt
            .query_row(params![id], |r| Ok(DocLinks { id: r.get(0)?, open: r.get::<_, Option<bool>>(1)?.unwrap_or(false), pdf: r.get::<_, Option<bool>>(2)?.unwrap_or(false), doi: r.get(3)? }))
            .optional()?;
        out.extend(row);
    }
    Ok(out)
}

/// The address to open for a document: the PDF, else the open landing page, else its DOI page.
pub fn target(conn: &Connection, id: i64) -> AppResult<Option<String>> {
    let row: Option<(Option<String>, Option<String>, Option<String>)> = conn
        .query_row(
            "SELECT o.pdf_url, o.url, p.doi FROM papers p LEFT JOIN paper_oa o ON o.paper_id = p.id WHERE p.id = ?1",
            params![id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .optional()?;
    let Some((pdf, page, doi)) = row else { return Ok(None) };
    let doi_page = doi.filter(|d| !d.is_empty()).map(|d| format!("https://doi.org/{d}"));
    Ok(pdf.or(page).or(doi_page).and_then(|u| crate::unpaywall::web_url(&u)))
}

#[cfg(test)]
#[path = "oa_tests.rs"]
mod tests;
