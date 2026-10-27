use crate::db::insert;
use crate::error::{AppError, AppResult};
use crate::import;
use crate::AppState;
use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

#[derive(Serialize)]
pub struct FileImportReport {
    origin: &'static str,
    fetched: usize,
    inserted: usize,
    duplicates: usize,
}

/// Opens the native file picker from the backend, so the frontend never
/// supplies a file path and needs no filesystem or dialog permission.
/// Returns None when the user cancels the dialog.
#[tauri::command(async)]
pub fn import_file(app: AppHandle, state: State<'_, AppState>) -> AppResult<Option<FileImportReport>> {
    let picked = app
        .dialog()
        .file()
        .set_title("Impor ekspor Scopus atau Web of Science")
        .add_filter("Ekspor bibliografi", &["bib", "csv", "txt"])
        .blocking_pick_file();
    let Some(picked) = picked else { return Ok(None) };
    let path = picked
        .into_path()
        .map_err(|_| AppError::Invalid("lokasi file tidak didukung".into()))?;

    let text = import::read_text(&path)?;
    let format = import::detect(&path, &text)?;
    let items = import::parse(format, &text);
    if items.is_empty() {
        return Err(AppError::Invalid("tidak ada rekaman yang bisa dibaca di file ini".into()));
    }

    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    let (inserted, duplicates) = insert::insert_many(&mut conn, &items, format.origin())?;
    Ok(Some(FileImportReport { origin: format.origin(), fetched: items.len(), inserted, duplicates }))
}
