use crate::error::{AppError, AppResult};
use crate::report::summary::{self, Summary, SummaryParams};
use crate::report::{self, Report, ReportParams};
use crate::AppState;
use super::with_analysis;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

#[tauri::command(async)]
pub fn report(state: State<'_, AppState>, params: ReportParams) -> AppResult<Report> {
    with_analysis(&state, |conn| report::build(conn, &params))
}

/// Draft methods and results paragraphs from the collection's figures.
#[tauri::command(async)]
pub fn report_summary(state: State<'_, AppState>, params: SummaryParams) -> AppResult<Summary> {
    with_analysis(&state, |conn| summary::build(conn, &params))
}

/// Saves the report in `format` ("markdown", "bibtex" or "ris") through the native
/// save dialog opened from the backend, so the frontend never supplies a path.
/// Returns the chosen file name, or None when the dialog is cancelled.
#[tauri::command(async)]
pub fn export_report(app: AppHandle, state: State<'_, AppState>, params: ReportParams, format: String) -> AppResult<Option<String>> {
    let (ext, label) = format_of(&format)?;
    let built = with_analysis(&state, |conn| report::build(conn, &params))?;
    let content = match ext {
        "md" => built.markdown,
        "bib" => built.bibtex,
        _ => built.ris,
    };
    save(&app, content, ext, label, "celah-penelitian")
}

/// Saves the methods and results draft, as `export_report` does for the gap draft.
#[tauri::command(async)]
pub fn export_summary(app: AppHandle, state: State<'_, AppState>, params: SummaryParams, format: String) -> AppResult<Option<String>> {
    let (ext, label) = format_of(&format)?;
    let built = with_analysis(&state, |conn| summary::build(conn, &params))?;
    let content = match ext {
        "md" => built.markdown,
        "bib" => built.bibtex,
        _ => built.ris,
    };
    save(&app, content, ext, label, "metode-hasil-bibliometrik")
}

fn format_of(format: &str) -> AppResult<(&'static str, &'static str)> {
    match format {
        "markdown" => Ok(("md", "Markdown")),
        "bibtex" => Ok(("bib", "BibTeX")),
        "ris" => Ok(("ris", "RIS")),
        _ => Err(AppError::Invalid("format ekspor tidak dikenal".into())),
    }
}

/// Native save dialog opened from the backend, so the frontend never supplies a path.
fn save(app: &AppHandle, content: String, ext: &str, label: &str, stem: &str) -> AppResult<Option<String>> {
    let picked = app
        .dialog()
        .file()
        .set_title("Simpan laporan")
        .set_file_name(format!("{stem}.{ext}"))
        .add_filter(label, &[ext])
        .blocking_save_file();
    let Some(picked) = picked else { return Ok(None) };
    let mut path = picked.into_path().map_err(|_| AppError::Invalid("lokasi file tidak didukung".into()))?;
    if path.extension().and_then(|e| e.to_str()) != Some(ext) {
        path.set_extension(ext);
    }
    std::fs::write(&path, content).map_err(|_| AppError::Invalid("file tidak bisa disimpan di lokasi itu".into()))?;
    Ok(path.file_name().and_then(|n| n.to_str()).map(str::to_string))
}
