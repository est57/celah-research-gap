//! Gap verification: search recent literature for papers that may already
//! address a gap, skipping papers already in the collection.

use crate::db::checks::CheckPaper;
use crate::db::insert::title_key;
use crate::db::papers::NewPaper;
use crate::error::AppResult;
use rusqlite::{params, Connection};

/// Candidates kept per gap after sorting matches first.
const KEEP: usize = 20;
const MAX_QUERY: usize = 300;

pub fn tokens(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() >= 2)
        .map(str::to_string)
        .collect()
}

/// Every word of `term` appears in `text`; a trailing "s" is ignored so
/// "network" matches "networks" and the other way round.
pub fn mentions(text_tokens: &[String], term: &str) -> bool {
    let stem = |w: &str| w.strip_suffix('s').filter(|s| s.len() >= 3).unwrap_or(w).to_string();
    let words: Vec<String> = tokens(term).iter().map(|w| stem(w)).collect();
    !words.is_empty() && words.iter().all(|w| text_tokens.iter().any(|t| stem(t) == *w))
}

pub fn matches_all(title: &str, abstract_text: Option<&str>, terms: &[String]) -> bool {
    let text = tokens(&format!("{title} {}", abstract_text.unwrap_or("")));
    !terms.is_empty() && terms.iter().all(|t| mentions(&text, t))
}

/// Search text for the gap's terms, bounded to the length the search APIs accept.
pub fn query_text(terms: &[String]) -> String {
    terms.join(" ").chars().filter(|c| !c.is_control()).take(MAX_QUERY).collect()
}

pub fn in_collection(conn: &Connection, p: &NewPaper) -> AppResult<bool> {
    Ok(conn
        .prepare_cached(
            "SELECT 1 FROM papers WHERE ext_id = ?1 OR (doi IS NOT NULL AND doi = ?2)
             OR (title_key = ?3 AND title_key <> '' AND year IS ?4)
             OR EXISTS (SELECT 1 FROM paper_aliases WHERE ext_id = ?1 OR (doi IS NOT NULL AND doi = ?2)) LIMIT 1",
        )?
        .exists(params![p.ext_id, p.doi, title_key(&p.title), p.year])?)
}

/// New papers (not in the collection), matching ones first, then by citations.
pub fn candidates(conn: &Connection, found: &[NewPaper], terms: &[String]) -> AppResult<Vec<CheckPaper>> {
    let mut out = Vec::new();
    for p in found {
        if in_collection(conn, p)? {
            continue;
        }
        out.push(CheckPaper {
            title: p.title.clone(),
            year: p.year,
            doi: p.doi.clone(),
            source: p.source_name.clone(),
            cited_by: p.cited_by,
            matches: matches_all(&p.title, p.abstract_text.as_deref(), terms),
        });
    }
    out.sort_by(|a, b| b.matches.cmp(&a.matches).then(b.cited_by.cmp(&a.cited_by)));
    out.truncate(KEEP);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{insert::insert_many, open_memory};

    #[test]
    fn matches_every_term_with_simple_plurals() {
        let terms = vec!["transformer".to_string(), "cough sound".to_string()];
        assert!(matches_all("Transformers for cough sounds in TB triage", None, &terms));
        assert!(matches_all("Audio models", Some("A transformer classifies the cough sound."), &terms));
        assert!(!matches_all("Transformers for chest X-ray", Some("No audio here."), &terms));
        assert!(!matches_all("Anything", None, &[]));
    }

    #[test]
    fn skips_papers_already_in_collection() {
        let mut conn = open_memory();
        let known = NewPaper { ext_id: "openalex:W1".into(), doi: Some("10.1/known".into()), title: "Known paper".into(), year: Some(2024), ..Default::default() };
        insert_many(&mut conn, std::slice::from_ref(&known), "Test").unwrap();
        let found = vec![
            NewPaper { ext_id: "s2:x".into(), doi: Some("10.1/known".into()), title: "Same DOI".into(), ..Default::default() },
            NewPaper { ext_id: "openalex:W9".into(), title: "Transformer cough sound study".into(), year: Some(2025), ..Default::default() },
        ];
        let c = candidates(&conn, &found, &["transformer".into(), "cough sound".into()]).unwrap();
        assert_eq!(c.len(), 1);
        assert!(c[0].matches);
    }
}
