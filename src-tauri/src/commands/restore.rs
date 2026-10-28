use crate::error::{AppError, AppResult};
use crate::export::{self, restore::BackupInfo};
use crate::AppState;
use serde::Serialize;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_dialog::DialogExt;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Restored {
    pub documents: i64,
    /// Name of the automatic backup of the data that was replaced.
    pub safety_file: String,
}

/// Step 1: the user picks a backup in the native dialog; it is checked and remembered
/// in the backend (the frontend never holds the path). None when cancelled.
#[tauri::command(async)]
pub fn choose_backup(app: AppHandle, state: State<'_, AppState>) -> AppResult<Option<BackupInfo>> {
    let picked = app.dialog().file().set_title(crate::tr!("Pilih file cadangan Celah", "Choose a Celah backup file")).add_filter(crate::tr!("Database SQLite", "SQLite database"), &["sqlite", "db"]).blocking_pick_file();
    let Some(picked) = picked else { return Ok(None) };
    let path = picked.into_path().map_err(|_| AppError::Invalid(crate::tr!("lokasi file tidak didukung", "unsupported file location").into()))?;
    let live = state.db_path.lock().map_err(|_| AppError::Internal)?.clone();
    let same = std::fs::canonicalize(&path).ok() == std::fs::canonicalize(&live).ok();
    if same {
        return Err(AppError::Invalid(crate::tr!("file itu adalah database yang sedang dipakai", "that file is the database currently in use").into()));
    }
    let info = export::restore::inspect(&path)?;
    *state.pending_restore.lock().map_err(|_| AppError::Internal)? = Some(path);
    Ok(Some(info))
}

#[tauri::command(async)]
pub fn cancel_restore(state: State<'_, AppState>) -> AppResult<()> {
    *state.pending_restore.lock().map_err(|_| AppError::Internal)? = None;
    Ok(())
}

/// Step 2, after the user confirms: saves the current data to an automatic backup,
/// then replaces it with the chosen backup.
#[tauri::command(async)]
pub fn restore_backup(app: AppHandle, state: State<'_, AppState>) -> AppResult<Restored> {
    let from = state.pending_restore.lock().map_err(|_| AppError::Internal)?.take()
        .ok_or_else(|| AppError::Invalid(crate::tr!("pilih file cadangan dulu", "choose a backup file first").into()))?;
    let dir = app.path().app_data_dir().map_err(|_| AppError::Internal)?.join("cadangan-otomatis");
    std::fs::create_dir_all(&dir).map_err(|_| AppError::Internal)?;
    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    let stamp: String = conn.query_row("SELECT strftime('%Y%m%d-%H%M%S', 'now', 'localtime')", [], |r| r.get(0))?;
    let safety = dir.join(format!("celah-sebelum-pulih-{stamp}.sqlite"));
    export::backup(&conn, &safety)?;
    export::restore::restore(&mut conn, &from)?;
    let documents = conn.query_row("SELECT COUNT(*) FROM papers", [], |r| r.get(0))?;
    let safety_file = safety.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
    Ok(Restored { documents, safety_file })
}
