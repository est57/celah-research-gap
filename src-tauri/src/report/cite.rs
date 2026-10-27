//! Cited documents: loading their metadata and formatting citations (APA, IEEE).

use crate::error::AppResult;
use rusqlite::{params_from_iter, Connection};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Style {
    Apa,
    Ieee,
}

#[derive(Clone)]
pub struct Work {
    pub id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub source: Option<String>,
    pub doi: Option<String>,
    pub doc_type: Option<String>,
    pub authors: Vec<String>,
}

/// Loads works by id, with their full author lists in byline order.
pub fn load(conn: &Connection, ids: &[i64]) -> AppResult<HashMap<i64, Work>> {
    let mut out = HashMap::new();
    if ids.is_empty() {
        return Ok(out);
    }
    // Placeholders come from the count only; ids are bound as parameters.
    let marks = vec!["?"; ids.len()].join(",");
    let mut stmt = conn.prepare(&format!("SELECT id, title, year, source_name, doi, doc_type FROM papers WHERE id IN ({marks})"))?;
    let rows = stmt.query_map(params_from_iter(ids.iter()), |r| {
        Ok(Work { id: r.get(0)?, title: r.get(1)?, year: r.get(2)?, source: r.get(3)?, doi: r.get(4)?, doc_type: r.get(5)?, authors: Vec::new() })
    })?;
    for w in rows {
        let w = w?;
        out.insert(w.id, w);
    }
    let mut stmt = conn.prepare(&format!("SELECT paper_id, name FROM paper_authors WHERE paper_id IN ({marks}) ORDER BY paper_id, position"))?;
    let mut rows = stmt.query(params_from_iter(ids.iter()))?;
    while let Some(r) = rows.next()? {
        if let Some(w) = out.get_mut(&r.get::<_, i64>(0)?) {
            w.authors.push(r.get(1)?);
        }
    }
    Ok(out)
}

pub use super::names::surname;
use super::names;

fn year_text(w: &Work) -> String {
    w.year.map_or_else(|| "t.t.".to_string(), |y| y.to_string())
}

/// APA in-text citation without parentheses: "Rahman, 2023", "Rahman & Chen, 2023", "Rahman et al., 2023".
pub fn apa_in_text(w: &Work) -> String {
    let names: Vec<String> = w.authors.iter().map(|a| surname(a)).collect();
    let who = match names.as_slice() {
        [] => w.title.split_whitespace().take(3).collect::<Vec<_>>().join(" "),
        [a] => a.clone(),
        [a, b] => format!("{a} & {b}"),
        [a, ..] => format!("{a} et al."),
    };
    format!("{who}, {}", year_text(w))
}

/// APA 7 author list: up to 20 names, otherwise the first 19, an ellipsis and the last.
fn author_list(w: &Work) -> String {
    let list: Vec<String> = w.authors.iter().map(|a| names::apa(a)).collect();
    match list.as_slice() {
        [] => String::new(),
        [a] => a.clone(),
        _ if list.len() > 20 => format!("{}, ... {}", list[..19].join(", "), list[list.len() - 1]),
        _ => format!("{}, & {}", list[..list.len() - 1].join(", "), list[list.len() - 1]),
    }
}

fn doi_url(w: &Work) -> String {
    w.doi.as_ref().map(|d| format!(" https://doi.org/{d}")).unwrap_or_default()
}

/// Reference list entry. IEEE entries are numbered by the caller.
pub fn reference(w: &Work, style: Style) -> String {
    let source = w.source.as_deref().map(|s| format!(" {s}.")).unwrap_or_default();
    match style {
        Style::Apa => {
            let authors = author_list(w);
            let lead = if authors.is_empty() { w.title.clone() } else { format!("{authors} ({}). {}", year_text(w), w.title) };
            let lead = if authors.is_empty() { format!("{lead} ({}).", year_text(w)) } else { format!("{lead}.") };
            format!("{lead}{source}{}", doi_url(w))
        }
        Style::Ieee => {
            let list: Vec<String> = w.authors.iter().map(|a| names::ieee(a)).collect();
            let authors = if list.len() > 6 { format!("{} et al.", list[0]) } else { list.join(", ") };
            let doi = w.doi.as_ref().map(|d| format!(", doi: {d}")).unwrap_or_default();
            format!("{authors}, \"{},\"{}{} {}{doi}.", w.title, source.trim_end_matches('.'), if source.is_empty() { "" } else { "," }, year_text(w))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn work(authors: &[&str]) -> Work {
        Work { id: 1, title: "Deep learning for TB".into(), year: Some(2023), source: Some("PLOS ONE".into()), doi: Some("10.1/x".into()), doc_type: None, authors: authors.iter().map(|a| a.to_string()).collect() }
    }

    #[test]
    fn formats_apa_and_ieee() {
        assert_eq!(apa_in_text(&work(&["Rahman, M."])), "Rahman, 2023");
        assert_eq!(apa_in_text(&work(&["Rahman, M.", "Chen, L."])), "Rahman & Chen, 2023");
        assert_eq!(apa_in_text(&work(&["Rahman, M.", "Chen, L.", "Okafor, C."])), "Rahman et al., 2023");
        assert_eq!(reference(&work(&["Rahman, M.", "Chen, L."]), Style::Apa), "Rahman, M., & Chen, L. (2023). Deep learning for TB. PLOS ONE. https://doi.org/10.1/x");
        assert_eq!(reference(&work(&["Rahman, M."]), Style::Ieee), "M. Rahman, \"Deep learning for TB,\" PLOS ONE, 2023, doi: 10.1/x.");
    }
}
