use super::*;
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: usize, year: i32, title: &str, kw: &str) -> NewPaper {
    NewPaper { ext_id: format!("p{id}"), title: format!("{title} {id}"), year: Some(year), cited_by: id as i64, keywords: Some(kw.into()), ..Default::default() }
}

#[test]
fn counts_concepts_alone_and_together() {
    let mut conn = open_memory();
    let mut items = Vec::new();
    for i in 0..10 {
        items.push(paper(i, 2020 + (i as i32 % 4), "Vision transformers for chest radiographs", "deep learning"));
        items.push(paper(20 + i, 2019, "Automated sputum smear microscopy", "microscopy"));
    }
    items.push(paper(40, 2024, "ViT on sputum smear images", "microscopy"));
    insert_many(&mut conn, &items, "Test").unwrap();
    let r = compute(&conn, &IdeaParams { a: "vision transformer, ViT".into(), b: "sputum smear".into() }).unwrap();
    assert_eq!((r.documents, r.with_a, r.with_b, r.with_both), (21, 11, 11, 1));
    assert!((r.expected - 11.0 * 11.0 / 21.0).abs() < 1e-9);
    assert_eq!(r.verdict, "celah");
    assert_eq!(r.both_docs[0].title, "ViT on sputum smear images 40");
    assert_eq!(r.a_only.len(), 3);
    assert_eq!(r.years.first(), Some(&2019));
    let i2024 = r.years.iter().position(|y| *y == 2024).unwrap();
    assert_eq!(r.per_year[2][i2024], 1);

    let apart = compute(&conn, &IdeaParams { a: "transformers vision".into(), b: "sputum smear".into() }).unwrap();
    assert_eq!(apart.with_a, 0, "words of a phrase must be consecutive and in order");
    let few = compute(&conn, &IdeaParams { a: "quantum".into(), b: "sputum smear".into() }).unwrap();
    assert_eq!(few.verdict, "kurang-data");
    assert!(compute(&conn, &IdeaParams { a: "".into(), b: "x y".into() }).is_err());
    assert!(compute(&conn, &IdeaParams { a: "a, b, c, d, e, f".into(), b: "smear".into() }).is_err());
}
