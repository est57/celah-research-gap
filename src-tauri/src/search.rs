//! Source-independent search request and the list of searchable sources.

use crate::secrets::Service;
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Openalex,
    SemanticScholar,
    Crossref,
    Pubmed,
    EuropePmc,
    Arxiv,
}

impl Source {
    pub fn label(self) -> &'static str {
        match self {
            Source::Openalex => "OpenAlex",
            Source::SemanticScholar => "Semantic Scholar",
            Source::Crossref => "Crossref",
            Source::Pubmed => "PubMed",
            Source::EuropePmc => "Europe PMC",
            Source::Arxiv => "arXiv",
        }
    }

    /// The keychain entry holding this source's API key, if it uses one.
    pub fn key_service(self) -> Option<Service> {
        match self {
            Source::Openalex => Some(Service::Openalex),
            Source::SemanticScholar => Some(Service::SemanticScholar),
            Source::Pubmed => Some(Service::Ncbi),
            Source::Crossref | Source::EuropePmc | Source::Arxiv => None,
        }
    }
}

/// Already validated search parameters.
pub struct SearchQuery {
    pub text: String,
    pub year_from: i32,
    pub year_to: i32,
    /// One of `validate::DOC_TYPES`.
    pub doc_type: Option<&'static str>,
    pub max_results: u32,
}
