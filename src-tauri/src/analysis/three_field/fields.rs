//! The fields a three-field plot column can show, and the items each document has.

use super::super::refkey::ref_key;
use super::super::terms_with_label;
use crate::error::{AppError, AppResult};
use rusqlite::Connection;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Field {
    Authors,
    AuthorKeywords,
    KeywordsPlus,
    Sources,
    Countries,
    References,
}

/// (key, label) items per paper, each key once.
pub type Items = HashMap<i64, Vec<(String, String)>>;

/// Turns the two text columns of a row into (key, label) items.
type Parse = fn(String, String) -> Vec<(String, String)>;

impl Field {
    pub fn parse(raw: &str) -> AppResult<Field> {
        Ok(match raw {
            "authors" => Field::Authors,
            "author_keywords" => Field::AuthorKeywords,
            "keywords_plus" => Field::KeywordsPlus,
            "sources" => Field::Sources,
            "countries" => Field::Countries,
            "references" => Field::References,
            _ => return Err(AppError::Invalid("bidang plot tidak dikenal".into())),
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Field::Authors => "Penulis",
            Field::AuthorKeywords => "Kata kunci penulis",
            Field::KeywordsPlus => "Keywords Plus",
            Field::Sources => "Sumber",
            Field::Countries => "Negara",
            Field::References => "Referensi tersitasi",
        }
    }

    pub fn items(self, conn: &Connection) -> AppResult<Items> {
        // Every SQL text below is a fixed literal; nothing comes from the caller.
        let (sql, parse): (&str, Parse) = match self {
            Field::Authors => ("SELECT paper_id, name_key, name FROM paper_authors", |k, l| vec![(k, l)]),
            Field::AuthorKeywords => ("SELECT id, keywords, '' FROM papers WHERE COALESCE(keywords, '') <> ''", |k, _| terms_with_label(Some(&k)).collect()),
            Field::KeywordsPlus => ("SELECT id, keywords_plus, '' FROM papers WHERE COALESCE(keywords_plus, '') <> ''", |k, _| terms_with_label(Some(&k)).collect()),
            Field::Sources => ("SELECT id, lower(trim(source_name)), trim(source_name) FROM papers WHERE trim(COALESCE(source_name, '')) <> ''", |k, l| vec![(k, l)]),
            Field::Countries => ("SELECT paper_id, code, code FROM paper_countries", |k, l| vec![(k, l)]),
            Field::References => ("SELECT paper_id, raw, '' FROM paper_refs", |k, _| ref_key(&k).into_iter().collect()),
        };
        let mut stmt = conn.prepare(sql)?;
        let mut rows = stmt.query([])?;
        let mut out: Items = HashMap::new();
        while let Some(r) = rows.next()? {
            let (id, a, b): (i64, String, String) = (r.get(0)?, r.get(1)?, r.get(2)?);
            let list = out.entry(id).or_default();
            for (key, label) in parse(a, b) {
                if !list.iter().any(|(k, _)| *k == key) {
                    list.push((key, label));
                }
            }
        }
        Ok(out)
    }
}
