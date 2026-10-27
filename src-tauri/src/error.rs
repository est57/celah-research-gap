use serde::{Serialize, Serializer};

/// Errors returned to the frontend. Messages are user-facing and never
/// include internal details such as SQL text, file paths or raw responses.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Input tidak valid: {0}")]
    Invalid(String),
    #[error("Gagal mengakses database lokal.")]
    Db,
    #[error("Gagal menghubungi sumber data: {0}")]
    Http(String),
    #[error("Kuota {0} habis atau permintaan terlalu cepat. Coba lagi nanti, atau pasang kunci API di Pengaturan.")]
    RateLimited(&'static str),
    #[error("{0} menolak kunci API. Periksa atau hapus kunci di Pengaturan.")]
    Unauthorized(&'static str),
    #[error("Gagal mengakses keychain sistem.")]
    Keychain,
    #[error("Terjadi kesalahan internal.")]
    Internal,
}

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
            AppError::Http("waktu tunggu habis".into())
        } else if err.is_decode() {
            AppError::Http("format respons tidak dikenali".into())
        } else {
            AppError::Http("koneksi gagal".into())
        }
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
