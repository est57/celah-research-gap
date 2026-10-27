//! The figures the methods and results draft is written from, gathered from the
//! (filtered, synonym-aware) analysis connection.

use crate::analysis::cited::{self, CitedParams};
use crate::analysis::rpys::{self, RpysParams};
use crate::analysis::{annual, author_table, country_table, current_year, main_info, sources};
use crate::db::filter;
use crate::error::AppResult;
use rusqlite::Connection;

const TOP: usize = 3;

pub struct Facts {
    pub info: main_info::MainInfo,
    /// (origin, documents), largest first.
    pub origins: Vec<(String, i64)>,
    /// Records folded into others by duplicate merging, over the whole collection.
    pub merged: i64,
    pub filtered: bool,
    pub sources: Vec<(String, i64)>,
    pub authors: Vec<(String, usize)>,
    /// (ISO code, documents).
    pub countries: Vec<(String, usize)>,
    /// (paper id, global citations), most cited first.
    pub cited: Vec<(i64, i64)>,
    /// (year, label of the most cited work of that year), strongest peaks first.
    pub roots: Vec<(u16, String)>,
}

pub fn gather(conn: &Connection) -> AppResult<Facts> {
    let now = current_year(conn)?;
    let points = annual::compute(conn, now)?;
    let info = main_info::compute(conn, now, annual::growth_rate(&points))?;
    let mut stmt = conn.prepare("SELECT origin, COUNT(*) FROM papers GROUP BY origin ORDER BY COUNT(*) DESC, origin")?;
    let origins = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<Result<Vec<_>, _>>()?;
    let merged: i64 = conn.query_row("SELECT COUNT(*) FROM main.paper_aliases", [], |r| r.get(0))?;
    let filtered = filter::load(conn)?.is_active();
    let sources = sources::ranked_sources(conn)?.into_iter().take(TOP).map(|s| (s.label, s.count)).collect();
    let mut people = author_table::compute(conn)?;
    people.sort_by(|a, b| b.documents.cmp(&a.documents).then(b.citations.cmp(&a.citations)).then(a.label.cmp(&b.label)));
    let authors = people.into_iter().take(TOP).map(|a| (a.label, a.documents)).collect();
    let countries = country_table::compute(conn)?.rows.into_iter().take(TOP).map(|c| (c.code, c.documents)).collect();
    let cited = cited::compute(conn, &CitedParams { sort: "global".into(), limit: 10 })?
        .docs
        .into_iter()
        .filter(|d| d.global > 0)
        .take(TOP)
        .map(|d| (d.id, d.global))
        .collect();
    let roots = rpys::compute(conn, &RpysParams { from: 0, to: 0 })?
        .peaks
        .into_iter()
        // Roots are works older than the collection itself, not its own recent surge.
        .filter(|p| info.year_min.is_some_and(|m| i32::from(p.year) < m))
        .filter_map(|p| p.refs.first().map(|r| (p.year, r.label.clone())))
        .take(TOP)
        .collect();
    Ok(Facts { info, origins, merged, filtered, sources, authors, countries, cited, roots })
}
