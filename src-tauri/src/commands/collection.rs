use crate::db::{papers, quality, quick_search, remove, stats};
use crate::error::{AppError, AppResult};
use crate::{validate, AppState};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ListInput {
    filter: Option<String>,
    limit: u32,
    offset: u32,
}

#[derive(Serialize)]
pub struct PaperPage {
    items: Vec<papers::PaperRow>,
    total: i64,
}

#[tauri::command(async)]
pub fn list_papers(state: State<'_, AppState>, input: ListInput) -> AppResult<PaperPage> {
    let filter = validate::filter(input.filter.as_deref())?;
    let (limit, offset) = validate::page(input.limit, input.offset)?;
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    let (items, total) = papers::list(&conn, filter.as_deref(), limit, offset)?;
    Ok(PaperPage { items, total })
}

#[tauri::command(async)]
pub fn collection_stats(state: State<'_, AppState>) -> AppResult<stats::CollectionStats> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    stats::collection(&conn)
}

#[tauri::command(async)]
pub fn metadata_quality(state: State<'_, AppState>) -> AppResult<quality::MetadataQuality> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    quality::quality(&conn)
}

/// Title-bar search over the whole collection (not the analysis filter).
#[tauri::command(async)]
pub fn quick_search(state: State<'_, AppState>, query: String) -> AppResult<quick_search::QuickResults> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    quick_search::search(&conn, &query)
}

/// Removes the selected documents from the collection; returns how many were removed.
#[tauri::command(async)]
pub fn delete_papers(state: State<'_, AppState>, ids: Vec<i64>) -> AppResult<usize> {
    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    remove::by_ids(&mut conn, &ids)
}

/// Removes every document imported from one source, e.g. "Crossref".
#[tauri::command(async)]
pub fn delete_origin(state: State<'_, AppState>, origin: String) -> AppResult<usize> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    remove::by_origin(&conn, &origin)
}
