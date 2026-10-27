use super::model::{Author, Record};
use crate::db::papers::NewPaper;
use crate::geo;
use crate::import::limits::{MAX_ABSTRACT, MAX_FIELD, MAX_TITLE};
use crate::text::{authors_display, clean, clean_opt, join_list, normalize_doc_type, normalize_doi, strip_tags};

fn is_token(s: &str, max: usize) -> bool {
    !s.is_empty() && s.len() <= max && s.chars().all(|c| c.is_ascii_alphanumeric())
}

/// PubMed records share the "pmid:" id with the PubMed connector, so the
/// same article found through both sources is stored once.
fn ext_id(r: &Record) -> Option<String> {
    if let Some(pmid) = r.pmid.as_deref().filter(|p| is_token(p, 12) && p.chars().all(|c| c.is_ascii_digit())) {
        return Some(format!("pmid:{pmid}"));
    }
    let (source, id) = (r.source.as_deref()?, r.id.as_deref()?);
    (is_token(source, 8) && is_token(id, 32)).then(|| format!("epmc:{source}:{id}"))
}

fn author_name(a: &Author) -> Option<String> {
    let name = match (&a.last_name, &a.first_name, &a.full_name) {
        (Some(l), Some(f), _) => format!("{l}, {f}"),
        (_, _, Some(full)) => full.clone(),
        (Some(l), None, None) => l.clone(),
        _ => return None,
    };
    Some(clean(&name, 120)).filter(|n| !n.is_empty())
}

fn doc_type(types: &[String], source: Option<&str>) -> Option<String> {
    let lower: Vec<String> = types.iter().map(|t| t.to_lowercase()).collect();
    if lower.iter().any(|t| t.contains("review")) {
        return Some("review".into());
    }
    if source == Some("PPR") || lower.iter().any(|t| t.contains("preprint")) {
        return Some("preprint".into());
    }
    lower.iter().find_map(|t| normalize_doc_type(t))
}

/// Affiliation strings from {"authorAffiliation": [{"affiliation": "..."}]}.
fn affiliations(a: &Author) -> Vec<String> {
    a.author_affiliation_details_list
        .as_ref()
        .and_then(|v| v.get("authorAffiliation"))
        .and_then(|v| v.as_array())
        .map(|list| list.iter().filter_map(|x| x.get("affiliation")?.as_str().map(str::to_string)).collect())
        .unwrap_or_default()
}

pub fn to_paper(r: Record) -> Option<NewPaper> {
    let ext_id = ext_id(&r)?;
    let title = clean_opt(r.title.as_deref(), MAX_TITLE)?;
    let authors = r.author_list.and_then(|l| l.author).unwrap_or_default();
    let names: Vec<String> = authors.iter().filter_map(author_name).take(2_000).collect();
    let keywords: Vec<String> = r.keyword_list.and_then(|k| k.keyword).unwrap_or_default().iter().map(|k| clean(k, 80)).take(100).collect();
    let mesh: Vec<String> = r
        .mesh_heading_list
        .and_then(|m| m.mesh_heading)
        .unwrap_or_default()
        .iter()
        .filter_map(|m| m.descriptor_name.as_deref())
        .map(|d| clean(d, 80))
        .take(100)
        .collect();
    let per_author: Vec<Vec<String>> = authors.iter().map(affiliations).collect();
    let countries = geo::countries_from(per_author.iter().flatten().map(String::as_str));
    // Europe PMC does not flag the corresponding author; the first author's country is used.
    let first_author_country = per_author.first().and_then(|a| a.iter().find_map(|t| geo::country_of(t))).map(str::to_string);
    let affiliations = geo::affil::collect(per_author.iter().flatten().map(|a| geo::affil::from_text(a, false)));
    let types = r.pub_type_list.and_then(|t| t.pub_type).unwrap_or_default();
    let journal = r.journal_info.and_then(|j| j.journal).and_then(|j| j.title);
    Some(NewPaper {
        ext_id,
        doi: r.doi.as_deref().and_then(normalize_doi),
        title,
        year: r.pub_year.as_deref().and_then(|y| y.trim().parse().ok()).filter(|y| (1500..=2100).contains(y)),
        source_name: clean_opt(journal.as_deref(), MAX_FIELD),
        doc_type: doc_type(&types, r.source.as_deref()),
        cited_by: r.cited_by_count.unwrap_or(0).max(0),
        authors: authors_display(&names),
        author_count: names.len() as i64,
        author_names: names.clone(),
        has_affiliation: authors.iter().any(|a| a.author_affiliation_details_list.is_some()),
        affiliations,
        corresponding_country: first_author_country.or_else(|| countries.first().cloned()),
        countries,
        abstract_text: clean_opt(r.abstract_text.as_deref().map(strip_tags).as_deref(), MAX_ABSTRACT),
        keywords: join_list(&keywords, MAX_FIELD),
        keywords_plus: join_list(&mesh, MAX_FIELD),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::super::model::Page;

    #[test]
    fn maps_core_results() {
        let json = r#"{"hitCount":2,"nextCursorMark":"AoII","resultList":{"result":[
            {"id":"42780254","source":"MED","pmid":"42780254","doi":"10.3389/FRAI.2026.1",
             "title":"CNNs in resource-constrained settings","pubYear":"2026","citedByCount":3,
             "abstractText":"<h4>Background</h4>CNNs &amp; imaging.",
             "authorList":{"author":[{"fullName":"Innocent DC","lastName":"Innocent","firstName":"David",
               "authorAffiliationDetailsList":{"authorAffiliation":[{"affiliation":"Centicini Research Lab, Abuja, Nigeria."}]}}]},
             "journalInfo":{"journal":{"title":"Frontiers in AI"}},
             "keywordList":{"keyword":["Deep Learning","CNN"]},
             "pubTypeList":{"pubType":["Systematic Review","Journal Article"]}},
            {"id":"PPR123","source":"PPR","title":"A preprint","pubYear":"2025"}]}}"#;
        let page: Page = serde_json::from_str(json).unwrap();
        let papers: Vec<_> = page.result_list.unwrap().result.unwrap().into_iter().filter_map(super::to_paper).collect();
        assert_eq!(papers.len(), 2);
        let p = &papers[0];
        assert_eq!(p.ext_id, "pmid:42780254");
        assert_eq!(p.authors, "Innocent, David");
        assert!(p.has_affiliation);
        assert_eq!(p.countries, vec!["NG"]);
        assert_eq!(p.abstract_text.as_deref(), Some("Background CNNs & imaging."));
        assert_eq!(p.doc_type.as_deref(), Some("review"));
        assert_eq!(papers[1].ext_id, "epmc:PPR:PPR123");
        assert_eq!(papers[1].doc_type.as_deref(), Some("preprint"));
    }
}
