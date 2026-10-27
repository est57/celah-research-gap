//! Word analysis on real records. Run with: cargo test live_words -- --ignored --nocapture

use crate::analysis::words::{compute, WordParams};
use crate::db::{insert::insert_many, open};
use crate::search::SearchQuery;
use crate::{europepmc, http};

#[tokio::test]
#[ignore]
async fn live_words() {
    let path = std::env::temp_dir().join(format!("celah-words-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let q = SearchQuery { text: "tuberculosis deep learning".into(), year_from: 2015, year_to: 2025, doc_type: None, max_results: 500 };
    let items = europepmc::search(&client, &q).await.unwrap();
    println!("europe pmc {:?}", insert_many(&mut conn, &items, "Europe PMC").unwrap());
    for (field, n) in [("author_keywords", 1), ("title", 1), ("title", 2), ("abstract", 1), ("abstract", 2), ("abstract", 3)] {
        let t = std::time::Instant::now();
        let w = compute(&conn, &WordParams { field: field.into(), ngram: n, top: 15 }).unwrap();
        let top: Vec<String> = w.words.iter().map(|x| format!("{} {}", x.label, x.documents)).collect();
        println!("{field}/{n} ({} of {} docs, {} distinct, {:?}): {}", w.with_field, w.documents, w.distinct, t.elapsed(), top.join(" | "));
    }
    drop(conn);
    let _ = std::fs::remove_file(&path);
}
