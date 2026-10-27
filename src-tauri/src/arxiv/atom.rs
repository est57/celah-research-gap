//! arXiv API Atom feed to paper records.

use crate::db::papers::NewPaper;
use crate::import::limits::{MAX_ABSTRACT, MAX_FIELD, MAX_TITLE};
use crate::text::{authors_display, clean, clean_opt, join_list, normalize_doi};
use roxmltree::{Document, Node, ParsingOptions};

fn child<'a, 'i>(n: Node<'a, 'i>, name: &str) -> Option<Node<'a, 'i>> {
    n.children().find(|c| c.has_tag_name(name))
}

fn child_text(n: Node, name: &str) -> Option<String> {
    child(n, name).and_then(|c| c.text()).map(str::to_string)
}

/// "http://arxiv.org/abs/2009.04160v1" -> "2009.04160"; also old ids like "hep-th/9901001v2".
pub fn canonical_id(url: &str) -> Option<String> {
    let raw = url.split("/abs/").nth(1)?;
    let base = match raw.rfind('v') {
        Some(i) if i > 0 && raw[i + 1..].chars().all(|c| c.is_ascii_digit()) && i + 1 < raw.len() => &raw[..i],
        _ => raw,
    };
    let ok = !base.is_empty()
        && base.len() <= 40
        && base.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '/' | '-'))
        && !base.contains("..");
    ok.then(|| base.to_string())
}

fn to_paper(entry: Node) -> Option<NewPaper> {
    let id = canonical_id(&child_text(entry, "id")?)?;
    let title = clean_opt(child_text(entry, "title").as_deref(), MAX_TITLE)?;
    let authors: Vec<Node> = entry.children().filter(|c| c.has_tag_name("author")).collect();
    let names: Vec<String> = authors
        .iter()
        .filter_map(|a| child_text(*a, "name"))
        .map(|n| clean(&n, 120))
        .filter(|n| !n.is_empty())
        .take(2_000)
        .collect();
    let categories: Vec<String> = entry
        .children()
        .filter(|c| c.has_tag_name("category"))
        .filter_map(|c| c.attribute("term"))
        .map(|t| clean(t, 40))
        .take(20)
        .collect();
    let journal = clean_opt(child_text(entry, "journal_ref").as_deref(), MAX_FIELD);
    Some(NewPaper {
        ext_id: format!("arxiv:{id}"),
        doi: child_text(entry, "doi").and_then(|d| normalize_doi(&d)),
        title,
        year: child_text(entry, "published")
            .and_then(|p| p.get(..4).and_then(|y| y.parse().ok()))
            .filter(|y: &i32| (1990..=2100).contains(y)),
        source_name: journal.or_else(|| Some("arXiv".into())),
        doc_type: Some("preprint".into()),
        authors: authors_display(&names),
        author_count: names.len() as i64,
        author_names: names.clone(),
        has_affiliation: authors.iter().any(|a| child(*a, "affiliation").is_some()),
        abstract_text: clean_opt(child_text(entry, "summary").as_deref(), MAX_ABSTRACT),
        keywords_plus: join_list(&categories, MAX_FIELD),
        ..Default::default()
    })
}

/// Returns the records and the number of <entry> elements seen (for paging).
pub fn parse(xml: &str) -> (Vec<NewPaper>, usize) {
    let opts = ParsingOptions { nodes_limit: 2_000_000, ..Default::default() };
    let Ok(doc) = Document::parse_with_options(xml, opts) else { return (Vec::new(), 0) };
    let entries: Vec<Node> = doc.root_element().children().filter(|n| n.has_tag_name("entry")).collect();
    let count = entries.len();
    // arXiv reports query errors as an entry whose id points at /api/errors.
    let papers = entries
        .into_iter()
        .filter(|e| !child_text(*e, "id").is_some_and(|id| id.contains("/api/errors")))
        .filter_map(to_paper)
        .collect();
    (papers, count)
}

#[cfg(test)]
#[path = "atom_tests.rs"]
mod tests;
