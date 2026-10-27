//! Most cited references of the collection (bibliometrix Most Local Cited References),
//! each marked with the collection document it points to, if any. Works cited often
//! but missing from the collection are candidates to read and add.

use super::historiograph::index::{Index, Scope};
use super::historiograph::refmatch::dois;
use super::conceptual::in_range;
use super::rpys::count::count;
use crate::error::AppResult;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TopRefParams {
    pub top: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopRef {
    /// "Surname Year".
    pub label: String,
    pub year: u16,
    /// The reference as one citing document wrote it.
    pub example: String,
    pub doi: Option<String>,
    /// Documents in the (filtered) collection that cite it.
    pub documents: usize,
    /// Id and title of the stored document it points to, searched in the whole collection.
    pub in_collection: Option<i64>,
    pub title: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopRefs {
    pub documents: i64,
    /// Documents with at least one text reference.
    pub citing: usize,
    /// Distinct cited works with a publication year.
    pub works: usize,
    pub refs: Vec<TopRef>,
    /// Listed works not found in the collection.
    pub missing: usize,
}

pub fn compute(conn: &Connection, p: &TopRefParams) -> AppResult<TopRefs> {
    let top = in_range(p.top, 20, 500, "jumlah rujukan")? as usize;
    let documents: i64 = conn.query_row("SELECT COUNT(*) FROM papers", [], |r| r.get(0))?;
    let newest: Option<i64> = conn.query_row("SELECT MAX(year) FROM main.papers", [], |r| r.get(0))?;
    let counts = count(conn, newest.map_or(2100, |y| y.clamp(1800, 2100) as u16))?;

    let mut ranked: Vec<_> = counts.refs.values().collect();
    ranked.sort_by(|a, b| b.documents.cmp(&a.documents).then(b.year.cmp(&a.year)).then(a.label.cmp(&b.label)));
    ranked.truncate(top);

    let index = Index::load(conn, Scope::Full)?;
    let mut title_of = conn.prepare("SELECT title FROM main.papers WHERE id = ?1")?;
    let mut refs = Vec::with_capacity(ranked.len());
    for t in ranked {
        let in_collection = index.find(&t.example);
        let title = match in_collection {
            Some(id) => title_of.query_row(params![id], |r| r.get(0)).optional()?,
            None => None,
        };
        refs.push(TopRef {
            label: t.label.clone(),
            year: t.year,
            example: t.example.clone(),
            doi: dois(&t.example).into_iter().next(),
            documents: t.documents,
            in_collection,
            title,
        });
    }
    let missing = refs.iter().filter(|r| r.in_collection.is_none()).count();
    Ok(TopRefs { documents, citing: counts.citing, works: counts.refs.len(), refs, missing })
}

#[cfg(test)]
mod tests {
    use super::{compute, TopRefParams};
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    fn paper(id: &str, title: &str, year: i32, author: &str, refs: &[&str]) -> NewPaper {
        NewPaper {
            ext_id: id.into(),
            title: title.into(),
            year: Some(year),
            author_names: vec![author.into()],
            source_name: Some("Scientific Reports".into()),
            refs: refs.iter().map(|r| r.to_string()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn ranks_cited_works_and_marks_the_ones_in_the_collection() {
        let mut conn = open_memory();
        let qin = "QIN ZZ, 2019, SCI REP, V9, P15000";
        let lecun = "LECUN Y, 2015, NATURE, V521, P436, DOI 10.1038/nature14539";
        let items = vec![
            paper("q", "Using artificial intelligence to read chest radiographs for tuberculosis detection", 2019, "Qin ZZ", &[]),
            paper("a", "A", 2021, "Chen L", &[qin, lecun]),
            paper("b", "B", 2022, "Lee J", &[qin, lecun, "HE K, 2016, PROC CVPR, P770"]),
            paper("c", "C", 2023, "Park C", &[lecun]),
        ];
        insert_many(&mut conn, &items, "Test").unwrap();
        let t = compute(&conn, &TopRefParams { top: 20 }).unwrap();
        assert_eq!((t.documents, t.citing, t.works), (4, 3, 3));
        assert_eq!((t.refs[0].label.as_str(), t.refs[0].documents), ("Lecun 2015", 3));
        assert_eq!(t.refs[0].doi.as_deref(), Some("10.1038/nature14539"));
        assert!(t.refs[0].in_collection.is_none());
        assert_eq!(t.refs[1].label, "Qin 2019");
        assert!(t.refs[1].in_collection.is_some(), "matched by surname, year and abbreviated source");
        assert_eq!(t.refs[1].title.as_deref(), Some("Using artificial intelligence to read chest radiographs for tuberculosis detection"));
        assert_eq!(t.missing, 2);
        assert!(compute(&conn, &TopRefParams { top: 5 }).is_err());
    }
}
