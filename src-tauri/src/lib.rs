mod analysis;
mod arxiv;
mod commands;
mod crossref;
mod db;
mod error;
mod europepmc;
mod export;
mod geo;
mod http;
mod import;
#[cfg(test)]
mod live_authors;
#[cfg(test)]
mod live_check;
#[cfg(test)]
mod live_coupling;
#[cfg(test)]
mod live_duplicates;
#[cfg(test)]
mod live_evolution;
#[cfg(test)]
mod live_hist;
#[cfg(test)]
mod live_idea;
#[cfg(test)]
mod live_rpys;
#[cfg(test)]
mod live_scale;
#[cfg(test)]
mod live_words;
mod openalex;
mod projects;
mod report;
mod pubmed;
mod search;
mod secrets;
mod semantic_scholar;
mod text;
mod validate;
mod verify;

use std::sync::Mutex;
use tauri::Manager;

pub struct AppState {
    /// Read-write connection to the whole collection: imports, reviews, merges, settings.
    pub db: Mutex<rusqlite::Connection>,
    /// Read-only connection that sees only the filtered collection; used by every analysis.
    pub analysis: Mutex<db::analysis_conn::AnalysisDb>,
    /// Location of the open project's database, so a restore never reads from it.
    pub db_path: Mutex<std::path::PathBuf>,
    /// App data folder holding the projects and their registry.
    pub data_dir: std::path::PathBuf,
    /// Backup chosen in the first restore step, waiting for the user's confirmation.
    pub pending_restore: Mutex<Option<std::path::PathBuf>>,
    pub http: reqwest::Client,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Used only from Rust (commands::import); no dialog permission is granted to the frontend.
        .plugin(tauri_plugin_dialog::init())
        // Also used only from Rust (commands::update), so no updater permission is granted either.
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            // The project open last time; a missing file falls back to the main project.
            let registry = projects::load(&dir);
            let path = projects::path_of(&dir, &registry.current)
                .ok()
                .filter(|p| p.exists())
                .unwrap_or_else(|| dir.join("celah.sqlite"));
            // The main connection runs the migrations before the analysis connection opens.
            let conn = db::open(&path)?;
            let analysis = db::analysis_conn::AnalysisDb::open(&path)?;
            let http = http::client()?;
            app.manage(AppState {
                db: Mutex::new(conn),
                analysis: Mutex::new(analysis),
                db_path: Mutex::new(path),
                data_dir: dir,
                pending_restore: Mutex::new(None),
                http,
            });
            Ok(())
        })
        .invoke_handler(commands::handler())
        .run(tauri::generate_context!())
        .expect("failed to start Celah");
}
