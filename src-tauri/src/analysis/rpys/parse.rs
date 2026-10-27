//! Publication year, counting key and label of one free-text reference.

use super::super::historiograph::refmatch::dois;
use super::super::refkey::{find_year, ref_key};

/// The reference without its DOIs and DOI markers, so digits inside a DOI
/// ("10.1016/j.ijtb.2020.01.003") are never read as a year or an author.
fn without_dois(raw: &str, found: &[String]) -> String {
    let mut text = raw.to_lowercase();
    for d in found {
        text = text.replace(d.as_str(), " ");
    }
    for marker in ["https://doi.org/", "http://dx.doi.org/", "doi:", "doi"] {
        text = text.replace(marker, " ");
    }
    text
}

pub struct Parsed {
    pub year: u16,
    /// "doi:..." when the reference has a DOI, else the surname-year key.
    pub key: String,
    /// Surname-year key of a reference with a DOI, so the same work written
    /// without its DOI elsewhere can be counted with it.
    pub alias: Option<String>,
    pub label: String,
}

/// None when no year is written outside a DOI.
pub fn reference(raw: &str, newest: u16) -> Option<Parsed> {
    let found = dois(raw);
    let text = without_dois(raw, &found);
    let year = find_year(&text).filter(|&y| y <= newest)?;
    let parsed = ref_key(&text);
    let label = match (&parsed, found.first()) {
        (Some((_, l)), _) => l.clone(),
        (None, Some(d)) => format!("{year} · {d}"),
        (None, None) => year.to_string(),
    };
    let (key, alias) = match (found.first(), parsed) {
        (Some(d), p) => (format!("doi:{d}"), p.map(|(k, _)| k)),
        (None, Some((k, _))) => (k, None),
        (None, None) => (text.split_whitespace().collect::<Vec<_>>().join(" "), None),
    };
    Some(Parsed { year, key, alias, label })
}
