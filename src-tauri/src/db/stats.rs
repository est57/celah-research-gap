use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
pub struct OriginCount {
    pub origin: String,
    pub count: i64,
}

#[derive(Serialize)]
pub struct FieldStat {
    pub tag: &'static str,
    pub label: &'static str,
    pub missing: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionStats {
    pub total: i64,
    /// Whether a collection filter is active, and how many documents the analyses use.
    pub filtered: bool,
    pub analyzed: i64,
    pub origins: Vec<OriginCount>,
    pub fields: Vec<FieldStat>,
}

/// Field tags follow the bibliometrix / Web of Science convention.
/// Each SQL condition is a fixed literal, never user input.
pub const FIELDS: [(&str, &str, &str); 13] = [
    ("AU", "Penulis", "author_count = 0"),
    ("TI", "Judul", "title = ''"),
    ("PY", "Tahun terbit", "year IS NULL"),
    ("SO", "Sumber", "source_name IS NULL OR source_name = ''"),
    ("TC", "Jumlah sitasi", "cited_by IS NULL"),
    ("AB", "Abstrak", "abstract IS NULL OR abstract = ''"),
    ("DI", "DOI", "doi IS NULL"),
    ("C1", "Afiliasi", "has_affiliation = 0"),
    ("DE", "Kata kunci penulis", "keywords IS NULL OR keywords = ''"),
    ("ID", "Keywords Plus", "keywords_plus IS NULL OR keywords_plus = ''"),
    // Counts alone (OpenAlex, Semantic Scholar) cannot feed co-citation, so only stored reference text counts.
    ("CR", "Referensi", "NOT EXISTS (SELECT 1 FROM paper_refs r WHERE r.paper_id = papers.id)"),
    ("UN", "Institusi", "NOT EXISTS (SELECT 1 FROM paper_affiliations f WHERE f.paper_id = papers.id)"),
    ("CU", "Negara afiliasi", "NOT EXISTS (SELECT 1 FROM paper_countries c WHERE c.paper_id = papers.id)"),
];

pub fn collection(conn: &Connection) -> AppResult<CollectionStats> {
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM papers", [], |r| r.get(0))?;

    let mut stmt = conn.prepare("SELECT origin, COUNT(*) FROM papers GROUP BY origin ORDER BY 2 DESC")?;
    let origins = stmt
        .query_map([], |r| Ok(OriginCount { origin: r.get(0)?, count: r.get(1)? }))?
        .collect::<Result<Vec<_>, _>>()?;

    let sums: Vec<String> = FIELDS
        .iter()
        .map(|(_, _, cond)| format!("COALESCE(SUM(CASE WHEN {cond} THEN 1 ELSE 0 END), 0)"))
        .collect();
    let sql = format!("SELECT {} FROM papers", sums.join(", "));
    let fields = conn.query_row(&sql, [], |r| {
        FIELDS
            .iter()
            .enumerate()
            .map(|(i, (tag, label, _))| Ok(FieldStat { tag, label, missing: r.get(i)? }))
            .collect::<Result<Vec<_>, _>>()
    })?;

    let active = super::filter::load(conn)?;
    let analyzed = if active.is_active() { super::filter::matching(conn, &active)? } else { total };
    Ok(CollectionStats { total, filtered: active.is_active(), analyzed, origins, fields })
}
