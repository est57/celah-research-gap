mod convert;
mod model;

use crate::db::papers::NewPaper;
use crate::error::AppResult;
use crate::http;
use crate::search::SearchQuery;

const SERVICE: &str = "Europe PMC";
const SEARCH_URL: &str = "https://www.ebi.ac.uk/europepmc/webservices/rest/search";
const PER_PAGE: u32 = 100;
const PAGE_PAUSE_MS: u64 = 200;

fn type_clause(doc_type: &str) -> Option<&'static str> {
    match doc_type {
        "article" => Some(" AND PUB_TYPE:\"journal article\""),
        "review" => Some(" AND PUB_TYPE:\"review\""),
        "preprint" => Some(" AND SRC:PPR"),
        _ => None,
    }
}

/// The user's text is grouped; years and type are validated values only.
fn build_query(q: &SearchQuery) -> String {
    let mut query = format!("({}) AND PUB_YEAR:[{} TO {}]", q.text, q.year_from, q.year_to);
    if let Some(clause) = q.doc_type.and_then(type_clause) {
        query.push_str(clause);
    }
    query
}

pub async fn search(client: &reqwest::Client, q: &SearchQuery) -> AppResult<Vec<NewPaper>> {
    let query = build_query(q);
    let mut out = Vec::new();
    let mut cursor = String::from("*");
    let mut fetched = 0;
    while fetched < q.max_results {
        if fetched > 0 {
            http::pause(PAGE_PAUSE_MS).await;
        }
        let size = PER_PAGE.min(q.max_results - fetched).to_string();
        let res = client
            .get(SEARCH_URL)
            .query(&[
                ("query", query.as_str()),
                ("format", "json"),
                ("resultType", "core"),
                ("pageSize", size.as_str()),
                ("cursorMark", cursor.as_str()),
            ])
            .send()
            .await?;
        http::check(&res, SERVICE)?;
        let page: model::Page = res.json().await?;
        let records = page.result_list.and_then(|l| l.result).unwrap_or_default();
        let count = records.len() as u32;
        fetched += count;
        out.extend(records.into_iter().filter_map(convert::to_paper));
        match page.next_cursor_mark {
            Some(next) if count > 0 && next != cursor && next.len() <= 200 => cursor = next,
            _ => break,
        }
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
            year_to: 2025,
            doc_type: Some("review"),
            max_results: 5,
        };
        let papers = super::search(&crate::http::client().unwrap(), &q).await.unwrap();
        assert!(!papers.is_empty());
        assert!(papers.iter().all(|p| p.doc_type.as_deref() == Some("review")));
    }
}
