//! Keywords that are probably one concept: spelling variants ("deep-learning",
//! "neural networks" vs "neural network") and acronyms ("CNN" for
//! "convolutional neural network"), counted over both keyword fields.

use super::key;
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;

const MAX_SUGGESTIONS: usize = 80;
const STOP: [&str; 6] = ["of", "and", "for", "the", "in", "on"];

#[derive(Serialize, Clone)]
pub struct Term {
    pub label: String,
    pub documents: usize,
}

#[derive(Serialize)]
pub struct KeywordSuggestion {
    /// "ejaan" (spelling or plural) or "singkatan" (acronym and expansion).
    pub kind: &'static str,
    /// Most frequent first; the first is the suggested name to keep.
    pub terms: Vec<Term>,
}

fn singular(word: &str) -> String {
    if word.len() > 4 && word.ends_with("ies") {
        format!("{}y", &word[..word.len() - 3])
    } else if word.len() > 3 && word.ends_with('s') && !word.ends_with("ss") && !word.ends_with("us") && !word.ends_with("is") {
        word[..word.len() - 1].to_string()
    } else {
        word.to_string()
    }
}

/// "Deep-Learning", "deep learning" -> "deep learning"; "neural networks" -> "neural network";
/// a parenthesized acronym ("... Network (CNN)") is ignored.
pub fn spelling_form(label: &str) -> String {
    let lower = key(label);
    let lower = match (lower.find('('), lower.rfind(')')) {
        (Some(a), Some(b)) if a < b => format!("{}{}", &lower[..a], &lower[b + 1..]),
        _ => lower,
    };
    lower.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(singular).collect::<Vec<_>>().join(" ")
}

/// "convolutional neural networks" -> "cnn"; None for single words.
fn initials(label: &str) -> Option<String> {
    let words: Vec<String> = spelling_form(label).split(' ').filter(|w| !STOP.contains(w)).map(str::to_string).collect();
    (words.len() >= 2).then(|| words.iter().filter_map(|w| w.chars().next()).collect())
}

/// "CNN", "CNNs" -> "cnn" when written in capitals; None otherwise.
fn acronym(label: &str) -> Option<String> {
    let t = label.trim().trim_end_matches('s');
    (t.len() >= 2 && t.len() <= 6 && t.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())).then(|| t.to_lowercase())
}

/// Documents per keyword key (first label seen), over both fields, skipping mapped keys.
fn counts(conn: &Connection) -> AppResult<HashMap<String, (String, usize)>> {
    let mapped: Vec<String> = super::load(conn)?.into_keys().collect();
    let mut counts: HashMap<String, (String, usize)> = HashMap::new();
    let mut stmt = conn.prepare("SELECT COALESCE(keywords, '') || ';' || COALESCE(keywords_plus, '') FROM main.papers")?;
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let text: String = r.get(0)?;
        let mut seen: Vec<String> = Vec::new();
        for t in text.split(';').map(str::trim).filter(|t| !t.is_empty()) {
            let k = key(t);
            if !seen.contains(&k) && !mapped.contains(&k) {
                counts.entry(k.clone()).or_insert_with(|| (t.to_string(), 0)).1 += 1;
                seen.push(k);
            }
        }
    }
    Ok(counts)
}

/// The most used keywords, for picking synonyms by hand.
pub fn top_terms(conn: &Connection, n: usize) -> AppResult<Vec<Term>> {
    let mut terms: Vec<Term> = counts(conn)?.into_values().map(|(label, documents)| Term { label, documents }).collect();
    terms.sort_by(|a, b| b.documents.cmp(&a.documents).then(a.label.cmp(&b.label)));
    terms.truncate(n);
    Ok(terms)
}

pub fn suggest(conn: &Connection) -> AppResult<Vec<KeywordSuggestion>> {
    let counts = counts(conn)?;
    let mut groups: HashMap<String, Vec<Term>> = HashMap::new();
    for (label, n) in counts.values() {
        groups.entry(spelling_form(label)).or_default().push(Term { label: label.clone(), documents: *n });
    }
    // Spelling forms by initials; an acronym pairs with its most used form only (its usual
    // meaning), together with that form's spelling variants.
    let mut by_initials: HashMap<String, Vec<(&String, usize)>> = HashMap::new();
    for (form, terms) in &groups {
        if let Some(i) = initials(form) {
            by_initials.entry(i).or_default().push((form, terms.iter().map(|t| t.documents).sum()));
        }
    }
    let mut out: Vec<KeywordSuggestion> = Vec::new();
    let mut used: Vec<String> = Vec::new();
    for (label, n) in counts.values() {
        let Some(forms) = acronym(label).and_then(|a| by_initials.get(&a)) else { continue };
        let Some((form, _)) = forms.iter().max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(a.0))) else { continue };
        let mut terms = groups[*form].clone();
        terms.push(Term { label: label.clone(), documents: *n });
        used.push((*form).clone());
        used.push(spelling_form(label));
        out.push(KeywordSuggestion { kind: "singkatan", terms });
    }
    for (form, terms) in groups {
        if terms.len() >= 2 && !used.contains(&form) {
            out.push(KeywordSuggestion { kind: "ejaan", terms });
        }
    }
    for g in &mut out {
        g.terms.sort_by(|a, b| b.documents.cmp(&a.documents).then(a.label.cmp(&b.label)));
    }
    let total = |g: &KeywordSuggestion| g.terms.iter().map(|t| t.documents).sum::<usize>();
    out.sort_by(|a, b| total(b).cmp(&total(a)).then(a.terms[0].label.cmp(&b.terms[0].label)));
    out.truncate(MAX_SUGGESTIONS);
    Ok(out)
}
