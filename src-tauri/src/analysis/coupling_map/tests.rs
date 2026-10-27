use super::{compute, CouplingMapParams};
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: usize, year: i32, cited: i64, kw: &str, refs: &[&str]) -> NewPaper {
    NewPaper {
        ext_id: format!("p{id}"),
        title: format!("Paper {id}"),
        year: Some(year),
        cited_by: cited,
        author_names: vec![format!("Author{id} A")],
        keywords: Some(kw.into()),
        refs: refs.iter().map(|r| r.to_string()).collect(),
        ..Default::default()
    }
}

#[test]
fn places_coupled_clusters_by_centrality_and_impact() {
    let mut conn = open_memory();
    let cad = ["QIN ZZ, 2019, SCI REP, V9", "LAKHANI P, 2017, RADIOLOGY, V284", "JAEGER S, 2014, QUANT IMAG MED SURG"];
    let audio = ["BOTHA GHR, 2018, PHYSIOL MEAS, V39", "PAHAR M, 2021, COMPUT BIOL MED"];
    let mut items = Vec::new();
    for i in 0..4 {
        // Chest x-ray papers are cited far more than cough papers of the same year.
        items.push(paper(i, 2022, 40, "chest x-ray; deep learning", &cad));
        items.push(paper(10 + i, 2022, 2, "cough sound; deep learning", &audio));
    }
    insert_many(&mut conn, &items, "Test").unwrap();

    let m = compute(&conn, &CouplingMapParams { documents: 50 }).unwrap();
    assert_eq!((m.coupled, m.used, m.clusters.len()), (8, 8, 2));
    let xray = m.clusters.iter().find(|c| c.keywords[0] == "chest x-ray").expect("specific keyword first");
    let cough = m.clusters.iter().find(|c| c.keywords[0] == "cough sound").unwrap();
    assert!(xray.impact > 1.0 && cough.impact < 1.0, "impact is relative to the year average");
    assert!(xray.y > cough.y);
    assert_eq!(xray.size, 4);
    assert_eq!(xray.documents[0].cited_by, 40);
    assert!(compute(&conn, &CouplingMapParams { documents: 10 }).is_err());
}
