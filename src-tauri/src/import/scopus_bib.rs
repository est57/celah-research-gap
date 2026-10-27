//! Scopus "BibTeX" export mapped onto the app's paper record.

use super::bibtex::{self, Entry};
use super::limits::{MAX_ABSTRACT, MAX_FIELD, MAX_RECORDS, MAX_REFS, MAX_REF_LEN, MAX_TITLE};
use crate::db::insert::title_key;
use crate::db::papers::NewPaper;
use crate::geo::{self, affil};
use crate::text::{authors_display, clean, clean_opt, join_list, normalize_doc_type, normalize_doi, split_list};

pub fn parse(text: &str) -> Vec<NewPaper> {
    bibtex::parse(text, MAX_RECORDS).iter().filter_map(to_paper).collect()
}

/// Scopus puts the record id in the link: "...record.uri?eid=2-s2.0-85...&...".
fn eid_from_url(url: &str) -> Option<String> {
    let start = url.find("eid=")? + 4;
    let eid: String = url[start..].chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '.').collect();
    (!eid.is_empty() && eid.len() <= 64).then_some(eid)
}

/// "Cited by: 12; All Open Access, Green Open Access" -> 12
fn cited_by(note: &str) -> Option<i64> {
    let rest = &note[note.find("Cited by:")? + 9..];
    rest.trim_start().split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()
}

fn to_paper(e: &Entry) -> Option<NewPaper> {
    let get = |k: &str| e.fields.get(k).map(String::as_str).filter(|v| !v.trim().is_empty());
    let title = clean_opt(get("title"), MAX_TITLE)?;
    let year: Option<i32> = get("year").and_then(|y| y.trim().parse().ok()).filter(|y| (1500..=2100).contains(y));
    let doi = get("doi").and_then(normalize_doi);
    let ext_id = match (get("url").and_then(eid_from_url), &doi) {
        (Some(eid), _) => format!("scopus:{eid}"),
        (None, Some(d)) => format!("doi:{d}"),
        (None, None) => format!("title:{}:{}", title_key(&title), year.unwrap_or(0)),
    };
    let names: Vec<String> = get("author")
        .map(|a| a.split(" and ").map(|n| clean(n, 120)).filter(|n| !n.is_empty()).take(2_000).collect())
        .unwrap_or_default();
    let refs = get("references").map(|r| split_list(r, ';', MAX_REFS, MAX_REF_LEN)).unwrap_or_default();
    let keywords = |k: &str| get(k).and_then(|v| join_list(&split_list(v, ';', 100, 80), MAX_FIELD));
    let (countries, corresponding_country) = geo::from_list(get("affiliations"), get("correspondence_address"));
    let affiliations = affil::collect(get("affiliations").unwrap_or("").split(';').map(|a| affil::from_text(a, false)));
    Some(NewPaper {
        ext_id,
        doi,
        title,
        year,
        source_name: clean_opt(get("journal").or(get("booktitle")), MAX_FIELD),
        doc_type: get("type").or(get("document_type")).or(Some(e.kind.as_str())).and_then(normalize_doc_type),
        cited_by: get("note").and_then(cited_by).unwrap_or(0).max(0),
        authors: authors_display(&names),
        author_count: names.len() as i64,
        author_names: names.clone(),
        has_affiliation: get("affiliations").is_some(),
        countries,
        corresponding_country,
        abstract_text: clean_opt(get("abstract"), MAX_ABSTRACT),
        keywords: keywords("author_keywords"),
        keywords_plus: keywords("keywords"),
        ref_count: refs.len() as i64,
        cites: Vec::new(),
        affiliations,
        refs,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn maps_scopus_fields() {
        let src = "@ARTICLE{Okafor2024,\n author = {Okafor, C. and Silva, R. and Chen, L.},\n\
            title = {Cough audio classification},\n year = {2024},\n journal = {IEEE Access},\n\
            doi = {10.1109/ACCESS.2024.1},\n url = {https://www.scopus.com/inward/record.uri?eid=2-s2.0-85000000001&partnerID=40},\n\
            author_keywords = {cough; audio},\n references = {Botha G., 2018; Pahar M., 2021},\n\
            type = {Article},\n note = {Cited by: 18; All Open Access}\n}";
        let p = &super::parse(src)[0];
        assert_eq!(p.ext_id, "scopus:2-s2.0-85000000001");
        assert_eq!(p.authors, "Okafor, C.; Silva, R.; +1");
        assert_eq!(p.cited_by, 18);
        assert_eq!(p.refs.len(), 2);
        assert_eq!(p.doc_type.as_deref(), Some("article"));
    }
}
