use super::with_analysis;
use crate::analysis::gaps;
use crate::analysis::top_refs::{self, TopRefParams};
use crate::db::workflow::{self, Workflow};
use crate::error::{AppError, AppResult};
use crate::AppState;
use tauri::State;

/// Most cited references checked against the collection.
const CORE: u32 = 50;

/// Progress of the open project through the workflow steps.
#[tauri::command(async)]
pub fn workflow_status(state: State<'_, AppState>) -> AppResult<Workflow> {
    let mut w = {
        let conn = state.db.lock().map_err(|_| AppError::Internal)?;
        workflow::status(&conn)?
    };
    let (gaps, listed, missing) = with_analysis(&state, |conn| {
        let refs = top_refs::compute(conn, &TopRefParams { top: CORE })?;
        Ok((gaps::compute(conn)?.gaps.len(), refs.refs.len(), refs.missing))
    })?;
    (w.gaps, w.core_listed, w.core_missing) = (gaps, listed, missing);
    Ok(w)
}
