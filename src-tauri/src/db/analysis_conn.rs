//! Read-only connection used by every analysis. When a collection filter or keyword
//! synonyms are active it holds TEMP tables named like the main ones (papers,
//! paper_refs, paper_cites, paper_affiliations, paper_authors, paper_countries) with
//! only the filtered documents and synonym-rewritten keywords. SQLite resolves
//! unqualified names to the temp schema first, so analysis queries need no changes,
//! while all writes go through the main connection and never see these tables.

use super::filter::{self, CollectionFilter};
use super::synonyms;
use crate::error::AppResult;
use rusqlite::{params, params_from_iter, Connection, OpenFlags};
use std::path::Path;

const CHILDREN: [(&str, &str); 5] = [
    ("paper_refs", "paper_id"),
    ("paper_affiliations", "paper_id, name_key"),
    ("paper_cites", "paper_id, cited"),
    ("paper_authors", "paper_id, name_key"),
    ("paper_countries", "paper_id, code"),
];

pub struct AnalysisDb {
    conn: Connection,
    /// Filter and data version the temp tables were built for.
    built: Option<(CollectionFilter, i64)>,
}

impl AnalysisDb {
    pub fn open(path: &Path) -> AppResult<Self> {
        let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX)?;
        conn.pragma_update(None, "temp_store", "MEMORY")?;
        Ok(Self { conn, built: None })
    }

    /// The connection, with temp tables matching the current filter and data.
    pub fn prepare(&mut self) -> AppResult<&Connection> {
        // Changes when another connection commits, so imports and merges trigger a rebuild.
        let version: i64 = self.conn.query_row("PRAGMA data_version", [], |r| r.get(0))?;
        let f = filter::load(&self.conn)?;
        let current = self.built.as_ref().is_some_and(|(bf, bv)| *bf == f && *bv == version);
        if !current {
            self.drop_temp()?;
            let synonyms = synonyms::load(&self.conn)?;
            if f.is_active() || !synonyms.is_empty() {
                self.build(&f)?;
                self.apply_synonyms(&synonyms)?;
            }
            self.built = Some((f, version));
        }
        Ok(&self.conn)
    }

    fn drop_temp(&self) -> AppResult<()> {
        for (table, _) in CHILDREN {
            self.conn.execute_batch(&format!("DROP TABLE IF EXISTS temp.{table};"))?;
        }
        self.conn.execute_batch("DROP TABLE IF EXISTS temp.papers;")?;
        Ok(())
    }

    /// Rewrites the temp copy's keyword strings so every analysis sees one term per synonym group.
    fn apply_synonyms(&self, map: &synonyms::Map) -> AppResult<()> {
        if map.is_empty() {
            return Ok(());
        }
        let rows: Vec<(i64, Option<String>, Option<String>)> = self
            .conn
            .prepare("SELECT id, keywords, keywords_plus FROM temp.papers")?
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
            .collect::<Result<_, _>>()?;
        let mut update = self.conn.prepare("UPDATE temp.papers SET keywords = ?2, keywords_plus = ?3 WHERE id = ?1")?;
        for (id, de, plus) in rows {
            let (new_de, new_plus) = (de.as_deref().map(|k| synonyms::rewrite(k, map)), plus.as_deref().map(|k| synonyms::rewrite(k, map)));
            if new_de != de || new_plus != plus {
                update.execute(params![id, new_de, new_plus])?;
            }
        }
        Ok(())
    }

    fn build(&self, f: &CollectionFilter) -> AppResult<()> {
        let (cond, values) = f.condition();
        self.conn.execute(&format!("CREATE TEMP TABLE papers AS SELECT * FROM main.papers WHERE {cond}"), params_from_iter(values))?;
        self.conn.execute_batch("CREATE INDEX temp.f_papers_id ON papers(id); CREATE INDEX temp.f_papers_title ON papers(title_key, year);\n             CREATE INDEX temp.f_papers_ext ON papers(ext_id);")?;
        for (table, index) in CHILDREN {
            // Table and column names are the fixed literals above.
            self.conn.execute_batch(&format!(
                "CREATE TEMP TABLE {table} AS SELECT * FROM main.{table} WHERE paper_id IN (SELECT id FROM temp.papers);
                 CREATE INDEX temp.f_{table} ON {table}({index});"
            ))?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "analysis_conn_tests.rs"]
mod tests;
