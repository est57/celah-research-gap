use serde::Deserialize;
use std::collections::HashMap;

/// Subset of the OpenAlex `works` response. Every field is optional because
/// records from the API are frequently incomplete.
#[derive(Deserialize)]
pub struct WorksPage {
    #[serde(default)]
    pub results: Vec<Work>,
}

#[derive(Deserialize)]
pub struct Work {
    pub id: Option<String>,
    pub doi: Option<String>,
    pub display_name: Option<String>,
    pub publication_year: Option<i32>,
    #[serde(rename = "type")]
    pub work_type: Option<String>,
    #[serde(default)]
    pub cited_by_count: i64,
    #[serde(default)]
    pub authorships: Vec<Authorship>,
    pub primary_location: Option<Location>,
    pub abstract_inverted_index: Option<HashMap<String, Vec<usize>>>,
    #[serde(default)]
    pub keywords: Vec<Keyword>,
    #[serde(default)]
    pub referenced_works_count: i64,
    /// Works this one cites, as OpenAlex URLs.
    #[serde(default)]
    pub referenced_works: Vec<String>,
}

#[derive(Deserialize)]
pub struct Authorship {
    pub author: Option<Author>,
    #[serde(default)]
    pub institutions: Vec<serde_json::Value>,
    /// ISO alpha-2 codes of this author's affiliations.
    #[serde(default)]
    pub countries: Vec<String>,
    pub is_corresponding: Option<bool>,
}

#[derive(Deserialize)]
pub struct Author {
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct Location {
    pub source: Option<Source>,
}

#[derive(Deserialize)]
pub struct Source {
    pub display_name: Option<String>,
    #[serde(default)]
    pub issn: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Keyword {
    pub display_name: Option<String>,
}
