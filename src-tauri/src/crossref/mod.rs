pub(crate) mod convert;
mod model;
pub mod refs;

use crate::db::papers::NewPaper;
use crate::error::AppResult;
use crate::http;
use crate::search::SearchQuery;

pub(crate) const SERVICE: &str = "Crossref";
pub(crate) const WORKS_URL: &str = "https://api.crossref.org/works";
const PER_PAGE: u32 = 100;
const SELECT: &str = "DOI,ISSN,title,author,container-title,issued,type,is-referenced-by-count,abstract,\
references-count,reference";
/// Crossref's public pool is shared; keep paging gentle.
const PAGE_PAUSE_MS: u64 = 250;

fn type_filter(doc_type: &str) -> Option<&'static str> {
    match doc_type {
        "article" => Some("journal-article"),
        "proceedings" => Some("proceedings-article"),
        "book-chapter" => Some("book-chapter"),
        "preprint" => Some("posted-content"),
        // Crossref has no "review" type; searching without a type filter.
        _ => None,
    }
}

fn filter_param(q: &SearchQuery) -> String {
    // Validated integers and allowlisted literals only.
    let mut f = format!("from-pub-date:{}-01-01,until-pub-date:{}-12-31", q.year_from, q.year_to);
    if let Some(t) = q.doc_type.and_then(type_filter) {
        f.push_str(",type:");
        f.push_str(t);
    }
    f
}

pub async fn search(client: &reqwest::Client, q: &SearchQuery) -> AppResult<Vec<NewPaper>> {
    let filter = filter_param(q);
    let mut out = Vec::new();
    let mut offset = 0;
    while offset < q.max_results {
        if offset > 0 {
            http::pause(PAGE_PAUSE_MS).await;
        }
        let rows = PER_PAGE.min(q.max_results - offset);
        let (rows_s, offset_s) = (rows.to_string(), offset.to_string());
        let res = client
            .get(WORKS_URL)
            .query(&[
                ("query", q.text.as_str()),
                ("filter", filter.as_str()),
                ("select", SELECT),
                ("rows", rows_s.as_str()),
                ("offset", offset_s.as_str()),
            ])
            .send()
            .await?;
        http::check(&res, SERVICE)?;
        let page: model::Page = res.json().await?;
        let items = page.message.items.unwrap_or_default();
        let count = items.len() as u32;
        out.extend(items.into_iter().filter_map(convert::to_paper));
        if count < rows {
            break;
        }
        offset += count;
    }
    Ok(out)
}

#[cfg(test)]
mod live {
    /// Hits the real API. Run with: cargo test -- --ignored
    #[tokio::test]
    #[ignore]
    async fn fetches_real_records() {
        let q = super::SearchQuery {
            text: "tuberculosis deep learning".into(),
            year_from: 2020,
            year_to: 2024,
            doc_type: Some("article"),
            max_results: 5,
        };
        let papers = super::search(&crate::http::client().unwrap(), &q).await.unwrap();
        assert!(!papers.is_empty());
        assert!(papers.iter().all(|p| p.ext_id.starts_with("doi:10.")));
    }
}
