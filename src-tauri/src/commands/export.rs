use super::with_analysis;
use crate::error::{AppError, AppResult};
use crate::{export, AppState};
use serde::Serialize;
use std::path::PathBuf;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

#[derive(Serialize)]
pub struct Saved {
    pub file: String,
    pub count: usize,
}

/// Native save dialog opened from the backend, so the frontend never supplies a path.
/// The chosen path always ends in `ext`.
fn ask_path(app: &AppHandle, title: &str, name: String, label: &str, ext: &str) -> AppResult<Option<PathBuf>> {
    let picked = app.dialog().file().set_title(title).set_file_name(name).add_filter(label, &[ext]).blocking_save_file();
    let Some(picked) = picked else { return Ok(None) };
    let mut path = picked.into_path().map_err(|_| AppError::Invalid("lokasi file tidak didukung".into()))?;
    if path.extension().and_then(|e| e.to_str()) != Some(ext) {
        path.set_extension(ext);
    }
    Ok(Some(path))
}

fn file_name(path: &std::path::Path) -> String {
    path.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string()
}

/// `format` is "csv", "bibtex" or "ris"; `scope` is "all" (whole collection) or
/// "filtered" (the documents the analyses use). Resolves to None when cancelled.
#[tauri::command(async)]
pub fn export_collection(app: AppHandle, state: State<'_, AppState>, format: String, scope: String) -> AppResult<Option<Saved>> {
    let (bytes, ext, count) = match scope.as_str() {
        "all" => {
            let conn = state.db.lock().map_err(|_| AppError::Internal)?;
            export::build(&conn, &format)?
        }
        "filtered" => with_analysis(&state, |conn| export::build(conn, &format))?,
        _ => return Err(AppError::Invalid("cakupan ekspor tidak dikenal".into())),
    };
    let label = match ext { "csv" => "CSV", "bib" => "BibTeX", _ => "RIS" };
    let Some(path) = ask_path(&app, "Ekspor koleksi", format!("celah-koleksi.{ext}"), label, ext)? else { return Ok(None) };
    std::fs::write(&path, bytes).map_err(|_| AppError::Invalid("file tidak bisa disimpan di lokasi itu".into()))?;
    Ok(Some(Saved { file: file_name(&path), count }))
}

/// A full copy of the database (collection, reviews, checks, filter) to a chosen file.
#[tauri::command(async)]
pub fn backup_database(app: AppHandle, state: State<'_, AppState>) -> AppResult<Option<String>> {
    let date: String = {
        let conn = state.db.lock().map_err(|_| AppError::Internal)?;
        conn.query_row("SELECT date('now', 'localtime')", [], |r| r.get(0))?
    };
    let Some(path) = ask_path(&app, "Cadangkan database", format!("celah-cadangan-{date}.sqlite"), "Database SQLite", "sqlite")? else {
        return Ok(None);
    };
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    export::backup(&conn, &path)?;
    Ok(Some(file_name(&path)))
}

/// Saves a chart image made by the UI. `kind` is "svg" or "png"; the content is checked
/// before the save dialog opens, and the file name is reduced to safe characters.
#[tauri::command(async)]
pub fn save_image(app: AppHandle, kind: String, data: Vec<u8>, name: String) -> AppResult<Option<String>> {
    let ext = export::image::check(&kind, &data)?;
    let label = if ext == "png" { "Gambar PNG" } else { "Gambar SVG" };
    let stem = export::image::file_stem(&name);
    let Some(path) = ask_path(&app, "Simpan grafik", format!("{stem}.{ext}"), label, ext)? else { return Ok(None) };
    std::fs::write(&path, data).map_err(|_| AppError::Invalid("gambar tidak bisa disimpan di lokasi itu".into()))?;
    Ok(Some(file_name(&path)))
}
