//! Most cited sources and first authors among the collection's references (bibliometrix
//! Most Local Cited Sources and Most Local Cited Authors). Each citing document counts once
//! per source or author.

use super::rpys::parse;
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

const TOP: usize = 30;

#[derive(Serialize)]
pub struct Cited {
    pub label: String,
    pub documents: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CitedEntities {
    /// Text references read.
    pub references: usize,
    pub sources: Vec<Cited>,
    pub authors: Vec<Cited>,
}

/// Source of a reference: "CHEN L, 2019, J DIGIT IMAGING, V32" (WoS, Crossref) gives the
/// third field; "Lakhani P., Title, (2017) Radiology, 284" (Scopus) the text after the year.
pub fn source_of(raw: &str) -> Option<String> {
    let scopus = raw.find(") ").filter(|&i| i >= 5 && raw[i - 5..i].starts_with('(') && raw[i - 4..i].chars().all(|c| c.is_ascii_digit()));
    let text = match scopus {
        Some(i) => raw[i + 2..].split(',').next()?,
        None => {
            let parts: Vec<&str> = raw.split(',').map(str::trim).collect();
            let year_second = parts.get(1).is_some_and(|p| p.len() == 4 && p.chars().all(|c| c.is_ascii_digit()));
            if !year_second {
                return None;
            }
            *parts.get(2)?
        }
    };
    let clean = text.trim().trim_end_matches('.').trim();
    let letters = clean.chars().filter(|c| c.is_alphabetic()).count();
    // Volume, page or DOI fields are not sources.
    let looks_like_field = clean.starts_with("DOI") || clean.starts_with("10.") || (clean.len() < 8 && clean.starts_with(['V', 'P']) && clean[1..].chars().all(|c| c.is_ascii_digit()));
    (letters >= 3 && clean.chars().count() <= 120 && !looks_like_field).then(|| clean.to_string())
}

#[derive(Default)]
struct Tally {
    papers: HashSet<i64>,
    spellings: HashMap<String, usize>,
}

fn top(map: HashMap<String, Tally>) -> Vec<Cited> {
    let mut list: Vec<Cited> = map
        .into_values()
        .map(|t| {
            let label = t.spellings.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))).map(|(l, _)| l.clone()).unwrap_or_default();
            Cited { label, documents: t.papers.len() }
        })
        .collect();
    list.sort_by(|a, b| b.documents.cmp(&a.documents).then(a.label.cmp(&b.label)));
    list.truncate(TOP);
    list
}

pub fn compute(conn: &Connection) -> AppResult<CitedEntities> {
    let mut stmt = conn.prepare("SELECT paper_id, raw FROM paper_refs")?;
    let mut rows = stmt.query([])?;
    let (mut sources, mut authors): (HashMap<String, Tally>, HashMap<String, Tally>) = Default::default();
    let mut references = 0;
    while let Some(r) = rows.next()? {
        let (paper, raw): (i64, String) = (r.get(0)?, r.get(1)?);
        references += 1;
        if let Some(s) = source_of(&raw) {
            let t = sources.entry(s.to_lowercase()).or_default();
            t.papers.insert(paper);
            *t.spellings.entry(s).or_default() += 1;
        }
        // The RPYS reader drops DOIs first, so "DOI 10.1/x" is never read as an author.
        if let Some(label) = parse::reference(&raw, 2100).map(|p| p.label).filter(|l| l.contains(' ')) {
            let who = label.rsplit_once(' ').map_or(label.as_str(), |(w, _)| w).to_string();
            let t = authors.entry(who.to_lowercase()).or_default();
            t.papers.insert(paper);
            *t.spellings.entry(who).or_default() += 1;
        }
    }
    Ok(CitedEntities { references, sources: top(sources), authors: top(authors) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    #[test]
    fn reads_sources_in_wos_scopus_and_crossref_formats() {
        assert_eq!(source_of("CHEN L, 2019, J DIGIT IMAGING, V32, P1").as_deref(), Some("J DIGIT IMAGING"));
        assert_eq!(source_of("Lakhani P., Sundaram B., Deep learning at chest radiography, (2017) Radiology, 284").as_deref(), Some("Radiology"));
        assert_eq!(source_of("Jaeger, 2014, Quant Imaging Med Surg., Two public chest X-ray datasets").as_deref(), Some("Quant Imaging Med Surg"));
        assert!(source_of("CHEN L, 2019, V32, P1").is_none(), "a volume is not a source");
        assert!(source_of("some free text without structure").is_none());
    }

    #[test]
    fn counts_citing_documents_per_source_and_author() {
        let mut conn = open_memory();
        let p = |id: &str, refs: &[&str]| NewPaper { ext_id: id.into(), title: id.into(), refs: refs.iter().map(|r| r.to_string()).collect(), ..Default::default() };
        let items = [
            p("a", &["CHEN L, 2019, RADIOLOGY, V1", "CHEN L, 2020, RADIOLOGY, V2"]),
            p("b", &["Chen L., Title, (2021) Radiology, 3", "QIN ZZ, 2019, SCI REP, V9"]),
        ];
        insert_many(&mut conn, &items, "Test").unwrap();
        let c = compute(&conn).unwrap();
        assert_eq!(c.references, 4);
        assert_eq!((c.sources[0].label.as_str(), c.sources[0].documents), ("RADIOLOGY", 2), "one count per citing document, spellings joined");
        assert_eq!((c.authors[0].label.as_str(), c.authors[0].documents), ("Chen", 2));
    }
}
