mod convert;
mod model;

use crate::db::papers::NewPaper;
use crate::error::AppResult;
use crate::http;
use crate::search::SearchQuery;

const SERVICE: &str = "Semantic Scholar";
const SEARCH_URL: &str = "https://api.semanticscholar.org/graph/v1/paper/search";
const FIELDS: &str = "paperId,externalIds,title,abstract,venue,year,referenceCount,citationCount,\
publicationTypes,authors,journal";
const PER_PAGE: u32 = 100;
/// The relevance search endpoint only serves the first 1,000 results.
const MAX_WINDOW: u32 = 1_000;
/// Semantic Scholar allows about one request per second per key.
const PAGE_PAUSE_MS: u64 = 1_100;

fn type_filter(doc_type: &str) -> Option<&'static str> {
    match doc_type {
        "article" => Some("JournalArticle"),
        "review" => Some("Review"),
        "proceedings" => Some("Conference"),
        "book-chapter" => Some("BookSection"),
        _ => None,
    }
}

pub async fn search(client: &reqwest::Client, q: &SearchQuery, api_key: Option<&str>) -> AppResult<Vec<NewPaper>> {
    let max = q.max_results.min(MAX_WINDOW);
    let years = format!("{}-{}", q.year_from, q.year_to);
    let types = q.doc_type.and_then(type_filter);
    let key_header = api_key.map(http::secret_header).transpose()?;
    let mut out = Vec::new();
    let mut offset = 0;
    while offset < max {
        if offset > 0 {
            http::pause(PAGE_PAUSE_MS).await;
        }
        let limit = PER_PAGE.min(max - offset);
        let (offset_s, limit_s) = (offset.to_string(), limit.to_string());
        let mut req = client.get(SEARCH_URL).query(&[
            ("query", q.text.as_str()),
            ("fields", FIELDS),
            ("year", years.as_str()),
            ("offset", offset_s.as_str()),
            ("limit", limit_s.as_str()),
        ]);
        if let Some(t) = types {
            req = req.query(&[("publicationTypes", t)]);
        }
        if let Some(h) = &key_header {
            req = req.header("x-api-key", h.clone());
        }
        let res = req.send().await?;
        http::check(&res, SERVICE)?;
        let page: model::SearchPage = res.json().await?;
        let data = page.data.unwrap_or_default();
        let count = data.len() as u32;
        out.extend(data.into_iter().filter_map(convert::to_paper));
        if page.next.is_none() || count < limit {
            break;
        }
        offset += count;
    }
    Ok(out)
}
