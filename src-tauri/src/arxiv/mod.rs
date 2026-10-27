//! arXiv API (Atom feed). Everything on arXiv is a preprint.

mod atom;

use crate::db::papers::NewPaper;
use crate::error::{AppError, AppResult};
use crate::http;
use crate::search::SearchQuery;

const SERVICE: &str = "arXiv";
const QUERY_URL: &str = "https://export.arxiv.org/api/query";
const PER_PAGE: u32 = 100;
/// arXiv asks clients to wait three seconds between calls.
const PAGE_PAUSE_MS: u64 = 3_000;
const MAX_FEED_BYTES: usize = 20 * 1024 * 1024;

/// Keeps letters, digits and hyphens of each word and joins them with AND,
/// so user text cannot inject arXiv query operators or field prefixes.
fn search_query(q: &SearchQuery) -> Option<String> {
    let terms: Vec<String> = q
        .text
        .split_whitespace()
        .map(|w| w.chars().filter(|c| c.is_alphanumeric() || *c == '-').collect::<String>())
        .filter(|w| !w.is_empty() && !matches!(w.to_uppercase().as_str(), "AND" | "OR" | "ANDNOT"))
        .take(20)
        .map(|w| format!("all:{w}"))
        .collect();
    if terms.is_empty() {
        return None;
    }
    Some(format!(
        "{} AND submittedDate:[{}01010000 TO {}12312359]",
        terms.join(" AND "),
        q.year_from,
        q.year_to
    ))
}

pub async fn search(client: &reqwest::Client, q: &SearchQuery) -> AppResult<Vec<NewPaper>> {
    if q.doc_type.is_some_and(|t| t != "preprint") {
        return Err(AppError::Invalid(
            "arXiv hanya berisi pra-cetak; kosongkan filter jenis atau pilih Pra-cetak".into(),
        ));
    }
    let query = search_query(q).ok_or_else(|| AppError::Invalid("kata kunci tidak bisa dipakai untuk arXiv".into()))?;
    let mut out = Vec::new();
    let mut start = 0;
    while start < q.max_results {
        if start > 0 {
            http::pause(PAGE_PAUSE_MS).await;
        }
        let size = PER_PAGE.min(q.max_results - start);
        let (start_s, size_s) = (start.to_string(), size.to_string());
        let res = client
            .get(QUERY_URL)
            .query(&[
                ("search_query", query.as_str()),
                ("start", start_s.as_str()),
                ("max_results", size_s.as_str()),
                ("sortBy", "relevance"),
            ])
            .send()
            .await?;
        http::check(&res, SERVICE)?;
        let body = http::body_limited(res, MAX_FEED_BYTES, SERVICE).await?;
        let (papers, seen) = atom::parse(&String::from_utf8_lossy(&body));
        out.extend(papers);
        if (seen as u32) < size {
            break;
        }
        start += seen as u32;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query(text: &str) -> SearchQuery {
        SearchQuery { text: text.into(), year_from: 2020, year_to: 2025, doc_type: None, max_results: 5 }
    }

    #[test]
    fn builds_safe_queries() {
        assert_eq!(
            search_query(&query("tuberculosis deep-learning")).unwrap(),
            "all:tuberculosis AND all:deep-learning AND submittedDate:[202001010000 TO 202512312359]"
        );
        let injected = search_query(&query("cough OR ti:(x) ANDNOT au:smith")).unwrap();
        assert!(injected.starts_with("all:cough AND all:tix AND all:ausmith AND"));
        assert!(search_query(&query("() :: OR")).is_none());
    }

    /// Hits the real API. Run with: cargo test -- --ignored
    #[tokio::test]
    #[ignore]
    async fn fetches_real_records() {
        let papers = search(&crate::http::client().unwrap(), &query("tuberculosis deep learning")).await.unwrap();
        assert!(!papers.is_empty());
        assert!(papers.iter().all(|p| p.ext_id.starts_with("arxiv:")));
    }
}
