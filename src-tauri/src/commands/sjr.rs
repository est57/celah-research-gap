use super::with_analysis;
use crate::analysis::quartiles::{self, Quartiles};
use crate::db::sjr::{self, SjrStatus};
use crate::error::{AppError, AppResult};
use crate::import;
use crate::AppState;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

/// Opens the native file picker from the backend and replaces the stored SJR list with
/// the chosen Scimago download. Returns None when the dialog is cancelled.
#[tauri::command(async)]
pub fn import_sjr(app: AppHandle, state: State<'_, AppState>) -> AppResult<Option<SjrStatus>> {
    let picked = app
        .dialog()
        .file()
        .set_title("Impor data SJR dari scimagojr.com")
        .add_filter("Data SJR (CSV)", &["csv"])
        .blocking_pick_file();
    let Some(picked) = picked else { return Ok(None) };
    let path = picked.into_path().map_err(|_| AppError::Invalid("lokasi file tidak didukung".into()))?;
    let (journals, year) = import::sjr::parse(&import::read_text(&path)?)?;
    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    sjr::replace(&mut conn, &journals, year).map(Some)
}

#[tauri::command(async)]
pub fn clear_sjr(state: State<'_, AppState>) -> AppResult<SjrStatus> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    sjr::clear(&conn)?;
    sjr::status(&conn)
}

/// Documents per journal quartile in the (filtered) collection.
#[tauri::command(async)]
pub fn quartiles(state: State<'_, AppState>) -> AppResult<Quartiles> {
    with_analysis(&state, quartiles::compute)
}
