//! Scopus "CSV" export. Columns are located by header name, so the parser
//! keeps working when the user picks a different column set in Scopus.

use super::limits::{MAX_ABSTRACT, MAX_FIELD, MAX_RECORDS, MAX_REFS, MAX_REF_LEN, MAX_TITLE};
use crate::db::papers::NewPaper;
use crate::geo::{self, affil};
use crate::text::{authors_display, clean_opt, join_list, normalize_doc_type, normalize_doi, split_list};
use std::collections::HashMap;

pub fn parse(text: &str) -> Vec<NewPaper> {
    let mut reader = csv::ReaderBuilder::new().flexible(true).from_reader(text.as_bytes());
    let Ok(headers) = reader.headers() else { return Vec::new() };
    let index: HashMap<String, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| (h.trim_start_matches('\u{feff}').trim().to_lowercase(), i))
        .collect();
    reader
        .records()
        .filter_map(Result::ok)
        .filter_map(|rec| {
            let get = |name: &str| index.get(name).and_then(|i| rec.get(*i)).map(str::trim).filter(|s| !s.is_empty());
            to_paper(&get)
        })
        .take(MAX_RECORDS)
        .collect()
}

fn to_paper<'a>(get: &dyn Fn(&str) -> Option<&'a str>) -> Option<NewPaper> {
    let title = clean_opt(get("title"), MAX_TITLE)?;
    let doi = get("doi").and_then(normalize_doi);
    let ext_id = match (get("eid"), &doi) {
        (Some(eid), _) => format!("scopus:{}", clean_opt(Some(eid), 64)?),
        (None, Some(d)) => format!("doi:{d}"),
        (None, None) => return None,
    };
    let names = get("authors").map(|a| split_list(a, ';', 2_000, 120)).unwrap_or_default();
    let refs = get("references").map(|r| split_list(r, ';', MAX_REFS, MAX_REF_LEN)).unwrap_or_default();
    let keywords = |col: &str| get(col).and_then(|k| join_list(&split_list(k, ';', 100, 80), MAX_FIELD));
    let (countries, corresponding_country) = geo::from_list(get("affiliations"), get("correspondence address"));
    let affiliations = affil::collect(get("affiliations").unwrap_or("").split(';').map(|a| affil::from_text(a, false)));
    let abstract_text = get("abstract")
        .filter(|a| !a.starts_with("[No abstract available]"))
        .and_then(|a| clean_opt(Some(a), MAX_ABSTRACT));
    Some(NewPaper {
        ext_id,
        doi,
        title,
        year: get("year").and_then(|y| y.parse().ok()).filter(|y| (1500..=2100).contains(y)),
        source_name: clean_opt(get("source title"), MAX_FIELD),
        doc_type: get("document type").and_then(normalize_doc_type),
        cited_by: get("cited by").and_then(|c| c.parse().ok()).unwrap_or(0_i64).max(0),
        authors: authors_display(&names),
        author_count: names.len() as i64,
        author_names: names.clone(),
        has_affiliation: get("affiliations").is_some(),
        countries,
        corresponding_country,
        abstract_text,
        keywords: keywords("author keywords"),
        keywords_plus: keywords("index keywords"),
        ref_count: refs.len() as i64,
        cites: Vec::new(),
        affiliations,
        refs,
    })
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "\u{feff}Authors,Title,Year,Source title,Cited by,DOI,Affiliations,Abstract,\
Author Keywords,Index Keywords,References,Document Type,EID
\"Okafor C.; Silva R.\",\"Cough audio classification, a TB triage study\",2024,IEEE Access,18,\
10.1109/ACCESS.2024.1,\"Univ A, Lagos\",[No abstract available],cough; audio,Tuberculosis,\
\"Botha G., 2018, Physiol Meas; Pahar M., 2021, Comput Biol Med\",Article,2-s2.0-85000000001
,,2020,,,,,,,,,,
";

    #[test]
    fn parses_quoted_rows_by_header() {
        let papers = super::parse(SAMPLE);
        assert_eq!(papers.len(), 1, "row without title is skipped");
        let p = &papers[0];
        assert_eq!(p.ext_id, "scopus:2-s2.0-85000000001");
        assert_eq!(p.title, "Cough audio classification, a TB triage study");
        assert_eq!(p.author_count, 2);
        assert_eq!(p.refs.len(), 2);
        assert_eq!(p.abstract_text, None);
        assert_eq!(p.keywords.as_deref(), Some("cough; audio"));
        assert_eq!(p.doc_type.as_deref(), Some("article"));
    }
}
