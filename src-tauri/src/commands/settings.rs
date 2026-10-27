use crate::error::AppResult;
use crate::secrets::{self, Service};
use crate::validate;
use serde::Serialize;

/// What the frontend may know about a stored key. The key itself is never returned.
#[derive(Serialize)]
pub struct KeyStatus {
    configured: bool,
    hint: Option<String>,
}

impl KeyStatus {
    fn from_key(key: Option<&str>) -> Self {
        KeyStatus { configured: key.is_some(), hint: key.map(secrets::hint) }
    }
}

// `async` runs these off the main thread, since keychain calls may block on an OS prompt.

#[tauri::command(async)]
pub fn api_key_status(service: Service) -> AppResult<KeyStatus> {
    let key = secrets::get(service)?;
    Ok(KeyStatus::from_key(key.as_deref()))
}

#[tauri::command(async)]
pub fn set_api_key(service: Service, key: String) -> AppResult<KeyStatus> {
    let key = validate::api_key(&key)?;
    secrets::set(service, &key)?;
    Ok(KeyStatus::from_key(Some(&key)))
}

#[tauri::command(async)]
pub fn clear_api_key(service: Service) -> AppResult<KeyStatus> {
    secrets::clear(service)?;
    Ok(KeyStatus::from_key(None))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    version: &'static str,
    os: &'static str,
    arch: &'static str,
    sqlite: &'static str,
    /// Folder of the open project's database, shown so the user can find and back it up.
    data_dir: String,
    database: String,
    /// Database file plus its write-ahead log, in bytes.
    database_bytes: u64,
    documents: i64,
}

/// Version and storage facts for the About tab and support requests. No network, no keys.
#[tauri::command(async)]
pub fn app_info(state: tauri::State<'_, crate::AppState>) -> AppResult<AppInfo> {
    use crate::error::AppError;
    let path = state.db_path.lock().map_err(|_| AppError::Internal)?.clone();
    let size = |p: &std::path::Path| std::fs::metadata(p).map(|m| m.len()).unwrap_or(0);
    let wal = path.with_extension("sqlite-wal");
    let documents: i64 = {
        let conn = state.db.lock().map_err(|_| AppError::Internal)?;
        conn.query_row("SELECT COUNT(*) FROM papers", [], |r| r.get(0))?
    };
    Ok(AppInfo {
        version: env!("CARGO_PKG_VERSION"),
        os: std::env::consts::OS,
        arch: std::env::consts::ARCH,
        sqlite: rusqlite::version(),
        data_dir: path.parent().map(|p| p.display().to_string()).unwrap_or_default(),
        database: path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string(),
        database_bytes: size(&path) + size(&wal),
        documents,
    })
}
