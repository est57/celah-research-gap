//! Duplicate detection on real records from several sources, in a throwaway database.
//! Run with: cargo test live_duplicates -- --ignored --nocapture

use crate::db::{duplicates, insert::insert_many, open};
use crate::search::SearchQuery;
use crate::{arxiv, crossref, europepmc, http, openalex, semantic_scholar};

#[tokio::test]
#[ignore]
async fn live_duplicates() {
    let path = std::env::temp_dir().join(format!("celah-dup-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let q = SearchQuery { text: "transformer chest x-ray tuberculosis".into(), year_from: 2020, year_to: 2025, doc_type: None, max_results: 200 };
    let batches = [
        ("OpenAlex", openalex::search(&client, &q, None).await),
        ("arXiv", arxiv::search(&client, &q).await),
        ("Semantic Scholar", semantic_scholar::search(&client, &q, None).await),
        ("Europe PMC", europepmc::search(&client, &q).await),
        ("Crossref", crossref::search(&client, &q).await),
    ];
    for (label, result) in batches {
        match result {
            Ok(items) => {
                let (new, dup) = insert_many(&mut conn, &items, label).unwrap();
                println!("{label:<17} {new:>4} new, {dup:>4} exact duplicates");
            }
            Err(e) => println!("{label:<17} skipped: {e}"),
        }
    }
    let list = duplicates::list(&conn).unwrap();
    println!("\n{} documents, {} candidate pairs\n", list.documents, list.pairs.len());
    for p in list.pairs.iter().take(25) {
        let keep = if p.keep == p.a.id { "A" } else { "B" };
        println!("[{} {:.2} keep {keep}]", p.reason, p.similarity);
        for (tag, d) in [("A", &p.a), ("B", &p.b)] {
            println!("  {tag} {} | {:?} | {} | {:?} | {}", d.origin, d.year, d.doc_type.as_deref().unwrap_or("-"), d.doi, d.title);
        }
    }
    drop(conn);
    let _ = std::fs::remove_file(&path);
}
