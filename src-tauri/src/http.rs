//! Shared HTTP client and response checks for every research API connector.

use crate::error::{AppError, AppResult};
use reqwest::header::HeaderValue;
use reqwest::{Response, StatusCode};
use std::time::Duration;

/// HTTPS only, short timeouts, few redirects.
pub fn client() -> AppResult<reqwest::Client> {
    reqwest::Client::builder()
        .https_only(true)
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::limited(3))
        .user_agent(concat!("Celah/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|_| AppError::Internal)
}

/// Maps non-success statuses onto user-facing errors for `service`.
pub fn check(res: &Response, service: &'static str) -> AppResult<()> {
    match res.status() {
        s if s.is_success() => Ok(()),
        // arXiv signals throttling with 503 instead of 429.
        StatusCode::TOO_MANY_REQUESTS | StatusCode::SERVICE_UNAVAILABLE => Err(AppError::RateLimited(service)),
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Err(AppError::Unauthorized(service)),
        s => Err(AppError::Http(crate::tr!(format!("{service} membalas status {}", s.as_u16()), format!("{service} returned status {}", s.as_u16())))),
    }
}

/// Header value for an API key, flagged sensitive so it is redacted from debug output.
pub fn secret_header(key: &str) -> AppResult<HeaderValue> {
    let mut value = HeaderValue::from_str(key).map_err(|_| AppError::Invalid(crate::tr!("kunci API berisi karakter tidak sah", "the API key has invalid characters").into()))?;
    value.set_sensitive(true);
    Ok(value)
}

/// Reads a response body chunk by chunk and stops once it exceeds `max` bytes,
/// so an unexpectedly large reply cannot exhaust memory.
pub async fn body_limited(mut res: Response, max: usize, service: &'static str) -> AppResult<Vec<u8>> {
    let mut body = Vec::new();
    while let Some(chunk) = res.chunk().await? {
        if body.len() + chunk.len() > max {
            return Err(AppError::Http(crate::tr!(format!("respons {service} terlalu besar"), format!("the {service} response is too large"))));
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

/// Polite pause between paged requests.
pub async fn pause(ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await;
}
