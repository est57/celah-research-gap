//! PubMed via NCBI E-utilities: esearch finds PMIDs, efetch returns full XML records.

mod xml;

use crate::db::papers::NewPaper;
use crate::error::AppResult;
use crate::http;
use crate::search::SearchQuery;
use serde::Deserialize;

const SERVICE: &str = "PubMed";
const ESEARCH_URL: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi";
const EFETCH_URL: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi";
const TOOL: &str = "celah";
const FETCH_BATCH: usize = 200;
const MAX_XML_BYTES: usize = 40 * 1024 * 1024;

#[derive(Deserialize)]
struct ESearch {
    esearchresult: ESearchResult,
}

#[derive(Deserialize)]
struct ESearchResult {
    #[serde(default)]
    idlist: Vec<String>,
}

fn type_filter(doc_type: &str) -> Option<&'static str> {
    match doc_type {
        "article" => Some("journal article[pt]"),
        "review" => Some("review[pt]"),
        "preprint" => Some("preprint[pt]"),
        "proceedings" => Some("congress[pt]"),
        _ => None,
    }
}

/// NCBI allows 3 requests per second without a key and 10 with one.
fn pause_ms(has_key: bool) -> u64 {
    if has_key { 120 } else { 360 }
}

pub async fn search(client: &reqwest::Client, q: &SearchQuery, api_key: Option<&str>) -> AppResult<Vec<NewPaper>> {
    let term = match q.doc_type.and_then(type_filter) {
        Some(t) => format!("({}) AND {t}", q.text),
        None => q.text.clone(),
    };
    let (retmax, from, to) = (q.max_results.to_string(), q.year_from.to_string(), q.year_to.to_string());
    let mut req = client.get(ESEARCH_URL).query(&[
        ("db", "pubmed"),
        ("term", term.as_str()),
        ("retmode", "json"),
        ("retmax", retmax.as_str()),
        ("datetype", "pdat"),
        ("mindate", from.as_str()),
        ("maxdate", to.as_str()),
        ("tool", TOOL),
    ]);
    if let Some(key) = api_key {
        // E-utilities only accepts the key as a query parameter; error logs drop the URL.
        req = req.query(&[("api_key", key)]);
    }
    let res = req.send().await?;
    http::check(&res, SERVICE)?;
    let ids: Vec<String> = res
        .json::<ESearch>()
        .await?
        .esearchresult
        .idlist
        .into_iter()
        .filter(|id| !id.is_empty() && id.len() <= 12 && id.chars().all(|c| c.is_ascii_digit()))
        .collect();

    let mut out = Vec::new();
    for batch in ids.chunks(FETCH_BATCH) {
        http::pause(pause_ms(api_key.is_some())).await;
        let id_list = batch.join(",");
        let mut req = client.get(EFETCH_URL).query(&[
            ("db", "pubmed"),
            ("id", id_list.as_str()),
            ("retmode", "xml"),
            ("tool", TOOL),
        ]);
        if let Some(key) = api_key {
            req = req.query(&[("api_key", key)]);
        }
        let res = req.send().await?;
        http::check(&res, SERVICE)?;
        let body = http::body_limited(res, MAX_XML_BYTES, SERVICE).await?;
        out.extend(xml::parse(&String::from_utf8_lossy(&body)));
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
        let papers = super::search(&crate::http::client().unwrap(), &q, None).await.unwrap();
        assert!(!papers.is_empty());
        assert!(papers.iter().all(|p| p.ext_id.starts_with("pmid:") && !p.title.is_empty()));
    }
}
