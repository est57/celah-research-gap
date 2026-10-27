//! Direct citations between documents of the (filtered) collection, found three ways:
//! cited source ids (OpenAlex referenced_works), DOIs in reference text, and
//! surname + year confirmed by title or source.

use super::index::{Index, Scope};
use crate::error::AppResult;
use rusqlite::Connection;
use std::collections::HashSet;

#[derive(Default)]
pub struct Found {
    /// (citing, cited) document ids, without self-citations or repeats.
    pub links: Vec<(i64, i64)>,
    pub by_id: usize,
    pub by_doi: usize,
    pub by_name: usize,
}

fn rows<T>(conn: &Connection, sql: &str, f: impl Fn(&rusqlite::Row) -> rusqlite::Result<T>) -> AppResult<Vec<T>> {
    let mut stmt = conn.prepare(sql)?;
    let out = stmt.query_map([], |r| f(r))?.collect::<Result<Vec<_>, _>>()?;
    Ok(out)
}

pub fn find(conn: &Connection) -> AppResult<Found> {
    let mut found = Found::default();
    let mut seen = HashSet::new();
    let mut add = |citing: i64, cited: i64, counter: &mut usize| {
        if citing != cited && seen.insert((citing, cited)) {
            found.links.push((citing, cited));
            *counter += 1;
        }
    };

    // Aliases point at the record kept after a duplicate merge; it must be in the filtered set too.
    let by_id = rows(conn,
        "SELECT c.paper_id, p.id FROM paper_cites c JOIN papers p ON p.ext_id = c.cited
         UNION SELECT c.paper_id, p.id FROM paper_cites c JOIN main.paper_aliases a ON a.ext_id = c.cited JOIN papers p ON p.id = a.paper_id",
        |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)))?;
    let mut n = 0;
    for (a, b) in by_id {
        add(a, b, &mut n);
    }
    let by_id = n;

    let index = Index::load(conn, Scope::Filtered)?;

    let refs = rows(conn, "SELECT paper_id, raw FROM paper_refs", |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)))?;
    let (mut doi_n, mut name_n) = (0, 0);
    for (citing, raw) in refs {
        if let Some(cited) = index.by_doi(&raw) {
            add(citing, cited, &mut doi_n);
        } else if let Some(cited) = index.by_name(&raw) {
            add(citing, cited, &mut name_n);
        }
    }
    found.by_id = by_id;
    found.by_doi = doi_n;
    found.by_name = name_n;
    Ok(found)
}
