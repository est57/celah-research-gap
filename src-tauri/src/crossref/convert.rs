use super::model::{Author, Item, Reference};
use crate::db::papers::NewPaper;
use crate::geo;
use crate::import::limits::{MAX_ABSTRACT, MAX_FIELD, MAX_REFS, MAX_REF_LEN, MAX_TITLE};
use crate::text::{authors_display, clean, clean_opt, normalize_doc_type, normalize_doi, strip_tags};

fn author_name(a: &Author) -> Option<String> {
    let name = match (&a.family, &a.given, &a.name) {
        (Some(f), Some(g), _) => format!("{f}, {g}"),
        (Some(f), None, _) => f.clone(),
        (None, _, Some(n)) => n.clone(),
        _ => return None,
    };
    Some(clean(&name, 120)).filter(|n| !n.is_empty())
}

/// Crossref references are either free text or structured parts.
/// Structured ones are rendered close to the WoS "AUTHOR, YEAR, SOURCE, DOI" style.
fn reference_text(r: &Reference) -> Option<String> {
    if let Some(u) = &r.unstructured {
        return Some(clean(u, MAX_REF_LEN)).filter(|s| !s.is_empty());
    }
    let parts: Vec<&str> = [&r.author, &r.year, &r.journal_title, &r.article_title]
        .into_iter()
        .filter_map(|p| p.as_deref())
        .collect();
    let mut text = parts.join(", ");
    if let Some(doi) = &r.doi {
        text.push_str(&format!(", DOI {doi}"));
    }
    Some(clean(&text, MAX_REF_LEN)).filter(|s| !s.is_empty())
}

pub fn to_paper(item: Item) -> Option<NewPaper> {
    let doi = item.doi.as_deref().and_then(normalize_doi)?;
    let title = clean_opt(item.title.as_ref()?.first().map(String::as_str), MAX_TITLE)?;
    let authors = item.author.unwrap_or_default();
    let names: Vec<String> = authors.iter().filter_map(author_name).take(2_000).collect();
    let refs: Vec<String> = item.reference.unwrap_or_default().iter().filter_map(reference_text).take(MAX_REFS).collect();
    let year = item
        .issued
        .and_then(|i| i.date_parts)
        .and_then(|d| d.first().and_then(|p| p.first().copied().flatten()))
        .and_then(|y| i32::try_from(y).ok())
        .filter(|y| (1500..=2100).contains(y));
    let abstract_text = item.abstract_text.as_deref().map(strip_tags);
    // Crossref affiliations are {"name": "..."} objects, often without a country.
    let affiliation_names = |a: &Author| -> Vec<String> {
        a.affiliation.iter().flatten().filter_map(|x| x.get("name")?.as_str().map(str::to_string)).collect()
    };
    let per_author: Vec<Vec<String>> = authors.iter().map(affiliation_names).collect();
    let countries = geo::countries_from(per_author.iter().flatten().map(String::as_str));
    let first_author_country = per_author.first().and_then(|a| a.iter().find_map(|t| geo::country_of(t))).map(str::to_string);
    let affiliations = geo::affil::collect(per_author.iter().flatten().map(|a| geo::affil::from_text(a, true)));
    Some(NewPaper {
        ext_id: format!("doi:{doi}"),
        affiliations,
        doi: Some(doi),
        title,
        year,
        source_name: clean_opt(item.container_title.as_ref().and_then(|c| c.first()).map(String::as_str), MAX_FIELD),
        doc_type: item.work_type.as_deref().and_then(normalize_doc_type),
        cited_by: item.is_referenced_by_count.unwrap_or(0).max(0),
        authors: authors_display(&names),
        author_count: names.len() as i64,
        author_names: names.clone(),
        has_affiliation: authors.iter().any(|a| a.affiliation.as_ref().is_some_and(|v| !v.is_empty())),
        corresponding_country: first_author_country.or_else(|| countries.first().cloned()),
        countries,
        abstract_text: clean_opt(abstract_text.as_deref(), MAX_ABSTRACT),
        ref_count: item.references_count.unwrap_or(refs.len() as i64).max(0),
        refs,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::super::model::Page;

    #[test]
    fn maps_works_items() {
        let json = r#"{"status":"ok","message":{"items":[{"DOI":"10.1016/J.TUBE.2022.1",
            "type":"journal-article","is-referenced-by-count":73,"title":["Deep learning for TB"],
            "author":[{"given":"Ahmed","family":"Iqbal","affiliation":[]},{"name":"WHO"}],
            "container-title":["Tuberculosis"],"issued":{"date-parts":[[2022,5]]},
            "abstract":"<jats:p>Chest X-ray &amp; AI</jats:p>","references-count":2,
            "reference":[{"key":"b1","author":"Pai","year":"2016","journal-title":"Nat Rev Dis Prim"},
            {"key":"b2","unstructured":"WHO. Screening guidelines. 2021."}]},
            {"DOI":null,"title":["No doi"]}]}}"#;
        let page: Page = serde_json::from_str(json).unwrap();
        let papers: Vec<_> = page.message.items.unwrap().into_iter().filter_map(super::to_paper).collect();
        assert_eq!(papers.len(), 1);
        let p = &papers[0];
        assert_eq!(p.ext_id, "doi:10.1016/j.tube.2022.1");
        assert_eq!(p.authors, "Iqbal, Ahmed; WHO");
        assert_eq!(p.year, Some(2022));
        assert_eq!(p.abstract_text.as_deref(), Some("Chest X-ray & AI"));
        assert_eq!(p.refs, vec!["Pai, 2016, Nat Rev Dis Prim", "WHO. Screening guidelines. 2021."]);
        assert_eq!(p.doc_type.as_deref(), Some("article"));
    }
}
