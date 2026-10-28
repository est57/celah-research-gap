//! The collection filter: which documents the analyses use. Stored in the database
//! so it survives restarts; every value reaches SQL as a bound parameter.

use crate::error::{AppError, AppResult};
use rusqlite::types::Value;
use rusqlite::{params, params_from_iter, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

const MAX_ITEMS: usize = 200;
const MAX_TEXT: usize = 300;

#[derive(Deserialize, Serialize, Default, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CollectionFilter {
    pub year_from: Option<i32>,
    pub year_to: Option<i32>,
    /// Stored document types; "" stands for documents without a type. Empty = all.
    pub doc_types: Vec<String>,
    /// Import sources, e.g. "Scopus". Empty = all.
    pub origins: Vec<String>,
    /// Journals or other publication sources; "" for none recorded. Empty = all.
    pub sources: Vec<String>,
    /// Journal quartiles from the imported SJR list: "Q1".."Q4", or "none" for no quartile. Empty = all.
    #[serde(default)]
    pub quartiles: Vec<String>,
}

impl CollectionFilter {
    pub fn is_active(&self) -> bool {
        *self != CollectionFilter::default()
    }

    pub fn validate(&self) -> AppResult<()> {
        let year_ok = |y: Option<i32>| y.is_none_or(|y| (1900..=2100).contains(&y));
        let order_ok = match (self.year_from, self.year_to) {
            (Some(a), Some(b)) => a <= b,
            _ => true,
        };
        if !year_ok(self.year_from) || !year_ok(self.year_to) || !order_ok {
            return Err(AppError::Invalid(crate::tr!("rentang tahun filter tidak sah", "invalid filter year range").into()));
        }
        if self.quartiles.iter().any(|q| !matches!(q.as_str(), "Q1" | "Q2" | "Q3" | "Q4" | "none")) || self.quartiles.len() > 5 {
            return Err(AppError::Invalid(crate::tr!("pilihan kuartil tidak dikenal", "unknown quartile option").into()));
        }
        for list in [&self.doc_types, &self.origins, &self.sources] {
            let bad = |v: &String| v.chars().count() > MAX_TEXT || v.chars().any(char::is_control);
            if list.len() > MAX_ITEMS || list.iter().any(bad) {
                return Err(AppError::Invalid(crate::tr!("pilihan filter terlalu banyak atau berisi karakter tidak sah", "too many filter options or invalid characters").into()));
            }
        }
        Ok(())
    }

    /// SQL condition on `papers` plus its parameters. Only placeholders are formatted in.
    pub fn condition(&self) -> (String, Vec<Value>) {
        let mut parts: Vec<String> = Vec::new();
        let mut values: Vec<Value> = Vec::new();
        if let Some(y) = self.year_from {
            parts.push("year >= ?".into());
            values.push(Value::Integer(y.into()));
        }
        if let Some(y) = self.year_to {
            parts.push("year <= ?".into());
            values.push(Value::Integer(y.into()));
        }
        let lists = [("COALESCE(doc_type, '')", &self.doc_types), ("origin", &self.origins), ("COALESCE(source_name, '')", &self.sources)];
        for (column, list) in lists {
            if !list.is_empty() {
                parts.push(format!("{column} IN ({})", vec!["?"; list.len()].join(", ")));
                values.extend(list.iter().map(|v| Value::Text(v.clone())));
            }
        }
        if !self.quartiles.is_empty() {
            parts.push(format!("{} IN ({})", super::sjr::QUARTILE_SQL, vec!["?"; self.quartiles.len()].join(", ")));
            values.extend(self.quartiles.iter().map(|q| Value::Text(if q == "none" { String::new() } else { q.clone() })));
        }
        let sql = if parts.is_empty() { "1".to_string() } else { parts.join(" AND ") };
        (sql, values)
    }
}

pub fn load(conn: &Connection) -> AppResult<CollectionFilter> {
    let json: Option<String> = conn.query_row("SELECT json FROM main.collection_filter WHERE id = 1", [], |r| r.get(0)).optional()?;
    // A stored filter that no longer parses (e.g. after a format change) is treated as no filter.
    Ok(json.and_then(|j| serde_json::from_str(&j).ok()).unwrap_or_default())
}

pub fn save(conn: &Connection, f: &CollectionFilter) -> AppResult<()> {
    f.validate()?;
    if !f.is_active() {
        conn.execute("DELETE FROM main.collection_filter", [])?;
        return Ok(());
    }
    let json = serde_json::to_string(f).map_err(|_| AppError::Internal)?;
    conn.execute(
        "INSERT INTO main.collection_filter (id, json) VALUES (1, ?1) ON CONFLICT(id) DO UPDATE SET json = excluded.json",
        params![json],
    )?;
    Ok(())
}

/// Documents of the whole collection that pass the filter.
pub fn matching(conn: &Connection, f: &CollectionFilter) -> AppResult<i64> {
    let (cond, values) = f.condition();
    Ok(conn.query_row(&format!("SELECT COUNT(*) FROM main.papers WHERE {cond}"), params_from_iter(values), |r| r.get(0))?)
}
