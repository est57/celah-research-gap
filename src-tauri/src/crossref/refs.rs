//! Reference lists of stored documents, looked up in Crossref by DOI.

use super::convert::reference_text;
use super::{model, SERVICE, WORKS_URL};
use crate::error::{AppError, AppResult};
use crate::http;
use crate::import::limits::MAX_REFS;
use crate::openalex::refs::filter_safe_doi;
use crate::text::normalize_doi;

/// DOIs per request; Crossref ORs repeated `doi:` filters.
pub const BATCH: usize = 20;

/// (DOI, reference texts) for each DOI Crossref knows; works without deposited references
/// come back with an empty list. `dois` must be normalized and filter-safe.
pub async fn by_doi(client: &reqwest::Client, dois: &[String]) -> AppResult<Vec<(String, Vec<String>)>> {
    if dois.is_empty() || dois.len() > BATCH || !dois.iter().all(|d| filter_safe_doi(d)) {
        return Err(AppError::Invalid(crate::tr!(format!("isi 1 sampai {BATCH} DOI yang valid"), format!("give 1 to {BATCH} valid DOIs"))));
    }
    let filter = dois.iter().map(|d| format!("doi:{d}")).collect::<Vec<_>>().join(",");
    let rows = dois.len().to_string();
    let res = client
        .get(WORKS_URL)
        .query(&[("filter", filter.as_str()), ("select", "DOI,reference"), ("rows", rows.as_str())])
        .send()
        .await?;
    http::check(&res, SERVICE)?;
    let page: model::Page = res.json().await?;
    Ok(page
        .message
        .items
        .unwrap_or_default()
        .into_iter()
        .filter_map(|item| {
            let doi = item.doi.as_deref().and_then(normalize_doi)?;
            let refs = item.reference.unwrap_or_default().iter().filter_map(reference_text).take(MAX_REFS).collect();
            Some((doi, refs))
        })
        .collect())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn rejects_unsafe_or_too_many_dois_before_any_request() {
        let client = crate::http::client().unwrap();
        assert!(super::by_doi(&client, &[]).await.is_err());
        assert!(super::by_doi(&client, &["10.1/a,doi:10.2/b".to_string()]).await.is_err(), "no filter injection");
        let many: Vec<String> = (0..21).map(|i| format!("10.1/{i}")).collect();
        assert!(super::by_doi(&client, &many).await.is_err());
    }
}

#[cfg(test)]
mod live {
    /// Run with: CELAH_DOIS=path cargo test crossref::refs::live -- --ignored --nocapture
    #[tokio::test]
    #[ignore]
    async fn live_refs_by_doi() {
        let path = std::env::var("CELAH_DOIS").unwrap();
        let dois: Vec<String> = std::fs::read_to_string(path).unwrap().lines().map(str::to_string).take(super::BATCH).collect();
        let found = super::by_doi(&crate::http::client().unwrap(), &dois).await.unwrap();
        let with = found.iter().filter(|(_, r)| !r.is_empty()).count();
        println!("asked {} found {} with references {}", dois.len(), found.len(), with);
        for (d, r) in found.iter().take(4) {
            println!("  {d}: {} refs; first: {}", r.len(), r.first().map(|s| s.chars().take(90).collect::<String>()).unwrap_or_default());
        }
    }
}
