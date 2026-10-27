//! API key storage in the OS keychain (macOS Keychain, Windows Credential
//! Manager, Secret Service on Linux). Keys never leave the backend: the
//! frontend only sees a short hint.

use crate::error::{AppError, AppResult};
use keyring::Entry;
use serde::Deserialize;

const SERVICE: &str = "id.celah.desktop";

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Service {
    Openalex,
    SemanticScholar,
    /// NCBI E-utilities (PubMed).
    Ncbi,
}

impl Service {
    fn account(self) -> &'static str {
        match self {
            Service::Openalex => "openalex-api-key",
            Service::SemanticScholar => "semantic-scholar-api-key",
            Service::Ncbi => "ncbi-api-key",
        }
    }
}

fn map(err: keyring::Error) -> AppError {
    // keyring errors describe the store, not the secret, so they are safe to log.
    eprintln!("[keychain] {err}");
    AppError::Keychain
}

fn entry(service: Service) -> AppResult<Entry> {
    Entry::new(SERVICE, service.account()).map_err(map)
}

pub fn get(service: Service) -> AppResult<Option<String>> {
    match entry(service)?.get_password() {
        Ok(key) => Ok(Some(key)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(err) => Err(map(err)),
    }
}

pub fn set(service: Service, key: &str) -> AppResult<()> {
    entry(service)?.set_password(key).map_err(map)
}

pub fn clear(service: Service) -> AppResult<()> {
    match entry(service)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(err) => Err(map(err)),
    }
}

/// Last four characters only, enough for the user to recognise the key.
pub fn hint(key: &str) -> String {
    let tail: String = key.chars().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect();
    format!("****{tail}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn hint_shows_only_the_tail() {
        assert_eq!(super::hint("abcdef123456"), "****3456");
        assert_eq!(super::hint("ab"), "****ab");
    }
}
