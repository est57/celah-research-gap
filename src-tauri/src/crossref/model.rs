use serde::Deserialize;

/// Subset of the Crossref REST API `works` response.
/// Lists are optional because Crossref omits or nulls them freely.
#[derive(Deserialize)]
pub struct Page {
    pub message: Message,
}

#[derive(Deserialize)]
pub struct Message {
    pub items: Option<Vec<Item>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Item {
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
    pub title: Option<Vec<String>>,
    pub author: Option<Vec<Author>>,
    pub container_title: Option<Vec<String>>,
    #[serde(rename = "ISSN")]
    pub issn: Option<Vec<String>>,
    pub issued: Option<Issued>,
    #[serde(rename = "type")]
    pub work_type: Option<String>,
    pub is_referenced_by_count: Option<i64>,
    #[serde(rename = "abstract")]
    pub abstract_text: Option<String>,
    pub references_count: Option<i64>,
    pub reference: Option<Vec<Reference>>,
}

#[derive(Deserialize)]
pub struct Author {
    pub given: Option<String>,
    pub family: Option<String>,
    /// Organisation authors carry `name` instead of given/family.
    pub name: Option<String>,
    pub affiliation: Option<Vec<serde_json::Value>>,
}

#[derive(Deserialize)]
pub struct Issued {
    #[serde(rename = "date-parts")]
    pub date_parts: Option<Vec<Vec<Option<i64>>>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Reference {
    pub unstructured: Option<String>,
    pub author: Option<String>,
    pub year: Option<String>,
    pub journal_title: Option<String>,
    pub article_title: Option<String>,
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
}
