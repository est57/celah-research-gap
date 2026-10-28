use crate::db::insert;
use crate::error::{AppError, AppResult};
use crate::openalex::lookup::{self, MAX_DOIS};
use crate::secrets::{self, Service};
use crate::text::normalize_doi;
use crate::AppState;
use serde::Serialize;
use tauri::State;

/// Origin shown in the collection, so these records can be reviewed or removed together.
const ORIGIN: &str = "OpenAlex (rujukan)";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddReport {
    pub requested: usize,
    pub found: usize,
    pub inserted: usize,
    pub duplicates: usize,
    /// Requested DOIs OpenAlex does not know.
    pub not_found: Vec<String>,
}

/// Adds cited works to the collection by DOI (up to 50 per call), fetched from OpenAlex.
#[tauri::command]
pub async fn add_works_by_doi(state: State<'_, AppState>, dois: Vec<String>) -> AppResult<AddReport> {
    let mut wanted: Vec<String> = Vec::new();
    for raw in dois.iter().take(MAX_DOIS + 1) {
        let doi = normalize_doi(raw).ok_or_else(|| AppError::Invalid(crate::tr!("DOI tidak valid", "invalid DOI").into()))?;
        if !wanted.contains(&doi) {
            wanted.push(doi);
        }
    }
    let api_key = tauri::async_runtime::spawn_blocking(|| secrets::get(Service::Openalex)).await.map_err(|_| AppError::Internal)??;
    let items = lookup::by_doi(&state.http, &wanted, api_key.as_deref()).await?;
    let found: Vec<&str> = items.iter().filter_map(|p| p.doi.as_deref()).collect();
    let not_found = wanted.iter().filter(|d| !found.contains(&d.as_str())).cloned().collect();
    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    let (inserted, duplicates) = insert::insert_many(&mut conn, &items, ORIGIN)?;
    Ok(AddReport { requested: wanted.len(), found: items.len(), inserted, duplicates, not_found })
}
