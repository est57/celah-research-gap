//! BibTeX and RIS export of cited works.

use super::cite::{surname, Work};
use std::collections::HashSet;

/// Escapes characters that BibTeX or LaTeX treat as commands, and drops control
/// characters, so imported metadata cannot break or inject into the .bib file.
fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars().filter(|c| !c.is_control()) {
        match c {
            '\\' => out.push_str("\\textbackslash{}"),
            '{' | '}' | '&' | '%' | '$' | '#' | '_' => {
                out.push('\\');
                out.push(c);
            }
            '~' => out.push_str("\\textasciitilde{}"),
            '^' => out.push_str("\\textasciicircum{}"),
            '@' => out.push_str("{@}"),
            _ => out.push(c),
        }
    }
    out
}

fn ascii_word(text: &str) -> String {
    text.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase()
}

/// Citation key "surname2023word", made unique with a, b, c... suffixes.
fn key(w: &Work, used: &mut HashSet<String>) -> String {
    let who = w.authors.first().map(|a| ascii_word(&surname(a))).filter(|s| !s.is_empty()).unwrap_or_else(|| "anon".into());
    let word = w.title.split_whitespace().map(ascii_word).find(|t| t.len() >= 4).unwrap_or_default();
    let base = format!("{who}{}{word}", w.year.map(|y| y.to_string()).unwrap_or_default());
    let mut candidate = base.clone();
    let mut suffix = b'a';
    while !used.insert(candidate.clone()) {
        candidate = format!("{base}{}", suffix as char);
        suffix = suffix.saturating_add(1);
    }
    candidate
}

fn entry_type(w: &Work) -> &'static str {
    match w.doc_type.as_deref() {
        Some("proceedings") => "inproceedings",
        Some("book-chapter") => "incollection",
        Some("preprint") => "misc",
        _ => "article",
    }
}

pub fn bibtex(works: &[Work]) -> String {
    let mut used = HashSet::new();
    let mut out = String::new();
    for w in works {
        let venue_field = match entry_type(w) {
            "inproceedings" | "incollection" => "booktitle",
            "misc" => "howpublished",
            _ => "journal",
        };
        out.push_str(&format!("@{}{{{},\n", entry_type(w), key(w, &mut used)));
        if !w.authors.is_empty() {
            let names: Vec<String> = w.authors.iter().map(|a| escape(a)).collect();
            out.push_str(&format!("  author = {{{}}},\n", names.join(" and ")));
        }
        out.push_str(&format!("  title = {{{{{}}}}},\n", escape(&w.title)));
        if let Some(s) = &w.source {
            out.push_str(&format!("  {venue_field} = {{{}}},\n", escape(s)));
        }
        if let Some(y) = w.year {
            out.push_str(&format!("  year = {{{y}}},\n"));
        }
        if let Some(d) = &w.doi {
            out.push_str(&format!("  doi = {{{}}},\n", escape(d)));
        }
        out.push_str("}\n\n");
    }
    out
}

/// RIS (Zotero, Mendeley, EndNote). One tag per line, so newlines are removed from values.
pub fn ris(works: &[Work]) -> String {
    let clean = |s: &str| s.chars().filter(|c| !c.is_control()).collect::<String>();
    let mut out = String::new();
    for w in works {
        let ty = match w.doc_type.as_deref() {
            Some("proceedings") => "CPAPER",
            Some("book-chapter") => "CHAP",
            _ => "JOUR",
        };
        out.push_str(&format!("TY  - {ty}\n"));
        for a in &w.authors {
            out.push_str(&format!("AU  - {}\n", clean(a)));
        }
        out.push_str(&format!("TI  - {}\n", clean(&w.title)));
        if let Some(s) = &w.source {
            out.push_str(&format!("T2  - {}\n", clean(s)));
        }
        if let Some(y) = w.year {
            out.push_str(&format!("PY  - {y}\n"));
        }
        if let Some(d) = &w.doi {
            out.push_str(&format!("DO  - {}\n", clean(d)));
        }
        out.push_str("ER  - \n\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn work(title: &str, authors: &[&str]) -> Work {
        Work { id: 1, title: title.into(), year: Some(2023), source: Some("PLOS ONE".into()), doi: Some("10.1/x_y".into()), doc_type: None, authors: authors.iter().map(|a| a.to_string()).collect() }
    }

    #[test]
    fn escapes_and_keys_uniquely() {
        let works = vec![work("Deep learning & TB: 100% recall}", &["Rahman, M."]), work("Deep learning again", &["Rahman, M."])];
        let bib = bibtex(&works);
        assert!(bib.contains("@article{rahman2023deep,"));
        assert!(bib.contains("@article{rahman2023deepa,"));
        assert!(bib.contains("Deep learning \\& TB: 100\\% recall\\}"));
        assert!(bib.contains("doi = {10.1/x\\_y}"));
        assert_eq!(bib.matches("@article").count(), 2, "an injected '}}' cannot close the entry early");
    }

    #[test]
    fn writes_ris_lines() {
        let r = ris(&[work("Line\nbreak", &["Rahman, M.", "Chen, L."])]);
        assert!(r.starts_with("TY  - JOUR\nAU  - Rahman, M.\nAU  - Chen, L.\nTI  - Linebreak\n"));
        assert!(r.contains("ER  - "));
    }
}
