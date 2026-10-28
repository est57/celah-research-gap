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
    /// Duplicates whose stored copy gained a reference list, abstract or keywords.
    completed: usize,
}

/// Opens the native file picker from the backend, so the frontend never
/// supplies a file path and needs no filesystem or dialog permission.
/// Returns None when the user cancels the dialog.
#[tauri::command(async)]
pub fn import_file(app: AppHandle, state: State<'_, AppState>) -> AppResult<Option<FileImportReport>> {
    let picked = app
        .dialog()
        .file()
        .set_title(crate::tr!("Impor ekspor Scopus, Web of Science, PubMed, atau RIS", "Import a Scopus, Web of Science, PubMed, or RIS export"))
        .add_filter(crate::tr!("Ekspor bibliografi", "Bibliographic export"), &["bib", "csv", "txt", "ris", "nbib"])
        .blocking_pick_file();
    let Some(picked) = picked else { return Ok(None) };
    let path = picked
        .into_path()
        .map_err(|_| AppError::Invalid(crate::tr!("lokasi file tidak didukung", "unsupported file location").into()))?;

    let text = import::read_text(&path)?;
    let format = import::detect(&path, &text)?;
    let items = import::parse(format, &text);
    if items.is_empty() {
        return Err(AppError::Invalid(crate::tr!("tidak ada rekaman yang bisa dibaca di file ini", "no readable records in this file").into()));
    }

    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    let (inserted, duplicates, completed) = insert::insert_and_fill(&mut conn, &items, format.origin())?;
    Ok(Some(FileImportReport { origin: format.origin(), fetched: items.len(), inserted, duplicates, completed }))
}
