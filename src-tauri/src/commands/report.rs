use crate::error::{AppError, AppResult};
use crate::report::summary::{self, Summary, SummaryParams};
use crate::report::cite::Style;
use crate::report::{self, docx, Report, ReportParams};
use crate::AppState;
use super::with_analysis;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

#[tauri::command(async)]
pub fn report(state: State<'_, AppState>, params: ReportParams) -> AppResult<Report> {
    with_analysis(&state, |conn| in_report_language(&params.language, || report::build(conn, &params)))
}

/// Generated text the report reuses (gap titles, field names) follows the report's language.
fn in_report_language<T>(language: &str, f: impl FnOnce() -> T) -> T {
    crate::lang::with_english(language == "en", f)
}

/// Draft methods and results paragraphs from the collection's figures.
#[tauri::command(async)]
pub fn report_summary(state: State<'_, AppState>, params: SummaryParams) -> AppResult<Summary> {
    with_analysis(&state, |conn| in_report_language(&params.language, || summary::build(conn, &params)))
}

/// Saves the report in `format` ("markdown", "docx", "bibtex" or "ris") through the native
/// save dialog opened from the backend, so the frontend never supplies a path.
/// Returns the chosen file name, or None when the dialog is cancelled.
#[tauri::command(async)]
pub fn export_report(app: AppHandle, state: State<'_, AppState>, params: ReportParams, format: String) -> AppResult<Option<String>> {
    let (ext, label) = format_of(&format)?;
    let built = with_analysis(&state, |conn| in_report_language(&params.language, || report::build(conn, &params)))?;
    let content = match ext {
        "md" => built.markdown.into_bytes(),
        "docx" => docx::render(&built.title, &built.intro, &built.paragraphs, &built.references_title, &built.references, style_of(&params.style))?,
        "bib" => built.bibtex.into_bytes(),
        _ => built.ris.into_bytes(),
    };
    // The file name follows the report's own language.
    let stem = if params.language == "en" { "celah-research-gaps" } else { "celah-penelitian" };
    save(&app, content, ext, label, stem)
}

/// Saves the methods and results draft, as `export_report` does for the gap draft.
#[tauri::command(async)]
pub fn export_summary(app: AppHandle, state: State<'_, AppState>, params: SummaryParams, format: String) -> AppResult<Option<String>> {
    let (ext, label) = format_of(&format)?;
    let built = with_analysis(&state, |conn| in_report_language(&params.language, || summary::build(conn, &params)))?;
    let content = match ext {
        "md" => built.markdown.into_bytes(),
        "docx" => docx::render(&built.title, &built.intro, &built.paragraphs, &built.references_title, &built.references, style_of(&params.style))?,
        "bib" => built.bibtex.into_bytes(),
        _ => built.ris.into_bytes(),
    };
    let stem = if params.language == "en" { "bibliometric-methods-results" } else { "metode-hasil-bibliometrik" };
    save(&app, content, ext, label, stem)
}

/// The citation style was already checked when the report was built.
fn style_of(style: &str) -> Style {
    if style == "ieee" { Style::Ieee } else { Style::Apa }
}

fn format_of(format: &str) -> AppResult<(&'static str, &'static str)> {
    match format {
        "markdown" => Ok(("md", "Markdown")),
        "docx" => Ok(("docx", "Word")),
        "bibtex" => Ok(("bib", "BibTeX")),
        "ris" => Ok(("ris", "RIS")),
        _ => Err(AppError::Invalid(crate::tr!("format ekspor tidak dikenal", "unknown export format").into())),
    }
}

/// Native save dialog opened from the backend, so the frontend never supplies a path.
fn save(app: &AppHandle, content: Vec<u8>, ext: &str, label: &str, stem: &str) -> AppResult<Option<String>> {
    let picked = app
        .dialog()
        .file()
        .set_title(crate::tr!("Simpan laporan", "Save report"))
        .set_file_name(format!("{stem}.{ext}"))
        .add_filter(label, &[ext])
        .blocking_save_file();
    let Some(picked) = picked else { return Ok(None) };
    let mut path = picked.into_path().map_err(|_| AppError::Invalid(crate::tr!("lokasi file tidak didukung", "unsupported file location").into()))?;
    if path.extension().and_then(|e| e.to_str()) != Some(ext) {
        path.set_extension(ext);
    }
    std::fs::write(&path, content).map_err(|_| AppError::Invalid(crate::tr!("file tidak bisa disimpan di lokasi itu", "the file cannot be saved in that location").into()))?;
    Ok(path.file_name().and_then(|n| n.to_str()).map(str::to_string))
}
