use serde::Deserialize;
use std::collections::HashMap;

/// Subset of the Semantic Scholar Graph API `paper/search` response.
/// Lists are optional because the API returns `null` for missing ones.
#[derive(Deserialize)]
pub struct SearchPage {
    pub data: Option<Vec<Paper>>,
    pub next: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paper {
    pub paper_id: Option<String>,
    pub external_ids: Option<HashMap<String, serde_json::Value>>,
    pub title: Option<String>,
    #[serde(rename = "abstract")]
    pub abstract_text: Option<String>,
    pub venue: Option<String>,
    pub year: Option<i32>,
    pub reference_count: Option<i64>,
    pub citation_count: Option<i64>,
    pub publication_types: Option<Vec<String>>,
    pub authors: Option<Vec<Author>>,
    pub journal: Option<Journal>,
}

#[derive(Deserialize)]
pub struct Author {
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct Journal {
    pub name: Option<String>,
}
