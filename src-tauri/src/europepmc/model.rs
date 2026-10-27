use serde::Deserialize;

/// Subset of the Europe PMC REST `search` response (resultType=core).
/// Every list is optional because the API omits empty ones.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub next_cursor_mark: Option<String>,
    pub result_list: Option<ResultList>,
}

#[derive(Deserialize)]
pub struct ResultList {
    pub result: Option<Vec<Record>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub id: Option<String>,
    pub source: Option<String>,
    pub pmid: Option<String>,
    pub doi: Option<String>,
    pub title: Option<String>,
    pub pub_year: Option<String>,
    pub cited_by_count: Option<i64>,
    pub abstract_text: Option<String>,
    pub author_list: Option<AuthorList>,
    pub journal_info: Option<JournalInfo>,
    pub keyword_list: Option<KeywordList>,
    pub mesh_heading_list: Option<MeshList>,
    pub pub_type_list: Option<PubTypeList>,
}

#[derive(Deserialize)]
pub struct AuthorList {
    pub author: Option<Vec<Author>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub full_name: Option<String>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub author_affiliation_details_list: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct JournalInfo {
    pub journal: Option<Journal>,
}

#[derive(Deserialize)]
pub struct Journal {
    pub title: Option<String>,
}

#[derive(Deserialize)]
pub struct KeywordList {
    pub keyword: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshList {
    pub mesh_heading: Option<Vec<Mesh>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mesh {
    pub descriptor_name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubTypeList {
    pub pub_type: Option<Vec<String>>,
}
