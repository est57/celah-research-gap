use crate::db::{self, analysis_conn::AnalysisDb};
use crate::error::{AppError, AppResult};
use crate::projects::{self, ProjectInfo};
use crate::AppState;
use tauri::State;

fn listing(state: &AppState) -> AppResult<Vec<ProjectInfo>> {
    Ok(projects::list(&state.data_dir, &projects::load(&state.data_dir)))
}

#[tauri::command(async)]
pub fn list_projects(state: State<'_, AppState>) -> AppResult<Vec<ProjectInfo>> {
    listing(&state)
}

#[tauri::command(async)]
pub fn create_project(state: State<'_, AppState>, name: String) -> AppResult<Vec<ProjectInfo>> {
    let mut r = projects::load(&state.data_dir);
    projects::create(&state.data_dir, &mut r, &name)?;
    listing(&state)
}

#[tauri::command(async)]
pub fn rename_project(state: State<'_, AppState>, id: String, name: String) -> AppResult<Vec<ProjectInfo>> {
    let mut r = projects::load(&state.data_dir);
    projects::rename(&state.data_dir, &mut r, &id, &name)?;
    listing(&state)
}

#[tauri::command(async)]
pub fn delete_project(state: State<'_, AppState>, id: String) -> AppResult<Vec<ProjectInfo>> {
    let mut r = projects::load(&state.data_dir);
    projects::delete(&state.data_dir, &mut r, &id)?;
    listing(&state)
}

/// Opens another project: both connections are replaced while their locks are held,
/// so no command runs against a half-switched state.
#[tauri::command(async)]
pub fn switch_project(state: State<'_, AppState>, id: String) -> AppResult<Vec<ProjectInfo>> {
    let mut r = projects::load(&state.data_dir);
    if !r.names.contains_key(&id) {
        return Err(AppError::Invalid(crate::tr!("proyek tidak dikenal", "unknown project").into()));
    }
    let path = projects::path_of(&state.data_dir, &id)?;
    let conn = db::open(&path)?;
    let analysis = AnalysisDb::open(&path)?;
    {
        let mut db = state.db.lock().map_err(|_| AppError::Internal)?;
        let mut an = state.analysis.lock().map_err(|_| AppError::Internal)?;
        *db = conn;
        *an = analysis;
        *state.db_path.lock().map_err(|_| AppError::Internal)? = path;
        *state.pending_restore.lock().map_err(|_| AppError::Internal)? = None;
    }
    r.current = id;
    projects::save(&state.data_dir, &r)?;
    listing(&state)
}
