use super::*;
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: usize, kw: &str) -> NewPaper {
    NewPaper { ext_id: format!("p{id}"), title: format!("Paper {id}"), year: Some(2020), keywords: Some(kw.into()), ..Default::default() }
}

#[test]
fn separates_two_research_streams() {
    let mut conn = open_memory();
    let mut items = Vec::new();
    for i in 0..12 {
        let imaging = ["deep learning", "chest x-ray", "cnn", "segmentation"];
        let audio = ["cough sound", "audio", "smartphone", "spectrogram"];
        let pick = |set: [&str; 4]| [set[i % 4], set[(i + 1) % 4], set[(i + 2) % 4]].join("; ");
        items.push(paper(i, &pick(imaging)));
        items.push(paper(100 + i, &pick(audio)));
    }
    insert_many(&mut conn, &items, "Test").unwrap();
    let f = compute(&conn, &FactorialParams { field: "auto".into(), words: 20, clusters: 0 }).unwrap();
    assert_eq!((f.words.len(), f.documents), (8, 24));
    assert_eq!(f.clusters.len(), 2, "{:?}", f.clusters.iter().map(|c| &c.top).collect::<Vec<_>>());
    let cluster_of = |w: &str| f.words.iter().find(|x| x.label == w).unwrap().cluster;
    assert_eq!(cluster_of("cnn"), cluster_of("chest x-ray"));
    assert_ne!(cluster_of("cnn"), cluster_of("audio"));
    // The streams sit on opposite sides of the first axis.
    let x = |w: &str| f.words.iter().find(|x| x.label == w).unwrap().x;
    assert!(x("cnn") * x("audio") < 0.0);
    assert!(f.explained[0] > 0.5);
    assert_eq!(f.top_documents.len(), 10);
}

#[test]
fn rejects_bad_params_and_handles_tiny_collections() {
    let mut conn = open_memory();
    let bad = |words, clusters| compute(&conn, &FactorialParams { field: "auto".into(), words, clusters }).is_err();
    assert!(bad(5, 0));
    assert!(bad(50, 9));
    insert_many(&mut conn, &[paper(1, "a; b")], "Test").unwrap();
    let f = compute(&conn, &FactorialParams { field: "auto".into(), words: 20, clusters: 0 }).unwrap();
    assert!(f.words.is_empty());
}
