use crate::db::synonyms::{self, suggest, Synonym};
use crate::error::{AppError, AppResult};
use crate::AppState;
use serde::Serialize;
use tauri::State;

/// Keywords offered when merging by hand.
const TOP_TERMS: usize = 500;

#[derive(Serialize)]
pub struct KeywordSynonyms {
    pub suggestions: Vec<suggest::KeywordSuggestion>,
    pub synonyms: Vec<Synonym>,
    pub terms: Vec<suggest::Term>,
}

/// Suggested synonym groups, existing synonyms and the most used keywords (whole collection).
#[tauri::command(async)]
pub fn keyword_synonyms(state: State<'_, AppState>) -> AppResult<KeywordSynonyms> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    Ok(KeywordSynonyms { suggestions: suggest::suggest(&conn)?, synonyms: synonyms::list(&conn)?, terms: suggest::top_terms(&conn, TOP_TERMS)? })
}

/// Every keyword in `labels` counts as `target` in analyses from now on.
#[tauri::command(async)]
pub fn merge_keywords(state: State<'_, AppState>, labels: Vec<String>, target: String) -> AppResult<usize> {
    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    synonyms::merge(&mut conn, &labels, &target)
}

#[tauri::command(async)]
pub fn remove_synonym(state: State<'_, AppState>, from_key: String) -> AppResult<()> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    synonyms::remove(&conn, &from_key)
}
