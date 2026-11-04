//! Unpaywall: legal open-access copies of a work, looked up by DOI. The API asks every
//! caller for a contact email, which the researcher enters in Settings (kept in the keychain).

use crate::error::{AppError, AppResult};
use crate::http;
use serde::Deserialize;

const BASE: &str = "https://api.unpaywall.org/v2/";
pub const SERVICE: &str = "Unpaywall";
const MAX_BODY: usize = 512 * 1024;
const MAX_URL: usize = 2_000;

#[derive(Deserialize)]
struct Work {
    is_oa: Option<bool>,
    best_oa_location: Option<Location>,
}

#[derive(Deserialize)]
struct Location {
    url: Option<String>,
    url_for_pdf: Option<String>,
    host_type: Option<String>,
    license: Option<String>,
}

#[derive(Debug, PartialEq, Default)]
pub struct OaLink {
    pub is_oa: bool,
    pub url: Option<String>,
    pub pdf_url: Option<String>,
    /// "publisher" or "repository".
    pub host: Option<String>,
    pub license: Option<String>,
}

/// Only plain web links are kept, so a stored value can never open anything else.
pub fn web_url(raw: &str) -> Option<String> {
    let url = raw.trim();
    let ok = (url.starts_with("https://") || url.starts_with("http://")) && url.len() <= MAX_URL && !url.chars().any(|c| c.is_whitespace() || c.is_control());
    ok.then(|| url.to_string())
}

fn short(raw: Option<String>) -> Option<String> {
    raw.map(|v| crate::text::clean(&v, 60)).filter(|v| !v.is_empty())
}

fn parse(body: &[u8]) -> AppResult<OaLink> {
    let w: Work = serde_json::from_slice(body).map_err(|_| AppError::Http(crate::tr!("format respons Unpaywall tidak dikenali", "unrecognized Unpaywall response").into()))?;
    let loc = w.best_oa_location;
    Ok(OaLink {
        is_oa: w.is_oa.unwrap_or(false),
        url: loc.as_ref().and_then(|l| l.url.as_deref()).and_then(web_url),
        pdf_url: loc.as_ref().and_then(|l| l.url_for_pdf.as_deref()).and_then(web_url),
        host: short(loc.as_ref().and_then(|l| l.host_type.clone())),
        license: short(loc.and_then(|l| l.license)),
    })
}

/// The open-access copy of `doi`, or a closed result when Unpaywall does not know the DOI.
pub async fn lookup(client: &reqwest::Client, doi: &str, email: &str) -> AppResult<OaLink> {
    let mut url = reqwest::Url::parse(BASE).map_err(|_| AppError::Internal)?;
    // The DOI goes in as one path segment, percent-encoded, never as raw URL text.
    url.path_segments_mut().map_err(|_| AppError::Internal)?.pop_if_empty().push(doi);
    let res = client.get(url).query(&[("email", email)]).send().await?;
    if res.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(OaLink::default());
    }
    http::check(&res, SERVICE)?;
    parse(&http::body_limited(res, MAX_BODY, SERVICE).await?)
}

#[cfg(test)]
mod tests {
    use super::{parse, web_url};

    #[test]
    fn reads_the_best_location_and_keeps_only_web_links() {
        let body = br#"{"is_oa":true,"best_oa_location":{"url":"https://europepmc.org/articles/pmc1","url_for_pdf":"javascript:alert(1)","host_type":"repository","license":"cc-by"}}"#;
        let link = parse(body).unwrap();
        assert!(link.is_oa);
        assert_eq!(link.url.as_deref(), Some("https://europepmc.org/articles/pmc1"));
        assert_eq!(link.pdf_url, None, "non-web links are dropped");
        assert_eq!((link.host.as_deref(), link.license.as_deref()), (Some("repository"), Some("cc-by")));
        let closed = parse(br#"{"is_oa":false,"best_oa_location":null}"#).unwrap();
        assert!(!closed.is_oa && closed.url.is_none());
        assert!(parse(b"<html>").is_err());
        assert_eq!(web_url("file:///etc/passwd"), None);
        assert_eq!(web_url("https://a.org/x y"), None);
    }
}
