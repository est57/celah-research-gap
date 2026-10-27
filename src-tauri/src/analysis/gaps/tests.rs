use super::model::Kind;
use crate::db::{insert::insert_many, open_memory, papers::NewPaper, reviews};

fn paper(id: usize, year: i32, kw: &str, abstract_text: Option<&str>) -> NewPaper {
    NewPaper {
        ext_id: format!("p{id}"),
        title: format!("Paper {id}"),
        year: Some(year),
        cited_by: id as i64 % 7,
        keywords: Some(kw.into()),
        abstract_text: abstract_text.map(Into::into),
        ..Default::default()
    }
}

fn collection() -> Vec<NewPaper> {
    let mut items = Vec::new();
    for i in 0..30 {
        items.push(paper(i, 2016 + (i % 6) as i32, "deep learning; chest x-ray; cnn", None));
        items.push(paper(100 + i, 2017 + (i % 5) as i32, "cough sound; audio; triage", None));
    }
    for i in 0..4 {
        let text = "Audio models look promising. Robustness to smartphone noise remains unclear.";
        items.push(paper(200 + i, 2024, "cough sound; smartphone", Some(text)));
    }
    items
}

#[test]
fn every_gap_has_evidence_and_a_valid_score() {
    let mut conn = open_memory();
    insert_many(&mut conn, &collection(), "Test").unwrap();
    let list = super::compute(&conn).unwrap();
    assert!(!list.gaps.is_empty());
    for g in &list.gaps {
        assert!(!g.evidence.is_empty(), "{} has no evidence", g.key);
        assert!((0.0..=1.0).contains(&g.score));
        assert!(g.key.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | ':')));
    }
    let matrix = list.gaps.iter().find(|g| g.kind == Kind::Matrix).expect("deep learning and cough sound never meet");
    assert!(matrix.statement.contains("0 dokumen"));
    let explicit = list.gaps.iter().find(|g| g.kind == Kind::Explicit).expect("four abstracts say remains unclear");
    assert!(explicit.evidence.iter().all(|e| e.quote.as_deref().is_some_and(|q| q.contains("remains unclear"))));
}

#[test]
fn review_status_survives_recomputation() {
    let mut conn = open_memory();
    insert_many(&mut conn, &collection(), "Test").unwrap();
    let key = super::compute(&conn).unwrap().gaps[0].key.clone();
    reviews::set(&conn, &key, "accepted", "cek dataset lokal", "").unwrap();
    let again = super::compute(&conn).unwrap();
    let g = again.gaps.iter().find(|g| g.key == key).unwrap();
    assert_eq!((g.status.as_str(), g.note.as_str()), ("accepted", "cek dataset lokal"));
}

#[test]
fn empty_collection_has_no_gaps() {
    let conn = open_memory();
    assert!(super::compute(&conn).unwrap().gaps.is_empty());
}
