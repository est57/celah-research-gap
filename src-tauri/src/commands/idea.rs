use super::with_analysis;
use crate::analysis::idea::{self, Idea, IdeaParams};
use crate::analysis::current_year;
use crate::db::checks::{self, CheckPaper};
use crate::db::ideas;
use crate::error::{AppError, AppResult};
use crate::search::SearchQuery;
use crate::secrets::{self, Service};
use crate::verify::in_collection;
use crate::{openalex, AppState};
use serde::Serialize;
use tauri::State;

const PER_IDEA: u32 = 50;
const KEEP: usize = 20;
const RECENT_YEARS: i32 = 2;

#[derive(Serialize)]
pub struct IdeaCheck {
    pub query: String,
    pub found: usize,
    pub matching: usize,
    /// "open", "new" or "answered", as for detected gaps.
    pub verdict: &'static str,
    pub papers: Vec<CheckPaper>,
}

/// Counts the two concepts in the (filtered, synonym-aware) collection.
#[tauri::command(async)]
pub fn test_idea(state: State<'_, AppState>, params: IdeaParams) -> AppResult<Idea> {
    with_analysis(&state, |conn| idea::compute(conn, &params))
}

/// Searches recent OpenAlex records for papers that combine both concepts and are not in the collection.
#[tauri::command]
pub async fn check_idea(state: State<'_, AppState>, params: IdeaParams) -> AppResult<IdeaCheck> {
    let (a, b) = (idea::parse(&params.a)?, idea::parse(&params.b)?);
    let query: String = format!("{} {}", a[0], b[0]).chars().take(300).collect();
    let year = {
        let conn = state.db.lock().map_err(|_| AppError::Internal)?;
        current_year(&conn)?
    };
    let api_key = tauri::async_runtime::spawn_blocking(|| secrets::get(Service::Openalex)).await.map_err(|_| AppError::Internal)??;
    let search = SearchQuery { text: query.clone(), year_from: year - RECENT_YEARS, year_to: year, doc_type: None, max_results: PER_IDEA };
    let found = openalex::search(&state.http, &search, api_key.as_deref()).await?;
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    let mut papers = Vec::new();
    for p in &found {
        if in_collection(&conn, p)? {
            continue;
        }
        let text = idea::stems(&format!("{} {}", p.title, p.abstract_text.as_deref().unwrap_or("")));
        papers.push(CheckPaper {
            title: p.title.clone(),
            year: p.year,
            doi: p.doi.clone(),
            source: p.source_name.clone(),
            cited_by: p.cited_by,
            matches: idea::any_form(&text, &a) && idea::any_form(&text, &b),
        });
    }
    papers.sort_by(|x, y| y.matches.cmp(&x.matches).then(y.cited_by.cmp(&x.cited_by)));
    let matching = papers.iter().filter(|p| p.matches).count();
    let total = papers.len();
    papers.truncate(KEEP);
    Ok(IdeaCheck { query, found: total, matching, verdict: checks::verdict(matching as i64), papers })
}

#[derive(Serialize)]
pub struct SavedIdea {
    pub key: String,
    pub a: String,
    pub b: String,
}

/// Saves the idea so Gap Explorer lists it; returns its gap key.
#[tauri::command(async)]
pub fn save_idea(state: State<'_, AppState>, params: IdeaParams) -> AppResult<String> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    ideas::save(&conn, &params.a, &params.b)
}

#[tauri::command(async)]
pub fn saved_ideas(state: State<'_, AppState>) -> AppResult<Vec<SavedIdea>> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    Ok(ideas::list(&conn)?.into_iter().map(|i| SavedIdea { key: i.key, a: i.a, b: i.b }).collect())
}

#[tauri::command(async)]
pub fn remove_idea(state: State<'_, AppState>, key: String) -> AppResult<()> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    ideas::remove(&conn, &key)
}
