//! Draft research-gap section and bibliography from the accepted gaps.

pub mod bibtex;
pub mod cite;
mod citations;
mod markdown;
pub(crate) mod names;
pub mod summary;
mod text;

use crate::analysis::gaps::{self, model::Gap};
use crate::error::{AppError, AppResult};
use citations::{evidence_ids, Citations};
use cite::Style;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use text::Lang;

const MAIN_CITES: usize = 3;
const NEAR_CITES: usize = 2;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReportParams {
    /// "apa" or "ieee".
    pub style: String,
    /// "id" or "en".
    pub language: String,
    pub include_verification: bool,
}

#[derive(Serialize)]
pub struct ReportParagraph {
    pub key: String,
    pub heading: String,
    pub text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub title: String,
    pub intro: String,
    pub paragraphs: Vec<ReportParagraph>,
    pub references_title: String,
    pub references: Vec<String>,
    pub bibtex: String,
    pub ris: String,
    pub markdown: String,
}

pub fn build(conn: &Connection, p: &ReportParams) -> AppResult<Report> {
    let style = match p.style.as_str() {
        "apa" => Style::Apa,
        "ieee" => Style::Ieee,
        _ => return Err(AppError::Invalid(crate::tr!("gaya sitasi tidak dikenal", "unknown citation style").into())),
    };
    let lang = match p.language.as_str() {
        "id" => Lang::Id,
        "en" => Lang::En,
        _ => return Err(AppError::Invalid(crate::tr!("bahasa tidak dikenal", "unknown language").into())),
    };
    let list = gaps::compute(conn)?;
    let accepted: Vec<&Gap> = list.gaps.iter().filter(|g| g.status == "accepted").collect();
    if accepted.is_empty() {
        return Err(AppError::Invalid(
            crate::tr!(
                "belum ada celah yang diterima; tandai celah sebagai Diterima di Gap Explorer",
                "no accepted gap yet; mark gaps as Accepted in Gap Explorer"
            )
            .into(),
        ));
    }
    let ids: Vec<i64> = accepted.iter().flat_map(|g| g.evidence.iter().map(|e| e.paper_id)).collect();
    let works = cite::load(conn, &ids)?;
    let mut cites = Citations { style, works: &works, order: Vec::new() };

    let paragraphs: Vec<ReportParagraph> = accepted
        .iter()
        .enumerate()
        .map(|(i, g)| {
            let main = cites.group(&evidence_ids(g, false, MAIN_CITES));
            let near = cites.group(&evidence_ids(g, true, NEAR_CITES));
            let mut parts = vec![text::claim(lang, g.kind, &g.terms, &main, i)];
            parts.extend(text::closest(lang, &near));
            if p.include_verification {
                parts.extend(g.check.as_ref().map(|c| text::verification(lang, c)));
            }
            parts.push(text::question(lang, g.kind, &g.terms));
            let heading = match lang {
                Lang::Id => g.title.clone(),
                Lang::En => text::heading_en(&g.terms),
            };
            ReportParagraph { key: g.key.clone(), heading, text: parts.join(" ") }
        })
        .collect();

    let (references, owned) = citations::references(&cites);
    let (title, references_title) = match lang {
        Lang::Id => ("Celah penelitian", "Daftar pustaka"),
        Lang::En => ("Research gaps", "References"),
    };
    let intro = text::intro(lang, list.documents, list.field, accepted.len());
    let markdown = markdown::render(title, &intro, &paragraphs, references_title, &references, style);
    Ok(Report {
        title: title.into(),
        intro,
        paragraphs,
        references_title: references_title.into(),
        references,
        bibtex: bibtex::bibtex(&owned),
        ris: bibtex::ris(&owned),
        markdown,
    })
}

#[cfg(test)]
mod tests;
