//! Journal quartiles of the (filtered) collection, from the imported SJR list.

use crate::db::sjr::{self, SjrStatus, QUARTILE_SQL, SJR_SQL};
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;

const TOP_SOURCES: usize = 40;
const LABELS: [&str; 5] = ["Q1", "Q2", "Q3", "Q4", ""];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuartileSource {
    pub label: String,
    pub documents: usize,
    /// "Q1".."Q4", or "" when the journal is not in the SJR list.
    pub quartile: String,
    pub sjr: Option<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Quartiles {
    pub status: SjrStatus,
    pub documents: usize,
    /// Documents per quartile: Q1, Q2, Q3, Q4, then without a quartile.
    pub counts: [usize; 5],
    pub sources: Vec<QuartileSource>,
}

pub fn compute(conn: &Connection) -> AppResult<Quartiles> {
    let status = sjr::status(conn)?;
    let sql = format!("SELECT lower(trim(COALESCE(source_name, ''))), trim(COALESCE(source_name, '')), {QUARTILE_SQL}, {SJR_SQL} FROM papers");
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;
    let (mut counts, mut documents) = ([0usize; 5], 0);
    let mut by_source: HashMap<String, QuartileSource> = HashMap::new();
    while let Some(r) = rows.next()? {
        let (key, label, q, value): (String, String, String, Option<f64>) = (r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?);
        documents += 1;
        counts[LABELS.iter().position(|l| *l == q).unwrap_or(4)] += 1;
        if key.is_empty() {
            continue;
        }
        let e = by_source.entry(key).or_insert(QuartileSource { label, documents: 0, quartile: q, sjr: value });
        e.documents += 1;
    }
    let mut sources: Vec<QuartileSource> = by_source.into_values().collect();
    sources.sort_by(|a, b| b.documents.cmp(&a.documents).then(a.label.cmp(&b.label)));
    sources.truncate(TOP_SOURCES);
    Ok(Quartiles { status, documents, counts, sources })
}

#[cfg(test)]
mod tests {
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper, sjr};
    use crate::import::sjr::Journal;

    #[test]
    fn counts_documents_per_quartile() {
        let mut conn = open_memory();
        let j = |issn: &str, title: &str, q: &str| Journal { issns: issn.into(), title: title.into(), sjr: Some(0.9), quartile: Some(q.into()), categories: None };
        sjr::replace(&mut conn, &[j("09252312", "Neurocomputing", "Q1"), j("21693536", "IEEE Access", "Q2")], Some(2024)).unwrap();
        let p = |id: &str, issn: &str, source: &str| NewPaper { ext_id: id.into(), title: id.into(), issn: Some(issn.into()), source_name: Some(source.into()), ..Default::default() };
        insert_many(&mut conn, &[p("a", "09252312", "Neurocomputing"), p("b", "09252312", "Neurocomputing"), p("c", "21693536", "IEEE Access"), p("d", "00000000", "Local Bulletin")], "Test").unwrap();
        let q = super::compute(&conn).unwrap();
        assert_eq!((q.documents, q.counts), (4, [2, 1, 0, 0, 1]));
        assert_eq!((q.sources[0].label.as_str(), q.sources[0].quartile.as_str(), q.sources[0].documents), ("Neurocomputing", "Q1", 2));
        assert_eq!(q.sources[0].sjr, Some(0.9));
        assert_eq!(q.status.year, Some(2024));
    }
}
