//! RPYS on real Crossref records. Run with: cargo test live_rpys -- --ignored --nocapture

use crate::analysis::rpys::{compute, RpysParams};
use crate::db::{insert::insert_many, open};
use crate::search::SearchQuery;
use crate::{crossref, http};

#[tokio::test]
#[ignore]
async fn live_rpys() {
    let path = std::env::temp_dir().join(format!("celah-rpys-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let q = SearchQuery { text: "tuberculosis deep learning chest radiograph".into(), year_from: 2018, year_to: 2025, doc_type: None, max_results: 300 };
    let items = crossref::search(&client, &q).await.unwrap();
    println!("crossref {:?}", insert_many(&mut conn, &items, "Crossref").unwrap());
    let t = std::time::Instant::now();
    let r = compute(&conn, &RpysParams { from: 0, to: 0 }).unwrap();
    println!("{:?}: citing {} dated {} undated {} range {}-{}", t.elapsed(), r.citing, r.dated, r.undated, r.from, r.to);
    for p in &r.peaks {
        let refs: Vec<String> = p.refs.iter().take(3).map(|x| format!("{} ({})", x.label, x.documents)).collect();
        println!("  {} cit {} dev {}: {}", p.year, p.citations, p.deviation, refs.join("; "));
    }
    let e = crate::analysis::cited_entities::compute(&conn).unwrap();
    println!("cited sources: {}", e.sources.iter().take(8).map(|c| format!("{} ({})", c.label, c.documents)).collect::<Vec<_>>().join("; "));
    println!("cited authors: {}", e.authors.iter().take(8).map(|c| format!("{} ({})", c.label, c.documents)).collect::<Vec<_>>().join("; "));
    let t = crate::analysis::top_refs::compute(&conn, &crate::analysis::top_refs::TopRefParams { top: 100 }).unwrap();
    println!("top refs: works {} missing {} of {}", t.works, t.missing, t.refs.len());
    for r in t.refs.iter().take(8) {
        println!("  {} ({}) in={:?} {}", r.label, r.documents, r.in_collection.is_some(), r.example.chars().take(70).collect::<String>());
    }
    drop(conn);
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
#[ignore]
async fn live_lookup_by_doi() {
    let client = http::client().unwrap();
    let dois = vec!["10.1038/nature14539".to_string(), "10.1148/radiol.2017162326".to_string(), "10.9999/does-not-exist-celah".to_string()];
    let found = crate::openalex::lookup::by_doi(&client, &dois, None).await.unwrap();
    for p in &found {
        println!("found {:?} {} ({:?}) refs {} cites {}", p.doi, p.title, p.year, p.refs.len(), p.cites.len());
    }
    assert_eq!(found.len(), 2);

    // The added work is then marked as in the collection.
    let mut conn = crate::db::open_memory();
    let citing = crate::db::papers::NewPaper {
        ext_id: "x".into(),
        title: "Citing paper".into(),
        year: Some(2024),
        refs: vec!["LECUN Y, 2015, NATURE, V521, P436, DOI 10.1038/nature14539".into()],
        ..Default::default()
    };
    insert_many(&mut conn, &[citing], "Test").unwrap();
    let before = crate::analysis::top_refs::compute(&conn, &crate::analysis::top_refs::TopRefParams { top: 20 }).unwrap();
    insert_many(&mut conn, &found, "OpenAlex (rujukan)").unwrap();
    let after = crate::analysis::top_refs::compute(&conn, &crate::analysis::top_refs::TopRefParams { top: 20 }).unwrap();
    println!("before in={:?} after in={:?} title {:?}", before.refs[0].in_collection.is_some(), after.refs[0].in_collection.is_some(), after.refs[0].title);
    assert!(before.refs[0].in_collection.is_none() && after.refs[0].in_collection.is_some());
}
