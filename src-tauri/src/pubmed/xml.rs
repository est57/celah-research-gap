//! PubMed efetch XML (PubmedArticleSet) to paper records.

use crate::db::papers::NewPaper;
use crate::geo;
use crate::import::limits::{MAX_ABSTRACT, MAX_FIELD, MAX_REFS, MAX_REF_LEN, MAX_TITLE};
use crate::text::{authors_display, clean, clean_opt, join_list, normalize_doc_type, normalize_doi};
use roxmltree::{Document, Node, ParsingOptions};

type El<'a, 'i> = Node<'a, 'i>;

fn child<'a, 'i>(n: El<'a, 'i>, name: &str) -> Option<El<'a, 'i>> {
    n.children().find(|c| c.has_tag_name(name))
}

fn path<'a, 'i>(n: El<'a, 'i>, names: &[&str]) -> Option<El<'a, 'i>> {
    names.iter().try_fold(n, |cur, name| child(cur, name))
}

/// Text of a node including inline markup such as <i> or <sup>.
fn text(n: El) -> String {
    n.descendants().filter(|d| d.is_text()).filter_map(|d| d.text()).collect()
}

fn texts<'a>(n: Option<El<'a, '_>>, item: &str, max_items: usize, max_len: usize) -> Vec<String> {
    n.map(|n| {
        n.descendants()
            .filter(|d| d.has_tag_name(item))
            .map(|d| clean(&text(d), max_len))
            .filter(|s| !s.is_empty())
            .take(max_items)
            .collect()
    })
    .unwrap_or_default()
}

fn author_name(a: El) -> Option<String> {
    let get = |name| child(a, name).map(text);
    let name = match (get("LastName"), get("ForeName").or_else(|| get("Initials")), get("CollectiveName")) {
        (Some(l), Some(f), _) => format!("{l}, {f}"),
        (Some(l), None, _) => l,
        (None, _, Some(c)) => c,
        _ => return None,
    };
    Some(clean(&name, 120)).filter(|n| !n.is_empty())
}

fn doc_type(types: &[String]) -> Option<String> {
    let has = |t: &str| types.iter().any(|x| x.eq_ignore_ascii_case(t));
    if has("Review") || has("Systematic Review") {
        return Some("review".into());
    }
    if has("Preprint") {
        return Some("preprint".into());
    }
    types.iter().find_map(|t| normalize_doc_type(t))
}

fn to_paper(article: El) -> Option<NewPaper> {
    let citation = child(article, "MedlineCitation")?;
    let pmid = clean(&text(child(citation, "PMID")?), 20);
    if pmid.is_empty() || !pmid.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let art = child(citation, "Article")?;
    let title = clean_opt(child(art, "ArticleTitle").map(text).as_deref(), MAX_TITLE)?;
    let authors: Vec<El> = child(art, "AuthorList").map(|l| l.children().filter(|c| c.has_tag_name("Author")).collect()).unwrap_or_default();
    let names: Vec<String> = authors.iter().filter_map(|a| author_name(*a)).take(2_000).collect();
    let doi = article
        .descendants()
        .filter(|d| (d.has_tag_name("ArticleId") && d.attribute("IdType") == Some("doi"))
            || (d.has_tag_name("ELocationID") && d.attribute("EIdType") == Some("doi")))
        .find_map(|d| normalize_doi(&text(d)));
    let abstract_text = texts(child(art, "Abstract"), "AbstractText", 50, MAX_ABSTRACT).join(" ");
    let year = path(art, &["Journal", "JournalIssue", "PubDate"])
        .and_then(|d| child(d, "Year").or_else(|| child(d, "MedlineDate")))
        .and_then(|y| text(y).get(..4).and_then(|s| s.parse().ok()))
        .filter(|y: &i32| (1500..=2100).contains(y));
    let keywords = texts(child(citation, "KeywordList"), "Keyword", 100, 80);
    let mesh = texts(child(citation, "MeshHeadingList"), "DescriptorName", 100, 80);
    let affiliations: Vec<String> = authors
        .iter()
        .flat_map(|a| a.descendants().filter(|d| d.has_tag_name("Affiliation")).map(text))
        .collect();
    let countries = geo::countries_from(affiliations.iter().map(String::as_str));
    let institutions = geo::affil::collect(affiliations.iter().map(|a| geo::affil::from_text(a, false)));
    // PubMed does not flag the corresponding author; the first author's country is used.
    let first_author_country = authors
        .first()
        .and_then(|a| a.descendants().filter(|d| d.has_tag_name("Affiliation")).find_map(|d| geo::country_of(&text(d))))
        .map(str::to_string);
    let refs: Vec<String> = article
        .descendants()
        .filter(|d| d.has_tag_name("Citation"))
        .map(|d| clean(&text(d), MAX_REF_LEN))
        .filter(|s| !s.is_empty())
        .take(MAX_REFS)
        .collect();
    Some(NewPaper {
        ext_id: format!("pmid:{pmid}"),
        doi,
        title,
        year,
        source_name: clean_opt(path(art, &["Journal", "Title"]).map(text).as_deref(), MAX_FIELD),
        doc_type: doc_type(&texts(child(art, "PublicationTypeList"), "PublicationType", 20, 80)),
        authors: authors_display(&names),
        author_count: names.len() as i64,
        author_names: names.clone(),
        affiliations: institutions,
        has_affiliation: !affiliations.is_empty(),
        corresponding_country: first_author_country.or_else(|| countries.first().cloned()),
        countries,
        abstract_text: clean_opt(Some(&abstract_text), MAX_ABSTRACT),
        keywords: join_list(&keywords, MAX_FIELD),
        keywords_plus: join_list(&mesh, MAX_FIELD),
        ref_count: refs.len() as i64,
        refs,
        ..Default::default()
    })
}

/// The DTD declaration is allowed but never fetched: no entity resolver is set,
/// so external entities stay unresolved, and roxmltree caps entity expansion.
/// Node count is capped as well.
pub fn parse(xml: &str) -> Vec<NewPaper> {
    let opts = ParsingOptions { allow_dtd: true, nodes_limit: 5_000_000, ..Default::default() };
    let Ok(doc) = Document::parse_with_options(xml, opts) else { return Vec::new() };
    doc.root_element()
        .children()
        .filter(|n| n.has_tag_name("PubmedArticle"))
        .filter_map(to_paper)
        .collect()
}

#[cfg(test)]
#[path = "xml_tests.rs"]
mod tests;
