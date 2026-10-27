use crate::db::duplicates::{self, DuplicateList};
use crate::error::{AppError, AppResult};
use crate::AppState;
use tauri::State;

#[tauri::command(async)]
pub fn duplicates(state: State<'_, AppState>) -> AppResult<DuplicateList> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    duplicates::list(&conn)
}

/// Keeps `keep`, fills its empty fields from `drop`, then removes `drop`.
#[tauri::command(async)]
pub fn merge_duplicate(state: State<'_, AppState>, keep: i64, drop: i64) -> AppResult<()> {
    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    duplicates::merge(&mut conn, keep, drop)
}

#[tauri::command(async)]
pub fn dismiss_duplicate(state: State<'_, AppState>, a: i64, b: i64) -> AppResult<()> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    duplicates::dismiss(&conn, a, b)
}
