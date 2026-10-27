//! Data for the "Sumber & penulis" screen.

use super::{authors, current_year, lotka, sources};
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;

const TOP_SOURCES: usize = 10;
const TOP_TIMELINE: usize = 10;
const TOP_IMPACT: usize = 15;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcesAuthors {
    pub current_year: i32,
    pub total_sources: usize,
    pub top_sources: Vec<sources::NamedCount>,
    pub bradford: sources::Bradford,
    pub lotka: lotka::Lotka,
    pub production: Vec<authors::Timeline>,
    pub impact: Vec<authors::Impact>,
}

pub fn compute(conn: &Connection) -> AppResult<SourcesAuthors> {
    let year = current_year(conn)?;
    let ranked = sources::ranked_sources(conn)?;
    let by_author = authors::load(conn)?;

    let mut most_productive: Vec<&authors::AuthorPapers> = by_author.values().collect();
    most_productive.sort_by(|a, b| b.papers.len().cmp(&a.papers.len()).then(a.label.cmp(&b.label)));
    let production = most_productive.iter().take(TOP_TIMELINE).map(|a| authors::timeline(a, year)).collect();

    let mut impact: Vec<authors::Impact> = by_author.values().map(|a| authors::impact(a, year)).collect();
    impact.sort_by(|a, b| b.h.cmp(&a.h).then(b.citations.cmp(&a.citations)).then(a.label.cmp(&b.label)));
    impact.truncate(TOP_IMPACT);

    Ok(SourcesAuthors {
        current_year: year,
        total_sources: ranked.len(),
        top_sources: ranked.iter().take(TOP_SOURCES).cloned().collect(),
        bradford: sources::bradford(&ranked),
        lotka: lotka::compute(by_author.values().map(|a| a.papers.len() as i64)),
        production,
        impact,
    })
}

#[cfg(test)]
mod tests {
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    fn paper(id: &str, year: i32, cited: i64, authors: &[&str], source: &str) -> NewPaper {
        NewPaper {
            ext_id: id.into(),
            title: format!("Paper {id}"),
            year: Some(year),
            cited_by: cited,
            author_names: authors.iter().map(|a| a.to_string()).collect(),
            author_count: authors.len() as i64,
            source_name: Some(source.into()),
            ..Default::default()
        }
    }

    #[test]
    fn ranks_sources_and_authors() {
        let mut conn = open_memory();
        let items = vec![
            paper("a", 2020, 10, &["Chen L", "Rahman M"], "PLOS ONE"),
            paper("b", 2021, 5, &["Chen L"], "plos one"),
            paper("c", 2022, 1, &["Chen L", "Okafor C"], "IEEE Access"),
        ];
        insert_many(&mut conn, &items, "Test").unwrap();
        let s = super::compute(&conn).unwrap();
        assert_eq!(s.total_sources, 2);
        assert_eq!((s.top_sources[0].count, s.bradford.zones[0].sources), (2, 1));
        assert_eq!(s.production[0].label, "Chen L");
        assert_eq!(s.production[0].years.len(), 3);
        let chen = &s.impact[0];
        assert_eq!((chen.label.as_str(), chen.h, chen.g, chen.documents), ("Chen L", 2, 3, 3));
        assert_eq!(s.lotka.total_authors, 3);
        assert_eq!(s.lotka.rows[0].authors, 2, "two authors wrote one document");
    }
}
