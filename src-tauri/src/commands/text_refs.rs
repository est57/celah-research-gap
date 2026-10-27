use crate::crossref::refs::{self, BATCH};
use crate::db::text_refs;
use crate::error::{AppError, AppResult};
use crate::{http, AppState};
use serde::Serialize;
use tauri::State;

/// Documents looked up per run: 30 requests of 20 DOIs.
const PER_RUN: i64 = 600;
/// Crossref's public pool is shared; pause between requests.
const PAUSE_MS: u64 = 300;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextRefsReport {
    pub checked: usize,
    /// Documents that now have a reference list.
    pub gained: usize,
    pub remaining: i64,
    /// Set when a request failed; batches before it are saved.
    pub error: Option<String>,
}

#[tauri::command(async)]
pub fn text_references_pending(state: State<'_, AppState>) -> AppResult<i64> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    text_refs::remaining(&conn)
}

/// Fills in reference lists from Crossref for documents with a DOI and no references,
/// holding the database lock only while reading the list and saving each batch.
#[tauri::command]
pub async fn complete_text_references(state: State<'_, AppState>) -> AppResult<TextRefsReport> {
    let todo = {
        let conn = state.db.lock().map_err(|_| AppError::Internal)?;
        text_refs::pending(&conn, PER_RUN)?
    };
    let (mut checked, mut gained, mut error) = (0, 0, None);
    for (i, batch) in todo.chunks(BATCH).enumerate() {
        if i > 0 {
            http::pause(PAUSE_MS).await;
        }
        let dois: Vec<String> = batch.iter().map(|(_, d)| d.clone()).collect();
        let found = match refs::by_doi(&state.http, &dois).await {
            Ok(f) => f,
            Err(e) => {
                error = Some(e.to_string());
                break;
            }
        };
        let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
        for (id, doi) in batch {
            let list = found.iter().find(|(d, _)| d == doi).map(|(_, r)| r.as_slice()).unwrap_or(&[]);
            gained += usize::from(text_refs::store(&mut conn, *id, list)?);
            checked += 1;
        }
    }
    let remaining = {
        let conn = state.db.lock().map_err(|_| AppError::Internal)?;
        text_refs::remaining(&conn)?
    };
    Ok(TextRefsReport { checked, gained, remaining, error })
}
