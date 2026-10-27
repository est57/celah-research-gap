//! The imported Scimago (SJR) journal list and the quartile of each document's journal,
//! matched by ISSN first and by exact journal title when no ISSN matches.

use crate::error::AppResult;
use crate::import::sjr::{title_key, Journal};
use rusqlite::{params, Connection};
use serde::Serialize;

/// Quartile of the current row's journal; usable in any query whose outer table is named
/// `papers` (main or the filtered temp copy). Columns are qualified so the subqueries never
/// read the SJR table's own `issn` by mistake.
/// ISSNs are stored as "XXXXXXXX; YYYYYYYY", so each of the first three sits at a fixed offset,
/// which keeps the lookup on the primary key.
pub const QUARTILE_SQL: &str = "COALESCE(
    (SELECT s.quartile FROM main.sjr_journals s
      WHERE s.issn IN (substr(COALESCE(papers.issn, ''), 1, 8), substr(COALESCE(papers.issn, ''), 11, 8), substr(COALESCE(papers.issn, ''), 21, 8))
        AND s.quartile IS NOT NULL LIMIT 1),
    (SELECT s.quartile FROM main.sjr_journals s
      WHERE s.title_key = replace(replace(lower(trim(COALESCE(papers.source_name, ''))), '   ', ' '), '  ', ' ')
        AND s.quartile IS NOT NULL LIMIT 1),
    '')";

/// SJR value of the current row's journal, matched by ISSN only (see `QUARTILE_SQL`).
pub const SJR_SQL: &str = "(SELECT s.sjr FROM main.sjr_journals s
      WHERE s.issn IN (substr(COALESCE(papers.issn, ''), 1, 8), substr(COALESCE(papers.issn, ''), 11, 8), substr(COALESCE(papers.issn, ''), 21, 8))
      LIMIT 1)";

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SjrStatus {
    /// Distinct journals (titles) in the imported list.
    pub journals: i64,
    pub year: Option<i32>,
}

/// Replaces the stored list. Each ISSN of a journal becomes its own row.
pub fn replace(conn: &mut Connection, journals: &[Journal], year: Option<i32>) -> AppResult<SjrStatus> {
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM sjr_journals", [])?;
    {
        let mut add = tx.prepare_cached(
            "INSERT OR IGNORE INTO sjr_journals (issn, title, title_key, sjr, quartile, categories, year) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        )?;
        for j in journals {
            for issn in j.issns.split("; ") {
                add.execute(params![issn, j.title, title_key(&j.title), j.sjr, j.quartile, j.categories, year])?;
            }
        }
    }
    tx.commit()?;
    status(conn)
}

pub fn status(conn: &Connection) -> AppResult<SjrStatus> {
    Ok(conn.query_row("SELECT COUNT(DISTINCT title_key), MAX(year) FROM main.sjr_journals", [], |r| {
        Ok(SjrStatus { journals: r.get(0)?, year: r.get(1)? })
    })?)
}

pub fn clear(conn: &Connection) -> AppResult<()> {
    conn.execute("DELETE FROM sjr_journals", [])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};
    use crate::import::sjr::Journal;

    #[test]
    fn matches_by_issn_then_by_title() {
        let mut conn = open_memory();
        let j = |issns: &str, title: &str, q: Option<&str>| Journal { issns: issns.into(), title: title.into(), sjr: Some(1.0), quartile: q.map(Into::into), categories: None };
        let st = super::replace(&mut conn, &[j("18728286; 09252312", "Neurocomputing", Some("Q1")), j("11112222", "IEEE Access", Some("Q2"))], Some(2024)).unwrap();
        assert_eq!((st.journals, st.year), (2, Some(2024)));
        let p = |id: &str, issn: Option<&str>, source: &str| NewPaper { ext_id: id.into(), title: id.into(), issn: issn.map(Into::into), source_name: Some(source.into()), ..Default::default() };
        insert_many(&mut conn, &[p("a", Some("09252312"), "Neurocomputing"), p("b", None, " ieee  access"), p("c", Some("99998888"), "Other")], "Test").unwrap();
        let sql = format!("SELECT ext_id, {} FROM papers ORDER BY ext_id", super::QUARTILE_SQL);
        let mut stmt = conn.prepare(&sql).unwrap();
        let got: Vec<(String, String)> = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?))).unwrap().map(Result::unwrap).collect();
        assert_eq!(got, vec![("a".into(), "Q1".into()), ("b".into(), "Q2".into()), ("c".into(), "".into())], "b matches by title despite case and spacing");
        let only = |q: &[&str]| crate::db::filter::CollectionFilter { quartiles: q.iter().map(|v| v.to_string()).collect(), ..Default::default() };
        assert_eq!(crate::db::filter::matching(&conn, &only(&["Q1"])).unwrap(), 1);
        assert_eq!(crate::db::filter::matching(&conn, &only(&["Q1", "Q2"])).unwrap(), 2);
        assert_eq!(crate::db::filter::matching(&conn, &only(&["none"])).unwrap(), 1);
        assert!(only(&["Q5"]).validate().is_err());
        let opts = crate::db::filter_options::options(&conn).unwrap();
        assert_eq!(opts.quartiles.iter().map(|c| (c.value.as_str(), c.count)).collect::<Vec<_>>(), vec![("Q1", 1), ("Q2", 1), ("none", 1)]);
        super::clear(&conn).unwrap();
        assert_eq!(super::status(&conn).unwrap().journals, 0);
    }
}
