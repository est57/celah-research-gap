use crate::db::insert;
use crate::error::{AppError, AppResult};
use crate::search::{SearchQuery, Source};
use crate::{arxiv, crossref, europepmc, openalex, pubmed, secrets, semantic_scholar, validate, AppState};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SearchInput {
    query: String,
    year_from: i32,
    year_to: i32,
    doc_type: Option<String>,
    max_results: u32,
    sources: Vec<Source>,
}

/// One result per source; a failing source does not stop the others.
#[derive(Serialize)]
pub struct SourceReport {
    source: &'static str,
    fetched: usize,
    inserted: usize,
    duplicates: usize,
    error: Option<String>,
}

#[tauri::command]
pub async fn search_sources(state: State<'_, AppState>, input: SearchInput) -> AppResult<Vec<SourceReport>> {
    let sources = validate::sources(&input.sources)?;
    let (year_from, year_to) = validate::year_range(input.year_from, input.year_to)?;
    let query = SearchQuery {
        text: validate::query(&input.query)?,
        year_from,
        year_to,
        doc_type: validate::doc_type(input.doc_type.as_deref())?,
        max_results: validate::max_results(input.max_results)?,
    };
    let mut reports = Vec::with_capacity(sources.len());
    for source in sources {
        let report = match run_source(state.inner(), source, &query).await {
            Ok((fetched, inserted, duplicates)) => {
                SourceReport { source: source.label(), fetched, inserted, duplicates, error: None }
            }
            Err(err) => SourceReport {
                source: source.label(),
                fetched: 0,
                inserted: 0,
                duplicates: 0,
                error: Some(err.to_string()),
            },
        };
        reports.push(report);
    }
    Ok(reports)
}

async fn run_source(state: &AppState, source: Source, q: &SearchQuery) -> AppResult<(usize, usize, usize)> {
    let key = match source.key_service() {
        // Keychain access can block on an OS prompt, so keep it off the async workers.
        Some(service) => tauri::async_runtime::spawn_blocking(move || secrets::get(service))
            .await
            .map_err(|_| AppError::Internal)??,
        None => None,
    };
    let items = match source {
        Source::Openalex => openalex::search(&state.http, q, key.as_deref()).await?,
        Source::SemanticScholar => semantic_scholar::search(&state.http, q, key.as_deref()).await?,
        Source::Crossref => crossref::search(&state.http, q).await?,
        Source::Pubmed => pubmed::search(&state.http, q, key.as_deref()).await?,
        Source::EuropePmc => europepmc::search(&state.http, q).await?,
        Source::Arxiv => arxiv::search(&state.http, q).await?,
    };
    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    let (inserted, duplicates) = insert::insert_many(&mut conn, &items, source.label())?;
    Ok((items.len(), inserted, duplicates))
}
