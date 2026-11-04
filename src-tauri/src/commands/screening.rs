//! Screening decisions and the PRISMA flow figures.

use crate::db::{prisma, screening};
use crate::error::{AppError, AppResult};
use crate::{tr, validate, AppState};
use serde::{Deserialize, Serialize};
use tauri::State;

const MAX_IDS: usize = 500;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PageInput {
    status: String,
    query: Option<String>,
    limit: u32,
    offset: u32,
}

#[derive(Serialize)]
pub struct ScreenPage {
    items: Vec<screening::ScreenRow>,
    total: i64,
    counts: screening::ScreenCounts,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DecisionInput {
    ids: Vec<i64>,
    /// "include", "exclude", or None to clear.
    decision: Option<String>,
    reason: String,
    note: String,
}

#[tauri::command(async)]
pub fn screening_page(state: State<'_, AppState>, input: PageInput) -> AppResult<ScreenPage> {
    let query = validate::filter(input.query.as_deref())?;
    let (limit, offset) = validate::page(input.limit, input.offset)?;
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    let (items, total) = screening::page(&conn, &input.status, query.as_deref(), limit, offset)?;
    Ok(ScreenPage { items, total, counts: screening::counts(&conn)? })
}

/// Checks a decision: known values, a reason only for exclusions, a short printable note.
fn checked(input: &DecisionInput) -> AppResult<(Option<&str>, &str, String)> {
    if input.ids.is_empty() || input.ids.len() > MAX_IDS {
        return Err(AppError::Invalid(tr!("pilih 1 sampai 500 dokumen", "choose 1 to 500 documents").into()));
    }
    let decision = input.decision.as_deref();
    let reason = match decision {
        Some("exclude") if screening::REASONS.contains(&input.reason.as_str()) => input.reason.as_str(),
        Some("exclude") => return Err(AppError::Invalid(tr!("alasan pengecualian tidak dikenal", "unknown exclusion reason").into())),
        Some("include") | None => "",
        Some(_) => return Err(AppError::Invalid(tr!("keputusan skrining tidak dikenal", "unknown screening decision").into())),
    };
    let note = input.note.trim();
    if note.chars().count() > screening::MAX_NOTE || note.chars().any(char::is_control) {
        return Err(AppError::Invalid(tr!("catatan maksimal 200 karakter tanpa karakter kontrol", "the note must be at most 200 characters, without control characters").into()));
    }
    Ok((decision, reason, if decision == Some("exclude") { note.to_string() } else { String::new() }))
}

#[tauri::command(async)]
pub fn set_screening(state: State<'_, AppState>, input: DecisionInput) -> AppResult<usize> {
    let (decision, reason, note) = checked(&input)?;
    let mut conn = state.db.lock().map_err(|_| AppError::Internal)?;
    screening::set(&mut conn, &input.ids, decision, reason, &note)
}

#[tauri::command(async)]
pub fn prisma_flow(state: State<'_, AppState>) -> AppResult<prisma::Prisma> {
    let conn = state.db.lock().map_err(|_| AppError::Internal)?;
    prisma::compute(&conn)
}

#[cfg(test)]
mod tests {
    use super::{checked, DecisionInput};

    fn input(decision: Option<&str>, reason: &str, note: &str, n: usize) -> DecisionInput {
        DecisionInput { ids: vec![1; n], decision: decision.map(Into::into), reason: reason.into(), note: note.into() }
    }

    #[test]
    fn rejects_bad_decisions() {
        assert!(checked(&input(Some("exclude"), "topic", "off topic", 1)).is_ok());
        assert_eq!(checked(&input(Some("include"), "topic", "x", 1)).unwrap(), (Some("include"), "", String::new()));
        assert!(checked(&input(Some("exclude"), "nope", "", 1)).is_err());
        assert!(checked(&input(Some("maybe"), "", "", 1)).is_err());
        assert!(checked(&input(None, "", "", 0)).is_err());
        assert!(checked(&input(None, "", "", 501)).is_err());
        assert!(checked(&input(Some("exclude"), "other", &"x".repeat(201), 1)).is_err());
        assert!(checked(&input(Some("exclude"), "other", "a\u{7}b", 1)).is_err());
    }
}
