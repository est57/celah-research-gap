use super::{build, ReportParams};
use crate::analysis::gaps;
use crate::db::{insert::insert_many, open_memory, papers::NewPaper, reviews};

fn paper(id: usize, year: i32, kw: &str) -> NewPaper {
    NewPaper {
        ext_id: format!("p{id}"),
        title: format!("Study {id} on {kw}"),
        year: Some(year),
        cited_by: (id % 9) as i64,
        keywords: Some(kw.into()),
        author_names: vec![format!("Author{id}, A."), "Chen, L.".into()],
        source_name: Some("PLOS ONE".into()),
        doi: Some(format!("10.1/s{id}")),
        ..Default::default()
    }
}

fn setup() -> rusqlite::Connection {
    let mut conn = open_memory();
    let mut items = Vec::new();
    for i in 0..30 {
        items.push(paper(i, 2016 + (i % 6) as i32, "deep learning; chest x-ray; cnn"));
        items.push(paper(100 + i, 2017 + (i % 5) as i32, "cough sound; audio; triage"));
    }
    insert_many(&mut conn, &items, "Test").unwrap();
    let first = gaps::compute(&conn).unwrap().gaps[0].key.clone();
    reviews::set(&conn, &first, "accepted", "", "").unwrap();
    conn
}

fn params(style: &str, language: &str) -> ReportParams {
    ReportParams { style: style.into(), language: language.into(), include_verification: true }
}

#[test]
fn builds_apa_report_in_indonesian() {
    let conn = setup();
    let r = build(&conn, &params("apa", "id")).unwrap();
    assert_eq!(r.paragraphs.len(), 1);
    let text = &r.paragraphs[0].text;
    assert!(text.contains("(") && text.contains("et al.") || text.contains(" & "), "APA citations in the text: {text}");
    assert!(!r.references.is_empty());
    assert!(r.references.iter().all(|x| x.contains("https://doi.org/10.1/s")));
    assert_eq!(r.bibtex.matches("@article{").count(), r.references.len());
    assert!(r.markdown.starts_with("# Celah penelitian"));
    assert!(r.markdown.contains("## Daftar pustaka"));
}

#[test]
fn builds_numbered_ieee_report_in_english() {
    let conn = setup();
    let r = build(&conn, &params("ieee", "en")).unwrap();
    assert!(r.paragraphs[0].text.contains("[1]"));
    assert!(r.references[0].starts_with("[1] "));
    assert!(r.markdown.contains("## References"));
    assert!(r.ris.contains("TY  - JOUR"));
}

#[test]
fn rejects_bad_params_and_empty_selection() {
    let conn = setup();
    assert!(build(&conn, &params("chicago", "id")).is_err());
    assert!(build(&conn, &params("apa", "fr")).is_err());
    let empty = open_memory();
    assert!(build(&empty, &params("apa", "id")).is_err(), "no accepted gaps");
}
