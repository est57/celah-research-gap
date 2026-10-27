use super::model::Work;
use crate::db::papers::NewPaper;
use crate::geo;
use crate::text::{authors_display, clean, clean_opt, normalize_doc_type, normalize_doi};
use std::collections::HashMap;

const MAX_TITLE: usize = 1_000;
const MAX_ABSTRACT: usize = 20_000;
const MAX_FIELD: usize = 500;
const MAX_ABSTRACT_WORDS: usize = 5_000;

/// OpenAlex ships abstracts as an inverted index (word -> positions).
fn rebuild_abstract(index: &HashMap<String, Vec<usize>>) -> Option<String> {
    let mut words: Vec<(usize, &str)> = index
        .iter()
        .flat_map(|(w, pos)| pos.iter().map(move |p| (*p, w.as_str())))
        .filter(|(p, _)| *p < MAX_ABSTRACT_WORDS)
        .collect();
    if words.is_empty() {
        return None;
    }
    words.sort_unstable_by_key(|(p, _)| *p);
    let text: Vec<&str> = words.into_iter().map(|(_, w)| w).collect();
    clean_opt(Some(&text.join(" ")), MAX_ABSTRACT)
}

/// Cited works kept per paper; reviews can list several hundred.
const MAX_CITES: usize = 1000;

fn short_id(url: &str) -> Option<String> {
    let id = url.rsplit('/').next()?;
    let valid = id.len() <= 32 && id.starts_with('W') && id[1..].chars().all(|c| c.is_ascii_digit());
    valid.then(|| id.to_string())
}

/// Institutions from OpenAlex authorships ({"display_name", "country_code"} objects).
pub fn institutions(authorships: &[super::model::Authorship]) -> Vec<geo::affil::Affiliation> {
    let named = authorships.iter().flat_map(|a| a.institutions.iter()).map(|inst| {
        let name = inst.get("display_name")?.as_str()?;
        geo::affil::from_named(name, inst.get("country_code").and_then(|c| c.as_str()))
    });
    geo::affil::collect(named)
}

fn countries(w: &Work) -> Vec<String> {
    let mut out = Vec::new();
    for code in w.authorships.iter().flat_map(|a| &a.countries).filter_map(|c| geo::normalize_code(c)) {
        geo::push_unique(&mut out, &code);
    }
    out
}

/// The corresponding author's first country, else the first author's.
fn corresponding_country(w: &Work) -> Option<String> {
    let first_code = |a: &super::model::Authorship| a.countries.iter().find_map(|c| geo::normalize_code(c));
    w.authorships
        .iter()
        .find(|a| a.is_corresponding == Some(true))
        .and_then(first_code)
        .or_else(|| w.authorships.first().and_then(first_code))
}

pub fn to_paper(w: Work) -> Option<NewPaper> {
    let ext_id = format!("openalex:{}", short_id(w.id.as_deref()?)?);
    let title = clean_opt(w.display_name.as_deref(), MAX_TITLE)?;
    let names: Vec<String> = w
        .authorships
        .iter()
        .filter_map(|a| a.author.as_ref()?.display_name.as_deref())
        .map(|n| clean(n, 120))
        .collect();
    let authors = authors_display(&names);
    let keywords: Vec<String> = w
        .keywords
        .iter()
        .filter_map(|k| k.display_name.as_deref())
        .map(|k| clean(k, 80))
        .collect();
    Some(NewPaper {
        ext_id,
        doi: w.doi.as_deref().and_then(normalize_doi),
        title,
        year: w.publication_year.filter(|y| (1500..=2100).contains(y)),
        source_name: clean_opt(
            w.primary_location.as_ref().and_then(|l| l.source.as_ref()).and_then(|s| s.display_name.as_deref()),
            MAX_FIELD,
        ),
        doc_type: w.work_type.as_deref().and_then(normalize_doc_type),
        cited_by: w.cited_by_count.max(0),
        author_count: names.len() as i64,
        author_names: names.clone(),
        has_affiliation: w.authorships.iter().any(|a| !a.institutions.is_empty()),
        affiliations: institutions(&w.authorships),
        countries: countries(&w),
        corresponding_country: corresponding_country(&w),
        authors,
        abstract_text: w.abstract_inverted_index.as_ref().and_then(rebuild_abstract),
        keywords: clean_opt(Some(&keywords.join("; ")), MAX_FIELD),
        ref_count: w.referenced_works_count.max(0),
        cites: w.referenced_works.iter().filter_map(|u| short_id(u)).take(MAX_CITES).map(|id| format!("openalex:{id}")).collect(),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_openalex_ids() {
        assert_eq!(short_id("https://openalex.org/W123").as_deref(), Some("W123"));
        assert!(short_id("https://openalex.org/../etc").is_none());
    }

    #[test]
    fn rebuilds_abstract_in_order() {
        let mut idx = HashMap::new();
        idx.insert("world".to_string(), vec![1]);
        idx.insert("hello".to_string(), vec![0, 2]);
        assert_eq!(rebuild_abstract(&idx).as_deref(), Some("hello world hello"));
    }
}
