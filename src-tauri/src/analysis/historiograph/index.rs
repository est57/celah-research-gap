//! Lookup of collection documents by DOI and by first-author surname + year, used to
//! tell which free-text references point at a document the collection already has.

use super::refmatch::{self, Doc};
use crate::error::AppResult;
use crate::report::names::surname;
use rusqlite::Connection;
use std::collections::HashMap;

/// Which documents to index: the filtered analysis set, or the whole stored collection.
#[derive(Clone, Copy)]
pub enum Scope {
    Filtered,
    Full,
}

pub struct Index {
    dois: HashMap<String, i64>,
    names: HashMap<(String, String), Vec<Doc>>,
}

fn surname_key(name: &str) -> String {
    surname(name).chars().filter(|c| c.is_alphabetic() || *c == '-').flat_map(char::to_lowercase).collect()
}

impl Index {
    pub fn load(conn: &Connection, scope: Scope) -> AppResult<Index> {
        // Both variants are fixed literals; `main.` reads past the collection filter.
        // Aliases point at the record kept after a duplicate merge.
        let (doi_sql, doc_sql) = match scope {
            Scope::Filtered => (
                "SELECT doi, id FROM papers WHERE doi IS NOT NULL
                 UNION SELECT a.doi, p.id FROM main.paper_aliases a JOIN papers p ON p.id = a.paper_id WHERE a.doi IS NOT NULL",
                "SELECT p.id, p.year, p.title, COALESCE(p.source_name, ''),
                        COALESCE((SELECT a.name FROM paper_authors a WHERE a.paper_id = p.id ORDER BY a.position LIMIT 1), '')
                 FROM papers p WHERE p.year IS NOT NULL",
            ),
            Scope::Full => (
                "SELECT doi, id FROM main.papers WHERE doi IS NOT NULL
                 UNION SELECT doi, paper_id FROM main.paper_aliases WHERE doi IS NOT NULL",
                "SELECT p.id, p.year, p.title, COALESCE(p.source_name, ''),
                        COALESCE((SELECT a.name FROM main.paper_authors a WHERE a.paper_id = p.id ORDER BY a.position LIMIT 1), '')
                 FROM main.papers p WHERE p.year IS NOT NULL",
            ),
        };
        let mut stmt = conn.prepare(doi_sql)?;
        let dois = stmt
            .query_map([], |r| Ok((r.get::<_, String>(0)?.to_lowercase(), r.get::<_, i64>(1)?)))?
            .collect::<Result<HashMap<_, _>, _>>()?;
        let mut names: HashMap<(String, String), Vec<Doc>> = HashMap::new();
        let mut stmt = conn.prepare(doc_sql)?;
        let mut rows = stmt.query([])?;
        while let Some(r) = rows.next()? {
            let (id, year, title, source, first): (i64, i32, String, String, String) = (r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?);
            let key = surname_key(&first);
            if !key.is_empty() {
                let doc = Doc { id, title_words: refmatch::words(&title), source_words: refmatch::words(&source) };
                names.entry((key, year.to_string())).or_default().push(doc);
            }
        }
        Ok(Index { dois, names })
    }

    /// The collection document a reference points to, by DOI first.
    pub fn find(&self, raw: &str) -> Option<i64> {
        refmatch::dois(raw).iter().find_map(|d| self.dois.get(d).copied()).or_else(|| self.by_name(raw))
    }

    pub fn by_doi(&self, raw: &str) -> Option<i64> {
        refmatch::dois(raw).iter().find_map(|d| self.dois.get(d).copied())
    }

    pub fn by_name(&self, raw: &str) -> Option<i64> {
        refmatch::by_author_year(raw, &self.names).map(|d| d.id)
    }
}
