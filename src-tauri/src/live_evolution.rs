//! Thematic evolution on real OpenAlex records, in a throwaway database.
//! Run with: cargo test live_evolution -- --ignored --nocapture

use crate::analysis::evolution::{compute, EvolutionParams};
use crate::db::{insert::insert_many, open};
use crate::search::SearchQuery;
use crate::{http, openalex};

#[tokio::test]
#[ignore]
async fn live_evolution() {
    let path = std::env::temp_dir().join(format!("celah-evo-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let q = SearchQuery { text: "tuberculosis deep learning".into(), year_from: 2012, year_to: 2025, doc_type: None, max_results: 600 };
    insert_many(&mut conn, &openalex::search(&client, &q, None).await.unwrap(), "OpenAlex").unwrap();
    let e = compute(&conn, &EvolutionParams { field: "auto".into(), words: 250, min_cluster_freq: 5, labels: 2, cuts: Vec::new() }).unwrap();
    println!("field {} cuts {:?}", e.field, e.cuts);
    for (i, p) in e.periods.iter().enumerate() {
        println!("\nperiod {i}: {}-{} ({} docs)", p.from, p.to, p.documents);
        for t in p.themes.iter().take(8) {
            println!("  [{}] {} (freq {}, {})", t.id, t.name, t.frequency, t.quadrant);
        }
    }
    println!("\nlinks:");
    for l in &e.links {
        println!("  p{} {} -> {}  inc {:.2}  {:?}", l.period, l.from, l.to, l.inclusion, l.shared);
    }
    let last = e.periods.len() - 1;
    let fresh: Vec<&str> = e.periods[last].themes.iter().filter(|t| !e.links.iter().any(|l| l.period + 1 == last && l.to == t.id)).map(|t| t.name.as_str()).collect();
    println!("\nnew in last period: {fresh:?}");
    drop(conn);
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
#[ignore]
async fn live_factorial() {
    use crate::analysis::factorial::{compute, FactorialParams};
    let path = std::env::temp_dir().join(format!("celah-mca-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let q = SearchQuery { text: "tuberculosis deep learning".into(), year_from: 2012, year_to: 2025, doc_type: None, max_results: 600 };
    insert_many(&mut conn, &openalex::search(&client, &q, None).await.unwrap(), "OpenAlex").unwrap();
    for words in [50, 100] {
        let start = std::time::Instant::now();
        let f = compute(&conn, &FactorialParams { field: "auto".into(), words, clusters: 0 }).unwrap();
        println!("\n{words} words: {:?}, {} docs, explained {:.2} / {:.2}", start.elapsed(), f.documents, f.explained[0], f.explained[1]);
        for c in &f.clusters {
            println!("  cluster {} ({} words) at ({:.2}, {:.2}): {:?}", c.id, c.size, c.x, c.y, c.top);
        }
        for d in f.top_documents.iter().take(4) {
            println!("  doc c{} ({:.2}, {:.2}) {}", d.cluster, d.x, d.y, d.title);
        }
    }
    drop(conn);
    let _ = std::fs::remove_file(&path);
}
