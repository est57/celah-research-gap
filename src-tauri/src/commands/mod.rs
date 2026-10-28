pub mod add_works;
pub mod analysis;
pub mod authors;
pub mod collection;
pub mod duplicates;
pub mod enrich;
pub mod export;
pub mod filter;
pub mod idea;
pub mod import;
pub mod projects;
pub mod report;
pub mod restore;
pub mod search;
pub mod settings;
pub mod sjr;
pub mod synonyms;
pub mod text_refs;
pub mod update;
pub mod verify;
pub mod workflow;

use crate::error::{AppError, AppResult};
use crate::AppState;
use rusqlite::Connection;

/// Runs an analysis on the filtered collection.
pub fn with_analysis<T>(state: &AppState, f: impl FnOnce(&Connection) -> AppResult<T>) -> AppResult<T> {
    let mut db = state.analysis.lock().map_err(|_| AppError::Internal)?;
    f(db.prepare()?)
}

/// Every command the frontend may invoke. Anything not listed here is unreachable.
pub fn handler() -> impl Fn(tauri::ipc::Invoke) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        search::search_sources,
        workflow::workflow_status,
        projects::list_projects,
        projects::create_project,
        projects::rename_project,
        projects::delete_project,
        projects::switch_project,
        collection::list_papers,
        collection::collection_stats,
        collection::metadata_quality,
        collection::quick_search,
        collection::delete_papers,
        collection::delete_origin,
        duplicates::duplicates,
        duplicates::merge_duplicate,
        duplicates::dismiss_duplicate,
        filter::filter_options,
        filter::preview_filter,
        filter::set_collection_filter,
        analysis::overview,
        analysis::sources_authors,
        analysis::conceptual,
        analysis::evolution,
        analysis::factorial,
        analysis::words,
        analysis::rpys,
        analysis::top_refs,
        analysis::cited_entities,
        analysis::coupling_map,
        add_works::add_works_by_doi,
        analysis::historiograph,
        analysis::cited,
        analysis::three_field,
        analysis::sources_detail,
        analysis::author_table,
        authors::author_names,
        authors::merge_authors,
        authors::split_author,
        synonyms::keyword_synonyms,
        synonyms::merge_keywords,
        synonyms::remove_synonym,
        analysis::affiliations,
        analysis::country_table,
        analysis::structure,
        analysis::countries,
        analysis::gaps,
        analysis::set_gap_review,
        analysis::gap_history,
        verify::verify_gaps,
        idea::test_idea,
        idea::check_idea,
        idea::save_idea,
        idea::saved_ideas,
        idea::remove_idea,
        enrich::references_pending,
        enrich::complete_references,
        text_refs::text_references_pending,
        text_refs::complete_text_references,
        sjr::import_sjr,
        sjr::clear_sjr,
        sjr::quartiles,
        report::report,
        report::export_report,
        report::report_summary,
        report::export_summary,
        export::export_collection,
        export::backup_database,
        export::save_image,
        restore::choose_backup,
        restore::cancel_restore,
        restore::restore_backup,
        import::import_file,
        settings::api_key_status,
        settings::set_api_key,
        settings::clear_api_key,
        settings::app_info,
        update::check_update,
        update::install_update
    ]
}
