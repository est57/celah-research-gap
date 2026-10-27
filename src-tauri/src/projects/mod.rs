//! Projects: one SQLite file each, so collections, reviews, synonyms and ideas of
//! different topics stay apart. The registry (names and the current project) is a
//! small JSON file next to the databases. The original database is the main project.

#[cfg(test)]
mod tests;

use crate::analysis::gaps::model::slug;
use crate::db;
use crate::error::{AppError, AppResult};
use rusqlite::{Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub const MAIN: &str = "utama";
const REGISTRY: &str = "proyek.json";
const MAX_NAME: usize = 60;
const MAX_PROJECTS: usize = 50;

#[derive(Serialize, Deserialize, Clone)]
pub struct Registry {
    pub current: String,
    pub names: BTreeMap<String, String>,
}

#[derive(Serialize)]
pub struct ProjectInfo {
    pub id: String,
    pub name: String,
    pub documents: i64,
    pub current: bool,
}

fn valid_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 40 && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Database file of a project; ids are checked so they can never leave the projects folder.
pub fn path_of(dir: &Path, id: &str) -> AppResult<PathBuf> {
    if id == MAIN {
        return Ok(dir.join("celah.sqlite"));
    }
    if !valid_id(id) {
        return Err(AppError::Invalid("proyek tidak dikenal".into()));
    }
    Ok(dir.join("proyek").join(format!("{id}.sqlite")))
}

pub fn load(dir: &Path) -> Registry {
    let main = || Registry { current: MAIN.into(), names: BTreeMap::from([(MAIN.to_string(), "Proyek utama".to_string())]) };
    let Ok(text) = std::fs::read_to_string(dir.join(REGISTRY)) else { return main() };
    match serde_json::from_str::<Registry>(&text) {
        Ok(mut r) if r.names.keys().all(|k| k == MAIN || valid_id(k)) => {
            r.names.entry(MAIN.into()).or_insert_with(|| "Proyek utama".into());
            if !r.names.contains_key(&r.current) {
                r.current = MAIN.into();
            }
            r
        }
        _ => main(),
    }
}

/// Written to a temporary file first, then moved into place.
pub fn save(dir: &Path, r: &Registry) -> AppResult<()> {
    let text = serde_json::to_string_pretty(r).map_err(|_| AppError::Internal)?;
    let tmp = dir.join("proyek.json.tmp");
    std::fs::write(&tmp, text).map_err(|_| AppError::Internal)?;
    std::fs::rename(&tmp, dir.join(REGISTRY)).map_err(|_| AppError::Internal)
}

fn clean_name(raw: &str) -> AppResult<String> {
    let name = raw.trim();
    if name.is_empty() || name.chars().count() > MAX_NAME || name.chars().any(char::is_control) {
        return Err(AppError::Invalid(format!("nama proyek harus 1 sampai {MAX_NAME} karakter")));
    }
    Ok(name.to_string())
}

pub fn list(dir: &Path, r: &Registry) -> Vec<ProjectInfo> {
    r.names
        .iter()
        .map(|(id, name)| {
            let documents = path_of(dir, id)
                .ok()
                .and_then(|p| Connection::open_with_flags(p, OpenFlags::SQLITE_OPEN_READ_ONLY).ok())
                .and_then(|c| c.query_row("SELECT COUNT(*) FROM papers", [], |row| row.get(0)).ok())
                .unwrap_or(0);
            ProjectInfo { id: id.clone(), name: name.clone(), documents, current: *id == r.current }
        })
        .collect()
}

/// Creates the project's database (with every migration) and returns its id.
pub fn create(dir: &Path, r: &mut Registry, raw_name: &str) -> AppResult<String> {
    let name = clean_name(raw_name)?;
    if r.names.len() >= MAX_PROJECTS {
        return Err(AppError::Invalid(format!("paling banyak {MAX_PROJECTS} proyek")));
    }
    let base: String = { let s = slug(&name); if valid_id(&s) && s != MAIN { s.chars().take(36).collect() } else { "proyek".into() } };
    let id = (1..).map(|n| if n == 1 { base.clone() } else { format!("{base}-{n}") }).find(|id| !r.names.contains_key(id)).unwrap_or(base);
    let path = path_of(dir, &id)?;
    std::fs::create_dir_all(dir.join("proyek")).map_err(|_| AppError::Internal)?;
    drop(db::open(&path)?);
    r.names.insert(id.clone(), name);
    save(dir, r)?;
    Ok(id)
}

pub fn rename(dir: &Path, r: &mut Registry, id: &str, raw_name: &str) -> AppResult<()> {
    let name = clean_name(raw_name)?;
    let slot = r.names.get_mut(id).ok_or_else(|| AppError::Invalid("proyek tidak dikenal".into()))?;
    *slot = name;
    save(dir, r)
}

/// Deletes a project's files. The open project and the main project cannot be deleted.
pub fn delete(dir: &Path, r: &mut Registry, id: &str) -> AppResult<()> {
    if id == MAIN || id == r.current || !r.names.contains_key(id) {
        return Err(AppError::Invalid("proyek utama dan proyek yang sedang dibuka tidak bisa dihapus".into()));
    }
    let path = path_of(dir, id)?;
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{suffix}", path.display()));
    }
    r.names.remove(id);
    save(dir, r)
}
