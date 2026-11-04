//! Word (.docx) rendering of a report: the same title, lead, numbered sections and
//! reference list as the Markdown version, written as a minimal Office Open XML package.

use super::cite::Style;
use super::docx_parts as parts;
use super::ReportParagraph;
use crate::error::{AppError, AppResult};
use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;

/// Text safe inside XML: markup characters escaped, characters XML forbids dropped.
fn esc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\t' | '\n' | '\r' => out.push(' '),
            c if (c as u32) < 0x20 || c == '\u{FFFE}' || c == '\u{FFFF}' => {}
            c => out.push(c),
        }
    }
    out
}

fn para(style: &str, text: &str) -> String {
    format!(r#"<w:p><w:pPr><w:pStyle w:val="{style}"/></w:pPr><w:r><w:t xml:space="preserve">{}</w:t></w:r></w:p>"#, esc(text))
}

/// The document body, in the same order as `markdown::render`.
fn body(title: &str, lead: &str, paragraphs: &[ReportParagraph], refs_title: &str, refs: &[String], style: Style) -> String {
    let mut out = para("Title", title);
    out.push_str(&para("Note", lead));
    for (i, p) in paragraphs.iter().enumerate() {
        out.push_str(&para("Heading2", &format!("{}. {}", i + 1, p.heading)));
        out.push_str(&para("Normal", &p.text));
    }
    out.push_str(&para("Heading1", refs_title));
    // APA uses a hanging indent; IEEE entries already carry their number.
    let ref_style = if style == Style::Apa { "Reference" } else { "Normal" };
    for r in refs {
        out.push_str(&para(ref_style, r));
    }
    out
}

/// The .docx file as bytes.
pub fn render(title: &str, lead: &str, paragraphs: &[ReportParagraph], refs_title: &str, refs: &[String], style: Style) -> AppResult<Vec<u8>> {
    let failed = |_| AppError::Invalid(crate::tr!("dokumen Word tidak bisa dibuat", "the Word document could not be created").into());
    let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let files = [
        ("[Content_Types].xml", parts::CONTENT_TYPES.to_string()),
        ("_rels/.rels", parts::ROOT_RELS.to_string()),
        ("docProps/core.xml", parts::core(&esc(title))),
        ("word/_rels/document.xml.rels", parts::DOCUMENT_RELS.to_string()),
        ("word/styles.xml", parts::STYLES.to_string()),
        ("word/document.xml", parts::document(&body(title, lead, paragraphs, refs_title, refs, style))),
    ];
    for (name, content) in files {
        zip.start_file(name, options).map_err(failed)?;
        zip.write_all(content.as_bytes()).map_err(|_| failed(zip::result::ZipError::FileNotFound))?;
    }
    Ok(zip.finish().map_err(failed)?.into_inner())
}

#[cfg(test)]
#[path = "docx_tests.rs"]
mod tests;
