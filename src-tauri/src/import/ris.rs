//! RIS export (Zotero, Mendeley, EndNote, Scopus): "TY  - JOUR" opens a record and
//! "ER  -" closes it. Scopus adds citation counts and references in N1 notes.

use super::limits::{MAX_ABSTRACT, MAX_FIELD, MAX_RECORDS, MAX_REFS, MAX_REF_LEN, MAX_TITLE};
use super::tagged::{self, first, keywords, list, values, Fields};
use crate::db::insert::title_key;
use crate::db::papers::NewPaper;
use crate::geo::{self, affil};
use crate::text::{authors_display, normalize_doc_type, normalize_doi, normalize_issns, split_list};

pub fn parse(text: &str) -> Vec<NewPaper> {
    tagged::records(text, "TY", "ER", MAX_RECORDS).iter().filter_map(to_paper).collect()
}

/// Scopus links carry the record id: "...record.uri?eid=2-s2.0-85...&...".
fn eid(url: &str) -> Option<String> {
    let start = url.find("eid=")? + 4;
    let id: String = url[start..].chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '.').collect();
    (!id.is_empty() && id.len() <= 64).then_some(id)
}

/// The value after "Label:" in a note, e.g. "Cited By: 12" or Scopus's older "Cited By :12".
fn note<'a>(f: &'a Fields, label: &str) -> Option<&'a str> {
    values(f, &["N1"]).find_map(|n| {
        // ASCII lowercasing keeps byte offsets valid for slicing `n`.
        let at = n.to_ascii_lowercase().find(label)?;
        let rest = n.get(at + label.len()..)?.trim_start();
        Some(rest.strip_prefix(':').unwrap_or(rest).trim())
    })
}

/// Scopus puts the finer type in M3 ("Review", "Conference paper"); other tools put free
/// text there, so M3 counts only when it maps onto a known type.
fn doc_type(f: &Fields) -> Option<String> {
    const KNOWN: [&str; 5] = ["article", "review", "proceedings", "book-chapter", "preprint"];
    let finer = first(f, &["M3"], 80).and_then(|t| normalize_doc_type(&t)).filter(|t| KNOWN.contains(&t.as_str()));
    let ty = first(f, &["TY"], 20).unwrap_or_default();
    let mapped = match ty.as_str() {
        "JOUR" | "JFULL" | "EJOUR" | "MGZN" | "NEWS" => "article",
        "CONF" | "CPAPER" => "proceedings",
        "CHAP" => "book-chapter",
        "BOOK" | "EBOOK" | "EDBOOK" => "book",
        "THES" => "thesis",
        "RPRT" => "report",
        _ => return finer.or_else(|| normalize_doc_type(&ty)),
    };
    finer.or(Some(mapped.into()))
}

fn to_paper(f: &Fields) -> Option<NewPaper> {
    let title = first(f, &["TI", "T1"], MAX_TITLE)?;
    let year = values(f, &["PY", "Y1", "DA"]).find_map(tagged::year);
    let urls: Vec<&str> = values(f, &["UR", "L2"]).collect();
    let doi = values(f, &["DO"]).chain(urls.iter().copied().filter(|u| u.contains("doi.org/"))).find_map(normalize_doi);
    let accession = first(f, &["AN"], 64).filter(|a| a.starts_with("WOS:"));
    let ext_id = match (urls.iter().find_map(|u| eid(u)), accession, &doi) {
        (Some(e), _, _) => format!("scopus:{e}"),
        (None, Some(a), _) => format!("wos:{a}"),
        (None, None, Some(d)) => format!("doi:{d}"),
        (None, None, None) => format!("title:{}:{}", title_key(&title), year.unwrap_or(0)),
    };
    let names = list(f, &["AU", "A1"], 2_000, 120);
    let addresses = list(f, &["AD"], 500, 1_000).join("; ");
    let (countries, corresponding_country) = geo::from_list(Some(&addresses), note(f, "correspondence address"));
    let refs = note(f, "references").map(|r| split_list(r, ';', MAX_REFS, MAX_REF_LEN)).unwrap_or_default();
    let cited_by = note(f, "cited by").and_then(|c| c.split(|ch: char| !ch.is_ascii_digit()).next()?.parse().ok());
    Some(NewPaper {
        ext_id,
        doi,
        title,
        year,
        source_name: first(f, &["T2", "JF", "JO", "JA", "J2", "BT"], MAX_FIELD),
        issn: normalize_issns(values(f, &["SN"])),
        doc_type: doc_type(f),
        cited_by: cited_by.unwrap_or(0).max(0),
        authors: authors_display(&names),
        author_count: names.len() as i64,
        author_names: names.clone(),
        has_affiliation: !addresses.is_empty(),
        affiliations: affil::collect(addresses.split(';').map(|a| affil::from_text(a, false))),
        countries,
        corresponding_country,
        abstract_text: first(f, &["AB", "N2"], MAX_ABSTRACT),
        keywords: keywords(values(f, &["KW"])),
        keywords_plus: None,
        ref_count: refs.len() as i64,
        cites: Vec::new(),
        refs,
    })
}

#[cfg(test)]
#[path = "ris_tests.rs"]
mod tests;
