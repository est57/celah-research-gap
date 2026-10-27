//! Draft methods and results paragraphs of a bibliometric study, from the collection's
//! figures, with in-text citations of the most cited documents and of the methods used.

mod countries;
mod facts;
mod text;
#[cfg(test)]
mod tests;

use super::citations::{self, Citations};
use super::cite::{self, Style, Work};
use super::text::Lang;
use super::{bibtex, markdown, ReportParagraph};
use crate::error::{AppError, AppResult};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

/// Ids for the two method references, outside the range of stored documents.
const BIBLIOMETRIX: i64 = -1;
const RPYS: i64 = -2;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SummaryParams {
    /// "apa" or "ieee".
    pub style: String,
    /// "id" or "en".
    pub language: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub title: String,
    /// One line reminding that the draft must be checked.
    pub intro: String,
    pub paragraphs: Vec<ReportParagraph>,
    pub references_title: String,
    pub references: Vec<String>,
    pub bibtex: String,
    pub ris: String,
    pub markdown: String,
}

fn method_works() -> [Work; 2] {
    let work = |id, title: &str, year, source: &str, doi: &str, authors: &[&str]| Work {
        id,
        title: title.into(),
        year: Some(year),
        source: Some(source.into()),
        doi: Some(doi.into()),
        doc_type: Some("article".into()),
        authors: authors.iter().map(|a| a.to_string()).collect(),
    };
    [
        work(BIBLIOMETRIX, "bibliometrix: An R-tool for comprehensive science mapping analysis", 2017, "Journal of Informetrics",
            "10.1016/j.joi.2017.08.007", &["Aria, Massimo", "Cuccurullo, Corrado"]),
        work(RPYS, "Detecting the historical roots of research fields by reference publication year spectroscopy (RPYS)", 2014,
            "Journal of the Association for Information Science and Technology", "10.1002/asi.23089",
            &["Marx, Werner", "Bornmann, Lutz", "Barth, Andreas", "Leydesdorff, Loet"]),
    ]
}

pub fn build(conn: &Connection, p: &SummaryParams) -> AppResult<Summary> {
    let style = match p.style.as_str() {
        "apa" => Style::Apa,
        "ieee" => Style::Ieee,
        _ => return Err(AppError::Invalid("gaya sitasi tidak dikenal".into())),
    };
    let lang = match p.language.as_str() {
        "id" => Lang::Id,
        "en" => Lang::En,
        _ => return Err(AppError::Invalid("bahasa tidak dikenal".into())),
    };
    let f = facts::gather(conn)?;
    if f.info.documents == 0 {
        return Err(AppError::Invalid("koleksi masih kosong; tambahkan dokumen di Koleksi & impor".into()));
    }
    let ids: Vec<i64> = f.cited.iter().map(|c| c.0).collect();
    let mut works = cite::load(conn, &ids)?;
    works.extend(method_works().into_iter().map(|w| (w.id, w)));
    let mut cites = Citations { style, works: &works, order: Vec::new() };

    let tools = cites.group(&[BIBLIOMETRIX]);
    let top: Vec<(String, i64)> = f.cited.iter().map(|&(id, n)| (cites.narrative(id), n)).collect();
    let rpys = if f.roots.is_empty() { String::new() } else { cites.group(&[RPYS]) };
    let (heads, title, refs_title) = match lang {
        Lang::Id => (["Data dan metode", "Kinerja publikasi", "Dokumen berpengaruh dan akar sejarah"], "Metode dan hasil bibliometrik", "Daftar pustaka"),
        Lang::En => (["Data and methods", "Publication performance", "Influential documents and historical roots"], "Bibliometric methods and results", "References"),
    };
    let mut sections = vec![
        ReportParagraph { key: "method".into(), heading: heads[0].into(), text: text::method(lang, &f, &tools) },
        ReportParagraph { key: "performance".into(), heading: heads[1].into(), text: text::performance(lang, &f) },
    ];
    if let Some(t) = text::influence(lang, &f, &top, &rpys) {
        sections.push(ReportParagraph { key: "influence".into(), heading: heads[2].into(), text: t });
    }
    let (references, owned) = citations::references(&cites);
    let lead = match lang {
        Lang::Id => "Draf ini disusun otomatis dari koleksi; periksa angka dan sesuaikan kalimatnya sebelum dipakai.",
        Lang::En => "This draft was generated from the collection; check the figures and adapt the wording before use.",
    };
    let markdown = markdown::render(title, lead, &sections, refs_title, &references, style);
    Ok(Summary {
        title: title.into(),
        intro: lead.into(),
        paragraphs: sections,
        references_title: refs_title.into(),
        references,
        bibtex: bibtex::bibtex(&owned),
        ris: bibtex::ris(&owned),
        markdown,
    })
}
