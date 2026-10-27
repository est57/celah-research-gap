use super::model::Paper;
use crate::db::papers::NewPaper;
use crate::text::{authors_display, clean, clean_opt, normalize_doc_type, normalize_doi};

const MAX_TITLE: usize = 1_000;
const MAX_ABSTRACT: usize = 20_000;
const MAX_FIELD: usize = 500;

/// Semantic Scholar paper ids are 40-character hex hashes.
fn valid_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 64 && id.chars().all(|c| c.is_ascii_alphanumeric())
}

fn doc_type(types: &[String]) -> Option<String> {
    let has = |t: &str| types.iter().any(|x| x == t);
    let mapped = if has("Review") {
        "review"
    } else if has("JournalArticle") {
        "article"
    } else if has("Conference") {
        "proceedings"
    } else if has("BookSection") {
        "book-chapter"
    } else {
        return types.first().and_then(|t| normalize_doc_type(t));
    };
    Some(mapped.to_string())
}

pub fn to_paper(p: Paper) -> Option<NewPaper> {
    let id = p.paper_id.filter(|id| valid_id(id))?;
    let title = clean_opt(p.title.as_deref(), MAX_TITLE)?;
    let names: Vec<String> = p
        .authors
        .unwrap_or_default()
        .iter()
        .filter_map(|a| a.name.as_deref())
        .map(|n| clean(n, 120))
        .filter(|n| !n.is_empty())
        .take(2_000)
        .collect();
    let doi = p
        .external_ids
        .as_ref()
        .and_then(|ids| ids.get("DOI"))
        .and_then(|v| v.as_str())
        .and_then(normalize_doi);
    let journal = p.journal.and_then(|j| j.name).filter(|n| !n.trim().is_empty());
    Some(NewPaper {
        ext_id: format!("s2:{id}"),
        doi,
        title,
        year: p.year.filter(|y| (1500..=2100).contains(y)),
        source_name: clean_opt(journal.as_deref().or(p.venue.as_deref()), MAX_FIELD),
        doc_type: doc_type(&p.publication_types.unwrap_or_default()),
        cited_by: p.citation_count.unwrap_or(0).max(0),
        authors: authors_display(&names),
        author_count: names.len() as i64,
        author_names: names.clone(),
        abstract_text: clean_opt(p.abstract_text.as_deref(), MAX_ABSTRACT),
        ref_count: p.reference_count.unwrap_or(0).max(0),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::super::model::SearchPage;

    #[test]
    fn maps_search_results() {
        let json = r#"{"total":1,"offset":0,"next":null,"data":[{"paperId":"abc123",
            "externalIds":{"DOI":"10.1/X","CorpusId":42},"title":"Cough audio","abstract":null,
            "venue":"IEEE Access","year":2024,"referenceCount":30,"citationCount":5,
            "publicationTypes":["JournalArticle"],"authors":[{"authorId":"1","name":"C. Okafor"}],
            "journal":null},{"paperId":"../x","title":"Bad id"}]}"#;
        let page: SearchPage = serde_json::from_str(json).unwrap();
        let papers: Vec<_> = page.data.unwrap().into_iter().filter_map(super::to_paper).collect();
        assert_eq!(papers.len(), 1);
        let p = &papers[0];
        assert_eq!(p.ext_id, "s2:abc123");
        assert_eq!(p.doi.as_deref(), Some("10.1/x"));
        assert_eq!(p.source_name.as_deref(), Some("IEEE Access"));
        assert_eq!(p.doc_type.as_deref(), Some("article"));
    }
}
