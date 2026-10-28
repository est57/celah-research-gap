use crate::tr;
use serde::{Serialize, Serializer};
use std::fmt;

/// Errors returned to the frontend. Messages are user-facing, in the interface language
/// (the detail strings are built with `tr!` where they are raised), and never include
/// internal details such as SQL text, file paths or raw responses.
#[derive(Debug)]
pub enum AppError {
    Invalid(String),
    Db,
    Http(String),
    RateLimited(&'static str),
    Unauthorized(&'static str),
    Keychain,
    Internal,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            AppError::Invalid(m) => tr!(format!("Input tidak valid: {m}"), format!("Invalid input: {m}")),
            AppError::Db => tr!("Gagal mengakses database lokal.", "Could not access the local database.").into(),
            AppError::Http(m) => tr!(format!("Gagal menghubungi sumber data: {m}"), format!("Could not reach the data source: {m}")),
            AppError::RateLimited(s) => tr!(
                format!("Kuota {s} habis atau permintaan terlalu cepat. Coba lagi nanti, atau pasang kunci API di Pengaturan."),
                format!("{s} quota is used up or requests came too fast. Try again later, or add an API key in Settings.")
            ),
            AppError::Unauthorized(s) => tr!(
                format!("{s} menolak kunci API. Periksa atau hapus kunci di Pengaturan."),
                format!("{s} rejected the API key. Check or remove the key in Settings.")
            ),
            AppError::Keychain => tr!("Gagal mengakses keychain sistem.", "Could not access the system keychain.").into(),
            AppError::Internal => tr!("Terjadi kesalahan internal.", "An internal error occurred.").into(),
        };
        f.write_str(&text)
    }
}

impl std::error::Error for AppError {}

pub type AppResult<T> = Result<T, AppError>;

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        eprintln!("[db] {err}");
        AppError::Db
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        // Drop the URL before logging: it may carry the api_key query parameter.
        let err = err.without_url();
        eprintln!("[http] {err}");
        if err.is_timeout() {
            AppError::Http(tr!("waktu tunggu habis", "timed out").into())
        } else if err.is_decode() {
            AppError::Http(tr!("format respons tidak dikenali", "unrecognized response format").into())
        } else {
            AppError::Http(tr!("koneksi gagal", "connection failed").into())
        }
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
