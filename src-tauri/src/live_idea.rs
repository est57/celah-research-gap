//! Idea tests on real records. Run with: cargo test live_idea -- --ignored --nocapture

use crate::analysis::idea::{compute, IdeaParams};
use crate::db::{insert::insert_many, open};
use crate::search::SearchQuery;
use crate::{europepmc, http, openalex};

#[tokio::test]
#[ignore]
async fn live_idea() {
    let path = std::env::temp_dir().join(format!("celah-idea-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let q = SearchQuery { text: "tuberculosis deep learning".into(), year_from: 2015, year_to: 2025, doc_type: None, max_results: 600 };
    match openalex::search(&client, &q, None).await {
        Ok(items) => println!("openalex {:?}", insert_many(&mut conn, &items, "OpenAlex").unwrap()),
        Err(e) => println!("openalex skipped: {e}"),
    }
    let epmc = europepmc::search(&client, &SearchQuery { max_results: 800, ..q }).await.unwrap();
    println!("europe pmc {:?}", insert_many(&mut conn, &epmc, "Europe PMC").unwrap());
    let ideas = [("vision transformer, ViT", "sputum smear"), ("transfer learning", "chest x-ray, chest radiograph"), ("cough sound, cough audio", "explainable, Grad-CAM")];
    for (a, b) in ideas {
        let r = compute(&conn, &IdeaParams { a: a.into(), b: b.into() }).unwrap();
        println!("{a} x {b}: A {} B {} both {} expected {:.1} -> {} | recent {:.2}/{:.2}/{:.2}", r.with_a, r.with_b, r.with_both, r.expected, r.verdict, r.recent_a, r.recent_b, r.recent_all);
        for d in r.both_docs.iter().take(2) {
            println!("    both: {}", d.title.chars().take(90).collect::<String>());
        }
    }
    drop(conn);
    let _ = std::fs::remove_file(&path);
}
