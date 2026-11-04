use super::render;
use crate::report::cite::Style;
use crate::report::ReportParagraph;
use std::io::{Cursor, Read};

fn part(bytes: &[u8], name: &str) -> String {
    let mut zip = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip package");
    let mut out = String::new();
    zip.by_name(name).expect(name).read_to_string(&mut out).unwrap();
    out
}

#[test]
fn writes_a_word_package_with_escaped_text() {
    let paragraphs = vec![ReportParagraph { key: "k".into(), heading: "A & B <rare>".into(), text: "Studi \"X\" (Chen et al., 2024).\u{1}".into() }];
    let refs = vec!["Chen, L. (2024). Cough audio. Contoh Jurnal.".to_string()];
    let bytes = render("Celah penelitian", "Draf otomatis.", &paragraphs, "Daftar pustaka", &refs, Style::Apa).unwrap();
    let names: Vec<String> = zip::ZipArchive::new(Cursor::new(&bytes[..])).unwrap().file_names().map(|n| n.unwrap().to_string()).collect();
    for needed in ["[Content_Types].xml", "_rels/.rels", "word/document.xml", "word/styles.xml", "word/_rels/document.xml.rels", "docProps/core.xml"] {
        assert!(names.iter().any(|n| n == needed), "missing {needed}");
    }
    let doc = part(&bytes, "word/document.xml");
    assert!(doc.contains("1. A &amp; B &lt;rare&gt;"));
    assert!(doc.contains("Studi &quot;X&quot; (Chen et al., 2024)."), "control characters are dropped");
    assert!(!doc.contains('\u{1}'));
    assert!(doc.contains(r#"<w:pStyle w:val="Reference"/>"#), "APA references use the hanging indent");
    assert!(part(&bytes, "docProps/core.xml").contains("<dc:title>Celah penelitian</dc:title>"));
}

#[test]
fn ieee_references_are_plain_paragraphs() {
    let bytes = render("T", "L", &[], "References", &["[1] A. Chen, ...".to_string()], Style::Ieee).unwrap();
    let doc = part(&bytes, "word/document.xml");
    assert!(!doc.contains(r#"w:val="Reference""#));
    assert!(doc.contains("[1] A. Chen, ..."));
}
