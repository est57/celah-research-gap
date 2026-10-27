//! Matching free-text references to documents of the collection: by DOI, or by
//! first-author surname and year confirmed by the title (Scopus style) or by the
//! abbreviated source (WoS style, "J DIGIT IMAGING" for "Journal of Digital Imaging").

use crate::analysis::refkey::ref_key;
use std::collections::HashMap;

pub struct Doc {
    pub id: i64,
    pub title_words: Vec<String>,
    pub source_words: Vec<String>,
}

const STOP: [&str; 9] = ["of", "the", "and", "in", "for", "on", "to", "a", "an"];
/// Share of a title's long words that must appear in a reference that quotes the title.
const TITLE_SHARE: f64 = 0.6;

pub fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
}

/// DOIs written anywhere in a reference ("DOI 10.1038/x", "https://doi.org/10.1/y."), lowercased.
pub fn dois(raw: &str) -> Vec<String> {
    let lower = raw.to_lowercase();
    let mut out = Vec::new();
    let mut rest = lower.as_str();
    while let Some(i) = rest.find("10.") {
        let tail = &rest[i..];
        let end = tail.find(|c: char| c.is_whitespace() || matches!(c, ',' | ';' | ')' | ']' | '"' | '>')).unwrap_or(tail.len());
        let doi = tail[..end].trim_end_matches('.');
        let prefix_ok = doi.split_once('/').is_some_and(|(p, s)| p.len() >= 7 && p[3..].chars().all(|c| c.is_ascii_digit() || c == '.') && !s.is_empty());
        if prefix_ok {
            out.push(doi.to_string());
        }
        rest = &tail[end.max(3)..];
    }
    out
}

/// (surname, year) of a reference, from the same parser used for co-citation keys.
pub fn author_year(raw: &str) -> Option<(String, String)> {
    let (key, _) = ref_key(raw)?;
    let mut parts = key.split(' ');
    Some((parts.next()?.to_string(), parts.next()?.to_string()))
}

fn title_quoted(raw_words: &[String], title: &[String]) -> bool {
    let long: Vec<&String> = title.iter().filter(|w| w.chars().count() >= 4).collect();
    if long.len() < 2 {
        return false;
    }
    let found = long.iter().filter(|w| raw_words.contains(w)).count();
    found as f64 / long.len() as f64 >= TITLE_SHARE
}

/// Every abbreviated token, in order, starts a significant word of the source name.
fn source_abbreviated(raw: &str, source: &[String]) -> bool {
    let Some(abbr) = raw.split(',').nth(2) else { return false };
    let tokens = words(abbr);
    let full: Vec<&String> = source.iter().filter(|w| !STOP.contains(&w.as_str())).collect();
    !tokens.is_empty() && tokens.len() <= full.len() && tokens.iter().zip(&full).all(|(t, w)| w.starts_with(t.as_str()))
}

/// The document a reference points to when exactly one candidate shares its surname and year.
pub fn by_author_year<'a>(raw: &str, index: &'a HashMap<(String, String), Vec<Doc>>) -> Option<&'a Doc> {
    let key = author_year(raw)?;
    match index.get(&key)?.as_slice() {
        [doc] => {
            let raw_words = words(raw);
            (title_quoted(&raw_words, &doc.title_words) || source_abbreviated(raw, &doc.source_words)).then_some(doc)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(title: &str, source: &str) -> Doc {
        Doc { id: 7, title_words: words(title), source_words: words(source) }
    }

    #[test]
    fn finds_dois_and_confirms_author_year_matches() {
        assert_eq!(dois("Qin ZZ, 2019, SCI REP, V9, DOI 10.1038/s41598-019-51503-3."), vec!["10.1038/s41598-019-51503-3"]);
        assert_eq!(dois("see https://doi.org/10.1016/J.IJTB.2020.01.003, p. 4"), vec!["10.1016/j.ijtb.2020.01.003"]);
        assert!(dois("Volume 10.5 pages").is_empty());

        let mut index = HashMap::new();
        index.insert(("qin".into(), "2019".into()), vec![doc("Using artificial intelligence to read chest radiographs for tuberculosis detection", "Scientific Reports")]);
        assert!(by_author_year("QIN ZZ, 2019, SCI REP, V9, P15000", &index).is_some(), "WoS style via the source");
        assert!(by_author_year("Qin, Z.Z., Using artificial intelligence to read chest radiographs for tuberculosis detection, (2019)", &index).is_some());
        assert!(by_author_year("QIN ZZ, 2019, LANCET DIGIT HEALTH, V1, P1", &index).is_none(), "same author and year, other work");
        assert!(by_author_year("WANG X, 2019, SCI REP, V9", &index).is_none());
    }
}
