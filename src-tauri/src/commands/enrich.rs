use crate::db::enrich::{self, Pending};
use crate::error::{AppError, AppResult};
use crate::openalex::refs::{self, BATCH};
use crate::secrets::{self, Service};
use crate::{http, AppState};
use serde::Serialize;
use tauri::State;

/// Documents looked up per run, about 40 requests.
const PER_RUN: i64 = 2000;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichReport {
    pub checked: usize,
    /// Documents that now have cited works.
    pub gained: usize,
    /// Documents still waiting for a lookup.
    pub remaining: i64,
    /// Set when a request failed; batches before it are saved and the next run continues.
    pub error: Option<String>,
}

#[tauri::command(async)]
pub fn references_pending(state: State<'_, AppState>) -> AppResult<i64> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    enrich::remaining(&conn)
}

/// Fetches cited works from OpenAlex for documents that have none yet, in batches,
/// holding the database lock only while reading the list and saving each batch.
#[tauri::command]
pub async fn complete_references(state: State<'_, AppState>) -> AppResult<EnrichReport> {
    let todo = {
        let conn = state.db.lock().map_err(|_| AppError::Internal)?;
        enrich::pending(&conn, PER_RUN)?
    };
    let api_key = tauri::async_runtime::spawn_blocking(|| secrets::get(Service::Openalex))
        .await
        .map_err(|_| AppError::Internal)??;
    let mut batches: Vec<(&'static str, Pending)> = Vec::new();
    let ids: Vec<(String, i64)> = todo.by_id.into_iter().collect();
    for chunk in ids.chunks(BATCH) {
        batches.push(("ids.openalex", Pending { by_id: chunk.iter().cloned().collect(), ..Default::default() }));
    }
    let dois: Vec<(String, i64)> = todo.by_doi.into_iter().collect();
    for chunk in dois.chunks(BATCH) {
        batches.push(("doi", Pending { by_doi: chunk.iter().cloned().collect(), ..Default::default() }));
    }

    let (mut checked, mut gained, mut error) = (0, 0, None);
    for (i, (field, batch)) in batches.iter().enumerate() {
        if i > 0 {
            http::pause(200).await;
        }
        let values: Vec<String> = batch.by_id.keys().chain(batch.by_doi.keys()).cloned().collect();
        let records = match refs::fetch(&state.http, field, &values, api_key.as_deref()).await {
            Ok(r) => r,
            Err(e) => {
                error = Some(e.to_string());
                break;
            }
        };
        let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
        gained += enrich::save(&mut conn, batch, &records)?;
        checked += values.len();
    }
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    Ok(EnrichReport { checked, gained, remaining: enrich::remaining(&conn)?, error })
}
