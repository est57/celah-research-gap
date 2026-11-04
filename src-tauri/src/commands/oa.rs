//! Open-access copies through Unpaywall, and opening a document in the system browser.

use crate::db::oa::{self, DocLinks, OaStatus};
use crate::error::{AppError, AppResult};
use crate::secrets::{self, Service};
use crate::{http, tr, unpaywall, AppState};
use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_opener::OpenerExt;

/// Lookups per run; Unpaywall allows about 100,000 a day, one DOI per request.
const PER_RUN: i64 = 300;
const PAUSE_MS: u64 = 120;
const MAX_IDS: usize = 500;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OaReport {
    pub checked: usize,
    pub found: usize,
    pub remaining: i64,
    /// Set when a request failed; lookups before it are saved.
    pub error: Option<String>,
}

#[tauri::command(async)]
pub fn open_access_status(state: State<'_, AppState>) -> AppResult<OaStatus> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    oa::status(&conn)
}

/// Looks up open-access copies for documents with a DOI, holding the lock only to read and save.
#[tauri::command]
pub async fn find_open_access(state: State<'_, AppState>) -> AppResult<OaReport> {
    let email = secrets::get(Service::Unpaywall)?
        .ok_or_else(|| AppError::Invalid(tr!("isi email untuk Unpaywall di Pengaturan > Sumber data", "enter an email for Unpaywall in Settings > Data sources").into()))?;
    let todo = {
        let conn = state.db.lock().map_err(|_| AppError::Internal)?;
        oa::pending(&conn, PER_RUN)?
    };
    let (mut checked, mut found, mut error) = (0, 0, None);
    for (i, (id, doi)) in todo.iter().enumerate() {
        if i > 0 {
            http::pause(PAUSE_MS).await;
        }
        let link = match unpaywall::lookup(&state.http, doi, &email).await {
            Ok(l) => l,
            Err(e) => {
                error = Some(e.to_string());
                break;
            }
        };
        found += usize::from(link.url.is_some() || link.pdf_url.is_some());
        let conn = state.db.lock().map_err(|_| AppError::Internal)?;
        oa::store(&conn, *id, &link)?;
        checked += 1;
    }
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    let s = oa::status(&conn)?;
    Ok(OaReport { checked, found, remaining: s.with_doi - s.checked, error })
}

#[tauri::command(async)]
pub fn document_links(state: State<'_, AppState>, ids: Vec<i64>) -> AppResult<Vec<DocLinks>> {
    if ids.len() > MAX_IDS {
        return Err(AppError::Invalid(tr!("terlalu banyak dokumen", "too many documents").into()));
    }
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    oa::links(&conn, &ids)
}

/// Opens the document's PDF, open page or DOI page in the default browser. The frontend
/// sends only the id; the address comes from the database and must be an http(s) link.
#[tauri::command(async)]
pub fn open_document(app: AppHandle, state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let url = {
        let conn = state.db.lock().map_err(|_| AppError::Internal)?;
        oa::target(&conn, id)?
    };
    let url = url.ok_or_else(|| AppError::Invalid(tr!("dokumen ini tidak punya tautan", "this document has no link").into()))?;
    app.opener().open_url(url, None::<&str>).map_err(|_| AppError::Invalid(tr!("browser tidak bisa dibuka", "the browser could not be opened").into()))
}
