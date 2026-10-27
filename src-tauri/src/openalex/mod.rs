pub(crate) mod convert;
pub(crate) mod model;
pub mod lookup;
pub mod refs;

use crate::db::papers::NewPaper;
use crate::error::AppResult;
use crate::http;
use crate::search::SearchQuery;

pub(crate) const SERVICE: &str = "OpenAlex";
pub(crate) const WORKS_URL: &str = "https://api.openalex.org/works";
const PER_PAGE: u32 = 100;
pub(crate) const SELECT: &str = "id,doi,display_name,publication_year,type,cited_by_count,authorships,\
primary_location,abstract_inverted_index,keywords,referenced_works_count,referenced_works";

fn type_filter(doc_type: &str) -> &'static str {
    match doc_type {
        "proceedings" => "conference-paper",
        "review" => "review",
        "preprint" => "preprint",
        "book-chapter" => "book-chapter",
        _ => "article",
    }
}

fn filter_param(q: &SearchQuery) -> String {
    // Values here are validated integers and allowlisted literals only.
    let mut f = format!(
        "from_publication_date:{}-01-01,to_publication_date:{}-12-31",
        q.year_from, q.year_to
    );
    if let Some(t) = q.doc_type {
        f.push_str(",type:");
        f.push_str(type_filter(t));
    }
    f
}

pub async fn search(client: &reqwest::Client, q: &SearchQuery, api_key: Option<&str>) -> AppResult<Vec<NewPaper>> {
    let filter = filter_param(q);
    let mut out = Vec::new();
    let pages = q.max_results.div_ceil(PER_PAGE);
    for page in 1..=pages {
        if page > 1 {
            http::pause(150).await;
        }
        let page_s = page.to_string();
        let per_page_s = PER_PAGE.to_string();
        let mut req = client.get(WORKS_URL).query(&[
            ("search", q.text.as_str()),
            ("filter", filter.as_str()),
            ("per-page", per_page_s.as_str()),
            ("page", page_s.as_str()),
            ("select", SELECT),
        ]);
        if let Some(key) = api_key {
            // OpenAlex only accepts the key as a query parameter; error logs drop the URL.
            req = req.query(&[("api_key", key)]);
        }
        let res = req.send().await?;
        http::check(&res, SERVICE)?;
        let body: model::WorksPage = res.json().await?;
        let count = body.results.len();
        out.extend(body.results.into_iter().filter_map(convert::to_paper));
        if count < PER_PAGE as usize || out.len() >= q.max_results as usize {
            break;
        }
    }
    out.truncate(q.max_results as usize);
    Ok(out)
}

#[cfg(test)]
mod live {
    /// Hits the real API. Run with: cargo test -- --ignored
    #[tokio::test]
    #[ignore]
    async fn reads_countries_from_real_records() {
        let q = super::SearchQuery {
            text: "tuberculosis deep learning".into(),
            year_from: 2020,
            year_to: 2024,
            doc_type: Some("article"),
            max_results: 20,
        };
        let papers = super::search(&crate::http::client().unwrap(), &q, None).await.unwrap();
        assert!(!papers.is_empty());
        let with_country = papers.iter().filter(|p| !p.countries.is_empty()).count();
        assert!(with_country * 2 >= papers.len(), "most OpenAlex records carry countries, got {with_country}");
        assert!(papers.iter().all(|p| p.countries.iter().all(|c| c.len() == 2)));
    }
}
