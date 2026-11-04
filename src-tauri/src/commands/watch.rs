//! Topic monitor: saved searches re-run on demand to catch papers published since.

use super::search::run_source;
use crate::analysis::current_year;
use crate::db::watches::{self, Hit, Watch, MAX_WATCHES};
use crate::error::{AppError, AppResult};
use crate::search::{SearchQuery, Source};
use crate::{tr, validate, AppState};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WatchInput {
    query: String,
    year_from: i32,
    doc_type: Option<String>,
    max_results: u32,
    sources: Vec<Source>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchRun {
    /// Documents this run added to the collection.
    pub added: usize,
    pub fetched: usize,
    /// "Source: message" for each source that failed; the others still ran.
    pub errors: Vec<String>,
}

#[tauri::command(async)]
pub fn save_watch(state: State<'_, AppState>, input: WatchInput) -> AppResult<Watch> {
    let sources = validate::sources(&input.sources)?;
    let query = validate::query(&input.query)?;
    let doc_type = validate::doc_type(input.doc_type.as_deref())?;
    let max_results = validate::max_results(input.max_results)?;
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    let (year_from, _) = validate::year_range(input.year_from, current_year(&conn)?.max(input.year_from))?;
    if watches::count(&conn)? >= MAX_WATCHES {
        return Err(AppError::Invalid(tr!("maksimal 50 pantauan; hapus yang tidak dipakai", "at most 50 monitors; delete ones you no longer use").into()));
    }
    let id = watches::create(&conn, &query, &sources, doc_type, year_from, max_results)?;
    watches::get(&conn, id)?.ok_or(AppError::Internal)
}

#[tauri::command(async)]
pub fn list_watches(state: State<'_, AppState>) -> AppResult<Vec<Watch>> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    watches::list(&conn)
}

#[tauri::command(async)]
pub fn delete_watch(state: State<'_, AppState>, id: i64) -> AppResult<bool> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    watches::delete(&conn, id)
}

#[tauri::command(async)]
pub fn watch_hits(state: State<'_, AppState>, id: i64) -> AppResult<Vec<Hit>> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    watches::hits(&conn, id, 200)
}

/// Runs the saved search again, from its first year to this year. Papers already in the
/// collection are skipped as duplicates, so what it adds is what appeared since.
#[tauri::command]
pub async fn run_watch(state: State<'_, AppState>, id: i64) -> AppResult<WatchRun> {
    let (watch, year) = {
        let conn = state.db.lock().map_err(|_| AppError::Internal)?;
        let watch = watches::get(&conn, id)?.ok_or_else(|| AppError::Invalid(tr!("pantauan tidak ditemukan", "monitor not found").into()))?;
        (watch, current_year(&conn)?)
    };
    let query = SearchQuery {
        text: validate::query(&watch.query)?,
        year_from: watch.year_from,
        year_to: year.max(watch.year_from),
        doc_type: validate::doc_type(watch.doc_type.as_deref())?,
        max_results: validate::max_results(watch.max_results)?,
    };
    let (mut added, mut fetched, mut errors) = (Vec::new(), 0, Vec::new());
    for source in validate::sources(&watch.sources)? {
        match run_source(state.inner(), source, &query).await {
            Ok((n, _, _, ids)) => {
                fetched += n;
                added.extend(ids);
            }
            Err(e) => errors.push(format!("{}: {e}", source.label())),
        }
    }
    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    watches::record_run(&mut conn, id, &added)?;
    Ok(WatchRun { added: added.len(), fetched, errors })
}
