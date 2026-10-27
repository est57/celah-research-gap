use super::*;
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(ext: &str, year: i32, title: &str, author: &str) -> NewPaper {
    NewPaper {
        ext_id: ext.into(),
        title: title.into(),
        year: Some(year),
        source_name: Some("Scientific Reports".into()),
        author_names: vec![author.into()],
        ..Default::default()
    }
}

#[test]
fn links_documents_by_id_doi_and_name() {
    let mut conn = open_memory();
    let mut root = paper("openalex:W1", 2017, "Deep learning at chest radiography for tuberculosis", "Paras Lakhani");
    root.doi = Some("10.1148/radiol.2017162326".into());
    let qin = paper("scopus:2", 2019, "Using artificial intelligence to read chest radiographs", "Qin ZZ");
    let mut a = paper("openalex:W3", 2021, "Transformer models for tuberculosis screening", "Rahman M");
    a.cites = vec!["openalex:W1".into(), "openalex:W999".into()];
    let mut b = paper("wos:4", 2022, "Cough audio screening", "Okafor C");
    b.refs = vec!["LAKHANI P, 2017, RADIOLOGY, V284, DOI 10.1148/RADIOL.2017162326".into(), "QIN ZZ, 2019, SCI REP, V9, P15000".into(),
        "WANG X, 2020, IEEE ACCESS, V8".into()];
    let mut c = paper("openalex:W5", 2023, "Explainable models", "Chen L");
    c.cites = vec!["openalex:W3".into(), "openalex:W5".into()];
    insert_many(&mut conn, &[root, qin, a, b, c], "Test").unwrap();

    let h = compute(&conn, &HistParams { nodes: 20 }).unwrap();
    assert_eq!((h.by_id, h.by_doi, h.by_name), (2, 1, 1), "self-citations and works outside the collection are skipped");
    assert_eq!(h.links, 4);
    let lak = h.nodes.iter().find(|n| n.label == "Lakhani 2017").expect("root node");
    assert_eq!(lak.lcs, 2);
    assert_eq!(h.nodes.len(), 3, "only cited documents become nodes");
    assert!(h.nodes.windows(2).all(|w| w[0].year <= w[1].year));
    assert_eq!(h.edges.len(), 1, "Rahman 2021 cites Lakhani 2017; both are nodes");
    assert!(compute(&conn, &HistParams { nodes: 5 }).is_err());
}
