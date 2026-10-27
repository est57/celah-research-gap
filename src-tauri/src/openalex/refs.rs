//! Cited works and institutions of records already in the collection, looked up by
//! OpenAlex id or DOI.

use super::SERVICE;
use super::convert::institutions;
use super::model::Authorship;
use crate::error::AppResult;
use crate::geo::affil::Affiliation;
use crate::http;
use crate::text::normalize_doi;
use serde::Deserialize;

const WORKS_URL: &str = "https://api.openalex.org/works";
/// OR-values per filter; OpenAlex accepts up to 100.
pub const BATCH: usize = 50;
const MAX_CITES: usize = 1000;

#[derive(Deserialize)]
struct Page {
    #[serde(default)]
    results: Vec<RefWork>,
}

#[derive(Deserialize)]
struct RefWork {
    id: Option<String>,
    doi: Option<String>,
    #[serde(default)]
    referenced_works: Vec<String>,
    #[serde(default)]
    authorships: Vec<Authorship>,
}

pub struct RefRecord {
    /// "W123".
    pub id: String,
    pub doi: Option<String>,
    /// "openalex:W..." of each cited work.
    pub cites: Vec<String>,
    pub affiliations: Vec<Affiliation>,
}

/// "W123" from an OpenAlex URL, digits only after the W.
pub fn short_id(url: &str) -> Option<String> {
    let id = url.rsplit('/').next()?;
    let valid = id.len() <= 32 && id.len() > 1 && id.starts_with('W') && id[1..].chars().all(|c| c.is_ascii_digit());
    valid.then(|| id.to_string())
}

/// A DOI that cannot break the filter syntax (no OR bar, comma or whitespace).
pub fn filter_safe_doi(doi: &str) -> bool {
    doi.starts_with("10.") && doi.len() <= 300 && !doi.chars().any(|c| c == '|' || c == ',' || c.is_whitespace() || c.is_control())
}

/// `field` is "ids.openalex" or "doi"; values must already be validated with short_id / filter_safe_doi.
pub async fn fetch(client: &reqwest::Client, field: &'static str, values: &[String], api_key: Option<&str>) -> AppResult<Vec<RefRecord>> {
    let filter = format!("{field}:{}", values.join("|"));
    let per_page = values.len().min(100).to_string();
    let mut req = client.get(WORKS_URL).query(&[("filter", filter.as_str()), ("per-page", per_page.as_str()), ("select", "id,doi,referenced_works,authorships")]);
    if let Some(key) = api_key {
        req = req.query(&[("api_key", key)]);
    }
    let res = req.send().await?;
    http::check(&res, SERVICE)?;
    let page: Page = res.json().await?;
    Ok(page
        .results
        .into_iter()
        .filter_map(|w| {
            Some(RefRecord {
                id: short_id(w.id.as_deref()?)?,
                doi: w.doi.as_deref().and_then(normalize_doi),
                cites: w.referenced_works.iter().filter_map(|u| short_id(u)).take(MAX_CITES).map(|id| format!("openalex:{id}")).collect(),
                affiliations: institutions(&w.authorships),
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_ids_and_dois_for_the_filter() {
        assert_eq!(short_id("https://openalex.org/W2963").as_deref(), Some("W2963"));
        assert!(short_id("https://openalex.org/W12|W3").is_none());
        assert!(short_id("W").is_none());
        assert!(filter_safe_doi("10.1148/radiol.2017162326"));
        assert!(!filter_safe_doi("10.1/a|doi:10.2/b"));
        assert!(!filter_safe_doi("10.1/a,b"));
    }
}
