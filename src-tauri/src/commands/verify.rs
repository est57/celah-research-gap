use crate::analysis::{current_year, gaps};
use crate::db::checks;
use crate::error::{AppError, AppResult};
use crate::search::SearchQuery;
use crate::secrets::{self, Service};
use crate::{http, openalex, verify, AppState};
use serde::Serialize;
use tauri::State;

/// Gaps checked per run, to keep runs short and within API quotas.
const MAX_TARGETS: usize = 20;
/// Recent papers fetched per gap.
const PER_GAP: u32 = 50;
/// Years back from the current one that count as "recent".
const RECENT_YEARS: i32 = 2;

#[derive(Serialize)]
pub struct VerifyReport {
    key: String,
    found: usize,
    matching: usize,
    verdict: &'static str,
    error: Option<String>,
}

/// Verifies the given gaps, or every accepted gap when `keys` is empty.
#[tauri::command]
pub async fn verify_gaps(state: State<'_, AppState>, keys: Vec<String>) -> AppResult<Vec<VerifyReport>> {
    if keys.len() > MAX_TARGETS || keys.iter().any(|k| k.len() > 300) {
        return Err(AppError::Invalid("daftar celah terlalu panjang".into()));
    }
    // Gaps come from the filtered collection; "already in the collection" below checks the whole one.
    let (targets, year) = super::with_analysis(&state, |conn| {
        let list = gaps::compute(conn)?;
        let chosen: Vec<(String, Vec<String>)> = list
            .gaps
            .into_iter()
            .filter(|g| if keys.is_empty() { g.status == "accepted" } else { keys.contains(&g.key) })
            .map(|g| (g.key, g.terms))
            .take(MAX_TARGETS)
            .collect();
        Ok((chosen, current_year(conn)?))
    })?;
    if targets.is_empty() {
        return Err(AppError::Invalid("belum ada celah yang diterima untuk diverifikasi".into()));
    }
    let api_key = tauri::async_runtime::spawn_blocking(|| secrets::get(Service::Openalex))
        .await
        .map_err(|_| AppError::Internal)??;

    let mut reports = Vec::new();
    for (i, (key, terms)) in targets.iter().enumerate() {
        if i > 0 {
            http::pause(300).await;
        }
        let query = verify::query_text(terms);
        if query.chars().count() < 2 {
            reports.push(VerifyReport { key: key.clone(), found: 0, matching: 0, verdict: "", error: Some("celah ini tidak punya kata kunci untuk dicari".into()) });
            continue;
        }
        let search = SearchQuery { text: query.clone(), year_from: year - RECENT_YEARS, year_to: year, doc_type: None, max_results: PER_GAP };
        let report = match openalex::search(&state.http, &search, api_key.as_deref()).await {
            Ok(found) => {
                let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
                let papers = verify::candidates(&conn, &found, terms)?;
                checks::save(&mut conn, key, &query, &papers)?;
                let matching = papers.iter().filter(|p| p.matches).count();
                VerifyReport { key: key.clone(), found: papers.len(), matching, verdict: checks::verdict(matching as i64), error: None }
            }
            Err(err) => VerifyReport { key: key.clone(), found: 0, matching: 0, verdict: "", error: Some(err.to_string()) },
        };
        reports.push(report);
    }
    Ok(reports)
}
