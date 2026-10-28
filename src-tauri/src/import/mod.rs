mod bibtex;
pub mod limits;
mod pubmed;
mod ris;
mod scopus_bib;
pub mod sjr;
mod scopus_csv;
mod tagged;
mod wos;

use crate::db::papers::NewPaper;
use crate::error::{AppError, AppResult};
use std::io::Read;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    Wos,
    ScopusCsv,
    ScopusBib,
    ScopusRis,
    Ris,
    Pubmed,
}

impl Format {
    pub fn origin(self) -> &'static str {
        match self {
            Format::Wos => "Web of Science",
            Format::ScopusCsv | Format::ScopusBib | Format::ScopusRis => "Scopus",
            Format::Ris => "RIS",
            Format::Pubmed => "PubMed",
        }
    }
}

/// Reads a user-chosen file with a hard size cap. Invalid UTF-8 is replaced, not rejected.
pub fn read_text(path: &Path) -> AppResult<String> {
    let file = std::fs::File::open(path).map_err(|_| AppError::Invalid("file tidak bisa dibuka".into()))?;
    let len = file.metadata().map(|m| m.len()).unwrap_or(0);
    if len > limits::MAX_FILE_BYTES {
        return Err(AppError::Invalid("ukuran file melebihi 50 MB".into()));
    }
    let mut bytes = Vec::with_capacity(len as usize);
    file.take(limits::MAX_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| AppError::Invalid("file tidak bisa dibaca".into()))?;
    if bytes.len() as u64 > limits::MAX_FILE_BYTES {
        return Err(AppError::Invalid("ukuran file melebihi 50 MB".into()));
    }
    let text = String::from_utf8_lossy(&bytes);
    Ok(text.trim_start_matches('\u{feff}').to_string())
}

/// Decides the format from the extension and the first bytes of content.
pub fn detect(path: &Path, text: &str) -> AppResult<Format> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    let head = text.trim_start();
    let ris = head.starts_with("TY  -");
    let pubmed = head.starts_with("PMID-");
    match ext.as_str() {
        "txt" if head.starts_with("FN ") || head.starts_with("PT ") => Ok(Format::Wos),
        "txt" | "nbib" if pubmed => Ok(Format::Pubmed),
        "txt" | "ris" if ris && text.contains("\nDB  - Scopus") => Ok(Format::ScopusRis),
        "txt" | "ris" if ris => Ok(Format::Ris),
        "csv" => Ok(Format::ScopusCsv),
        "bib" if head.contains('@') => Ok(Format::ScopusBib),
        "ris" | "nbib" => Err(AppError::Invalid(format!("isi file .{ext} tidak sesuai formatnya"))),
        "txt" => Err(AppError::Invalid(
            "file .txt bukan ekspor Web of Science (Plain text), PubMed, atau RIS".into(),
        )),
        _ => Err(AppError::Invalid("format file tidak dikenali; gunakan .bib, .csv, .ris, .nbib, atau .txt".into())),
    }
}

pub fn parse(format: Format, text: &str) -> Vec<NewPaper> {
    match format {
        Format::Wos => wos::parse(text),
        Format::ScopusCsv => scopus_csv::parse(text),
        Format::ScopusBib => scopus_bib::parse(text),
        Format::ScopusRis | Format::Ris => ris::parse(text),
        Format::Pubmed => pubmed::parse(text),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_formats() {
        let p = |n: &str| Path::new(n).to_path_buf();
        assert_eq!(detect(&p("a.txt"), "FN Clarivate\nVR 1.0").unwrap(), Format::Wos);
        assert_eq!(detect(&p("a.CSV"), "Authors,Title").unwrap(), Format::ScopusCsv);
        assert_eq!(detect(&p("a.bib"), "@ARTICLE{x,}").unwrap(), Format::ScopusBib);
        assert_eq!(detect(&p("a.ris"), "TY  - JOUR\nER  - ").unwrap(), Format::Ris);
        assert_eq!(detect(&p("a.ris"), "TY  - JOUR\nDB  - Scopus\nER  - ").unwrap(), Format::ScopusRis);
        assert_eq!(detect(&p("pubmed-x.txt"), "PMID- 1\nTI  - x").unwrap(), Format::Pubmed);
        assert_eq!(detect(&p("a.nbib"), "\nPMID- 1").unwrap(), Format::Pubmed);
        assert!(detect(&p("a.ris"), "hello").is_err());
        assert!(detect(&p("a.txt"), "hello").is_err());
        assert!(detect(&p("a.exe"), "MZ").is_err());
    }
}
