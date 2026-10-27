use crate::db::filter::{self, CollectionFilter};
use crate::db::filter_options::{self, FilterOptions};
use crate::error::{AppError, AppResult};
use crate::AppState;
use tauri::State;

#[tauri::command(async)]
pub fn filter_options(state: State<'_, AppState>) -> AppResult<FilterOptions> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    filter_options::options(&conn)
}

/// Documents a filter would keep, without saving it.
#[tauri::command(async)]
pub fn preview_filter(state: State<'_, AppState>, filter: CollectionFilter) -> AppResult<i64> {
    filter.validate()?;
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    filter::matching(&conn, &filter)
}

/// Saves the filter for every analysis; an empty filter removes it.
#[tauri::command(async)]
pub fn set_collection_filter(state: State<'_, AppState>, filter: CollectionFilter) -> AppResult<FilterOptions> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    filter::save(&conn, &filter)?;
    filter_options::options(&conn)
}
