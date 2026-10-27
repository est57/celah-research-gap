//! Data for the "Struktur konseptual" screen: thematic map and trend topics.

use super::cooccur::{self, Field};
use super::thematic::{self, ThematicMap};
use super::trends::{self, Trend};
use super::current_year;
use crate::error::{AppError, AppResult};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

const TRENDS_PER_YEAR: usize = 3;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConceptualParams {
    /// "auto", "author" or "plus".
    pub field: String,
    pub words: u32,
    /// Minimum theme frequency per 1,000 documents.
    pub min_cluster_freq: u32,
    pub labels: u32,
    pub trend_min_freq: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Conceptual {
    pub field: &'static str,
    pub words_used: usize,
    pub documents: usize,
    pub map: ThematicMap,
    pub trends: Vec<Trend>,
}

pub fn in_range(value: u32, min: u32, max: u32, what: &str) -> AppResult<u32> {
    if (min..=max).contains(&value) {
        Ok(value)
    } else {
        Err(AppError::Invalid(format!("{what} harus {min} sampai {max}")))
    }
}

/// "auto" picks the keyword field with more filled-in documents.
pub fn field_choice(conn: &Connection, raw: &str) -> AppResult<Field> {
    match raw {
        "auto" => cooccur::best_field(conn),
        "author" => Ok(Field::AuthorKeywords),
        "plus" => Ok(Field::KeywordsPlus),
        _ => Err(AppError::Invalid("field kata kunci tidak dikenal".into())),
    }
}

pub fn compute(conn: &Connection, p: &ConceptualParams) -> AppResult<Conceptual> {
    let words = in_range(p.words, 20, 1000, "jumlah kata")? as usize;
    let min_freq = in_range(p.min_cluster_freq, 0, 100, "frekuensi minimum tema")?;
    let labels = in_range(p.labels, 1, 5, "label per tema")? as usize;
    let trend_min = in_range(p.trend_min_freq, 1, 100, "frekuensi minimum tren")? as usize;
    let field = field_choice(conn, &p.field)?;
    let year = current_year(conn)?;
    let net = cooccur::build(conn, field, words)?;
    Ok(Conceptual {
        field: field.label(),
        words_used: net.labels.len(),
        documents: net.years.len(),
        map: thematic::compute(&net, min_freq, labels, year),
        trends: trends::compute(&net, trend_min, TRENDS_PER_YEAR),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    fn paper(id: usize, year: i32, kw: &str) -> NewPaper {
        NewPaper { ext_id: format!("p{id}"), title: format!("Paper {id}"), year: Some(year), keywords: Some(kw.into()), ..Default::default() }
    }

    fn params() -> ConceptualParams {
        ConceptualParams { field: "auto".into(), words: 50, min_cluster_freq: 0, labels: 2, trend_min_freq: 2 }
    }

    #[test]
    fn finds_two_separate_themes() {
        let mut conn = open_memory();
        let mut items = Vec::new();
        for i in 0..6 {
            items.push(paper(i, 2018 + i as i32 % 3, "deep learning; chest x-ray; cnn"));
            items.push(paper(100 + i, 2023 + i as i32 % 2, "cough sound; audio; smartphone"));
        }
        insert_many(&mut conn, &items, "Test").unwrap();
        let c = compute(&conn, &params()).unwrap();
        assert_eq!(c.field, "Kata kunci penulis");
        assert_eq!(c.map.themes.len(), 2);
        assert!(c.map.themes.iter().all(|t| t.words.len() == 3 && t.documents == 6));
        let audio = c.map.themes.iter().find(|t| t.name.contains("audio")).unwrap();
        assert!(audio.recent_share > c.map.collection_recent_share, "the audio theme is newer than the collection");
        assert!(c.trends.iter().any(|t| t.label == "cough sound" && t.median >= 2023));
    }

    #[test]
    fn rejects_out_of_range_params() {
        let conn = open_memory();
        let mut p = params();
        p.words = 5;
        assert!(compute(&conn, &p).is_err());
        let mut p = params();
        p.field = "keywords; DROP TABLE papers".into();
        assert!(compute(&conn, &p).is_err());
    }
}
