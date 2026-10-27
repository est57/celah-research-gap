use super::model::Kind;
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: usize, cited: i64, kw: &str, refs: &[&str]) -> NewPaper {
    NewPaper {
        ext_id: format!("p{id}"),
        title: format!("Paper {id}"),
        year: Some(2022),
        cited_by: cited,
        author_names: vec![format!("Author{id} A")],
        keywords: Some(kw.into()),
        refs: refs.iter().map(|r| r.to_string()).collect(),
        ..Default::default()
    }
}

#[test]
fn flags_a_highly_cited_front_that_shares_no_references_with_the_main_ones() {
    let mut conn = open_memory();
    let (a, b) = (["QIN ZZ, 2019, SCI REP", "LAKHANI P, 2017, RADIOLOGY"], ["HE K, 2016, PROC CVPR", "SIMONYAN K, 2015, ICLR"]);
    let bridge = "LECUN Y, 2015, NATURE";
    let mut items = Vec::new();
    for i in 0..4 {
        let link = if i == 0 { vec![bridge] } else { vec![] };
        items.push(paper(i, 10, "chest x-ray; deep learning", &[a.as_slice(), &link].concat()));
        items.push(paper(10 + i, 10, "cnn architecture; deep learning", &[b.as_slice(), &link].concat()));
    }
    for i in 0..3 {
        items.push(paper(20 + i, 100, "cough sound; smartphone", &["BOTHA GHR, 2018, PHYSIOL MEAS", "PAHAR M, 2021, COMPUT BIOL MED"]));
        items.push(paper(30 + i, 0, "sputum smear; microscopy", &["STEINGART KR, 2006, LANCET INFECT DIS", "DORMAN SE, 2018, LANCET"]));
    }
    insert_many(&mut conn, &items, "Test").unwrap();

    let list = super::compute(&conn).unwrap();
    let fronts: Vec<_> = list.gaps.iter().filter(|g| g.kind == Kind::Front).collect();
    assert_eq!(fronts.len(), 1, "only the cited, isolated front; the uncited one is not a gap");
    let g = fronts[0];
    assert_eq!(g.terms[0], "cough sound");
    assert!(g.key.starts_with("front:cough-sound:"));
    assert!(g.evidence.iter().any(|e| e.role == "mendukung"));
    assert!(g.evidence.iter().any(|e| e.role == "konteks"), "the main front is shown for contrast");
}
