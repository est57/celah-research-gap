//! Author name suggestions on real records from two sources with different name styles.
//! Run with: cargo test live_author_names -- --ignored --nocapture

use crate::db::{author_suggest, insert::insert_many, open};
use crate::search::SearchQuery;
use crate::{europepmc, http, openalex};

#[tokio::test]
#[ignore]
async fn live_author_names() {
    let path = std::env::temp_dir().join(format!("celah-names-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let q = SearchQuery { text: "tuberculosis deep learning chest x-ray".into(), year_from: 2015, year_to: 2025, doc_type: None, max_results: 400 };
    insert_many(&mut conn, &openalex::search(&client, &q, None).await.unwrap(), "OpenAlex").unwrap();
    match europepmc::search(&client, &SearchQuery { max_results: 200, ..q }).await {
        Ok(items) => println!("europe pmc {:?}", insert_many(&mut conn, &items, "Europe PMC").unwrap()),
        Err(e) => println!("europe pmc skipped: {e}"),
    }
    let s = author_suggest::suggest(&conn).unwrap();
    println!("{} suggestions", s.len());
    for g in s.iter().take(15) {
        let names: Vec<String> = g.variants.iter().map(|v| format!("{} ({})", v.name, v.documents)).collect();
        println!("  co {} src {} | {}", g.shared_coauthors, g.shared_sources, names.join(" / "));
    }
    drop(conn);
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
#[ignore]
async fn live_synonyms() {
    use crate::db::{analysis_conn::AnalysisDb, synonyms};
    let path = std::env::temp_dir().join(format!("celah-syn-live-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let q = SearchQuery { text: "tuberculosis deep learning".into(), year_from: 2015, year_to: 2025, doc_type: None, max_results: 600 };
    insert_many(&mut conn, &openalex::search(&client, &q, None).await.unwrap(), "OpenAlex").unwrap();
    match europepmc::search(&client, &SearchQuery { max_results: 500, ..q }).await {
        Ok(items) => println!("europe pmc {:?}", insert_many(&mut conn, &items, "Europe PMC").unwrap()),
        Err(e) => println!("europe pmc skipped: {e}"),
    }
    let s = synonyms::suggest::suggest(&conn).unwrap();
    println!("{} suggestions", s.len());
    for g in s.iter().take(20) {
        let t: Vec<String> = g.terms.iter().map(|t| format!("{} ({})", t.label, t.documents)).collect();
        println!("  {:<9} {}", g.kind, t.join(" / "));
    }
    let mut db = AnalysisDb::open(&path).unwrap();
    let count = |db: &mut AnalysisDb| {
        let g = crate::analysis::gaps::compute(db.prepare().unwrap()).unwrap().gaps;
        (g.len(), g.iter().filter(|x| x.key.starts_with("matrix:")).count())
    };
    println!("gaps before (all, matrix): {:?}", count(&mut db));
    for g in s.iter().filter(|g| g.kind == "ejaan") {
        let labels: Vec<String> = g.terms.iter().map(|t| t.label.clone()).collect();
        synonyms::merge(&mut conn, &labels, &labels[0]).unwrap();
    }
    println!("gaps after merging spelling variants: {:?}", count(&mut db));
    drop((db, conn));
    let _ = std::fs::remove_file(&path);
}
