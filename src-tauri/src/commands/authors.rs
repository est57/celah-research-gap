use crate::db::author_merge::{self, Merge};
use crate::db::author_suggest::{self, Suggestion};
use crate::error::{AppError, AppResult};
use crate::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
pub struct AuthorNames {
    pub suggestions: Vec<Suggestion>,
    pub merges: Vec<Merge>,
}

/// Possible duplicates and existing merges, over the whole collection.
#[tauri::command(async)]
pub fn author_names(state: State<'_, AppState>) -> AppResult<AuthorNames> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    Ok(AuthorNames { suggestions: author_suggest::suggest(&conn)?, merges: author_merge::list(&conn)? })
}

/// Counts every key in `keys` as `target`; returns the author rows changed.
#[tauri::command(async)]
pub fn merge_authors(state: State<'_, AppState>, keys: Vec<String>, target: String) -> AppResult<usize> {
    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    author_merge::merge(&mut conn, &keys, &target)
}

/// Undoes one merge.
#[tauri::command(async)]
pub fn split_author(state: State<'_, AppState>, from_key: String) -> AppResult<usize> {
    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    author_merge::split(&mut conn, &from_key)
}
