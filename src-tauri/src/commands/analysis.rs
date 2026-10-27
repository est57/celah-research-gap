use crate::analysis::cited::{Cited, CitedParams};
use crate::analysis::conceptual::{Conceptual, ConceptualParams};
use crate::analysis::countries::Countries;
use crate::analysis::evolution::{Evolution, EvolutionParams};
use crate::analysis::factorial::{Factorial, FactorialParams};
use crate::analysis::historiograph::{HistParams, Historiograph};
use crate::analysis::gaps::GapList;
use crate::db::{review_history, reviews};
use crate::analysis::structure::{Structure, StructureParams};
use crate::analysis::{self, sources_authors::SourcesAuthors, Overview};
use crate::error::{AppError, AppResult};
use crate::AppState;
use super::with_analysis;
use tauri::State;

#[tauri::command(async)]
pub fn overview(state: State<'_, AppState>) -> AppResult<Overview> {
    with_analysis(&state, analysis::overview)
}

#[tauri::command(async)]
pub fn conceptual(state: State<'_, AppState>, params: ConceptualParams) -> AppResult<Conceptual> {
    with_analysis(&state, |conn| analysis::conceptual::compute(conn, &params))
}

#[tauri::command(async)]
pub fn evolution(state: State<'_, AppState>, params: EvolutionParams) -> AppResult<Evolution> {
    with_analysis(&state, |conn| analysis::evolution::compute(conn, &params))
}

#[tauri::command(async)]
pub fn factorial(state: State<'_, AppState>, params: FactorialParams) -> AppResult<Factorial> {
    with_analysis(&state, |conn| analysis::factorial::compute(conn, &params))
}

#[tauri::command(async)]
pub fn historiograph(state: State<'_, AppState>, params: HistParams) -> AppResult<Historiograph> {
    with_analysis(&state, |conn| analysis::historiograph::compute(conn, &params))
}

#[tauri::command(async)]
pub fn cited(state: State<'_, AppState>, params: CitedParams) -> AppResult<Cited> {
    with_analysis(&state, |conn| analysis::cited::compute(conn, &params))
}

#[tauri::command(async)]
pub fn three_field(state: State<'_, AppState>, params: analysis::three_field::ThreeFieldParams) -> AppResult<analysis::three_field::ThreeField> {
    with_analysis(&state, |conn| analysis::three_field::compute_params(conn, &params))
}

#[tauri::command(async)]
pub fn structure(state: State<'_, AppState>, params: StructureParams) -> AppResult<Structure> {
    with_analysis(&state, |conn| analysis::structure::compute(conn, &params))
}

#[tauri::command(async)]
pub fn countries(state: State<'_, AppState>) -> AppResult<Countries> {
    with_analysis(&state, analysis::countries::compute)
}

#[tauri::command(async)]
pub fn gaps(state: State<'_, AppState>) -> AppResult<GapList> {
    with_analysis(&state, analysis::gaps::compute)
}

/// `status` is "accepted", "check", "rejected", or "" to clear the review.
/// `title` is stored so the review history stays readable when the gap drops out.
#[tauri::command(async)]
pub fn set_gap_review(state: State<'_, AppState>, key: String, status: String, note: String, title: String) -> AppResult<()> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    reviews::set(&conn, &key, &status, &note, &title)
}

#[tauri::command(async)]
pub fn gap_history(state: State<'_, AppState>) -> AppResult<Vec<review_history::ReviewEntry>> {
    with_analysis(&state, review_history::list)
}

#[tauri::command(async)]
pub fn sources_authors(state: State<'_, AppState>) -> AppResult<SourcesAuthors> {
    with_analysis(&state, analysis::sources_authors::compute)
}

#[tauri::command(async)]
pub fn sources_detail(state: State<'_, AppState>) -> AppResult<analysis::source_impact::SourcesDetail> {
    with_analysis(&state, analysis::source_impact::compute)
}

#[tauri::command(async)]
pub fn author_table(state: State<'_, AppState>) -> AppResult<Vec<analysis::author_table::AuthorRow>> {
    with_analysis(&state, analysis::author_table::compute)
}

#[tauri::command(async)]
pub fn affiliations(state: State<'_, AppState>) -> AppResult<analysis::affiliations::Affiliations> {
    with_analysis(&state, analysis::affiliations::compute)
}

#[tauri::command(async)]
pub fn words(state: State<'_, AppState>, params: analysis::words::WordParams) -> AppResult<analysis::words::Words> {
    with_analysis(&state, |conn| analysis::words::compute(conn, &params))
}

#[tauri::command(async)]
pub fn rpys(state: State<'_, AppState>, params: analysis::rpys::RpysParams) -> AppResult<analysis::rpys::Rpys> {
    with_analysis(&state, |conn| analysis::rpys::compute(conn, &params))
}

#[tauri::command(async)]
pub fn country_table(state: State<'_, AppState>) -> AppResult<analysis::country_table::CountryTable> {
    with_analysis(&state, analysis::country_table::compute)
}

#[tauri::command(async)]
pub fn top_refs(state: State<'_, AppState>, params: analysis::top_refs::TopRefParams) -> AppResult<analysis::top_refs::TopRefs> {
    with_analysis(&state, |conn| analysis::top_refs::compute(conn, &params))
}

#[tauri::command(async)]
pub fn coupling_map(state: State<'_, AppState>, params: analysis::coupling_map::CouplingMapParams) -> AppResult<analysis::coupling_map::CouplingMap> {
    with_analysis(&state, |conn| analysis::coupling_map::compute(conn, &params))
}

#[tauri::command(async)]
pub fn cited_entities(state: State<'_, AppState>) -> AppResult<analysis::cited_entities::CitedEntities> {
    with_analysis(&state, analysis::cited_entities::compute)
}
