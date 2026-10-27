//! Full OpenAlex records for a list of DOIs, to add cited works to the collection.

use super::{convert, model, refs::filter_safe_doi, SELECT, SERVICE, WORKS_URL};
use crate::db::papers::NewPaper;
use crate::error::{AppError, AppResult};
use crate::http;

/// OR-values per request; OpenAlex accepts up to 100.
pub const MAX_DOIS: usize = 50;

/// `dois` must be normalized (lowercase, "10." prefix); unsafe values are rejected.
pub async fn by_doi(client: &reqwest::Client, dois: &[String], api_key: Option<&str>) -> AppResult<Vec<NewPaper>> {
    if dois.is_empty() || dois.len() > MAX_DOIS || !dois.iter().all(|d| filter_safe_doi(d)) {
        return Err(AppError::Invalid(format!("isi 1 sampai {MAX_DOIS} DOI yang valid")));
    }
    let filter = format!("doi:{}", dois.join("|"));
    let per_page = dois.len().to_string();
    let mut req = client.get(WORKS_URL).query(&[("filter", filter.as_str()), ("per-page", per_page.as_str()), ("select", SELECT)]);
    if let Some(key) = api_key {
        req = req.query(&[("api_key", key)]);
    }
    let res = req.send().await?;
    http::check(&res, SERVICE)?;
    let page: model::WorksPage = res.json().await?;
    Ok(page.results.into_iter().filter_map(convert::to_paper).collect())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn rejects_unsafe_or_too_many_dois_before_any_request() {
        let client = crate::http::client().unwrap();
        let bad = |d: &str| vec![d.to_string()];
        assert!(super::by_doi(&client, &[], None).await.is_err());
        assert!(super::by_doi(&client, &bad("10.1/a|doi:10.2/b"), None).await.is_err(), "no OR injection");
        assert!(super::by_doi(&client, &bad("10.1/a,b"), None).await.is_err());
        assert!(super::by_doi(&client, &bad("not a doi"), None).await.is_err());
        let many: Vec<String> = (0..51).map(|i| format!("10.1/{i}")).collect();
        assert!(super::by_doi(&client, &many, None).await.is_err());
    }
}
