//! App updates from the signed release feed set in tauri.conf.json (plugins.updater).
//! The plugin verifies every download against the public key before installing it.

use crate::error::{AppError, AppResult};
use crate::text::clean;
use serde::Serialize;
use std::time::Duration;
use tauri::AppHandle;
use tauri_plugin_updater::{Update, UpdaterExt};

const TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    version: String,
    current: String,
    /// Release notes, shown as plain text.
    notes: Option<String>,
}

fn failed(err: tauri_plugin_updater::Error) -> AppError {
    eprintln!("[update] {err}");
    AppError::Http("server pembaruan tidak bisa dihubungi atau berkasnya tidak valid".into())
}

async fn latest(app: &AppHandle) -> AppResult<Option<Update>> {
    app.updater_builder().timeout(TIMEOUT).build().map_err(failed)?.check().await.map_err(failed)
}

/// The newer version on the release feed, or None when this one is current.
#[tauri::command]
pub async fn check_update(app: AppHandle) -> AppResult<Option<UpdateInfo>> {
    Ok(latest(&app).await?.map(|u| UpdateInfo {
        version: clean(&u.version, 40),
        current: clean(&u.current_version, 40),
        notes: u.body.as_deref().map(|b| clean(b, 4_000)).filter(|b| !b.is_empty()),
    }))
}

/// Downloads, verifies and installs the newer version, then restarts the app.
#[tauri::command]
pub async fn install_update(app: AppHandle) -> AppResult<()> {
    let update = latest(&app).await?.ok_or_else(|| AppError::Invalid("tidak ada versi baru".into()))?;
    update.download_and_install(|_, _| {}, || {}).await.map_err(failed)?;
    app.restart()
}
