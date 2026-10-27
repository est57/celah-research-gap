//! Historiograph on real records (OpenAlex ids plus PubMed reference text), in a throwaway database.
//! Run with: cargo test live_historiograph -- --ignored --nocapture

use crate::analysis::historiograph::{compute, HistParams};
use crate::db::{insert::insert_many, open};
use crate::search::SearchQuery;
use crate::{http, openalex, pubmed};

#[tokio::test]
#[ignore]
async fn live_historiograph() {
    let path = std::env::temp_dir().join(format!("celah-hist-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let q = SearchQuery { text: "tuberculosis deep learning chest x-ray".into(), year_from: 2015, year_to: 2025, doc_type: None, max_results: 600 };
    insert_many(&mut conn, &openalex::search(&client, &q, None).await.unwrap(), "OpenAlex").unwrap();
    match pubmed::search(&client, &SearchQuery { max_results: 200, ..q }, None).await {
        Ok(items) => {
            let with_refs = items.iter().filter(|p| !p.refs.is_empty()).count();
            println!("pubmed {with_refs} with refs, {:?}", insert_many(&mut conn, &items, "PubMed").unwrap());
            if let Some(p) = items.iter().find(|p| p.refs.len() > 3) {
                println!("  sample refs: {:?}", &p.refs[..3]);
            }
        }
        Err(e) => println!("pubmed skipped: {e}"),
    }
    let start = std::time::Instant::now();
    let h = compute(&conn, &HistParams { nodes: 30 }).unwrap();
    println!("{:?}: {} docs, {} links (id {}, doi {}, name {}), {} edges among {} nodes",
        start.elapsed(), h.documents, h.links, h.by_id, h.by_doi, h.by_name, h.edges.len(), h.nodes.len());
    for n in h.nodes.iter().take(30) {
        println!("  {} lcs {:>3} gcs {:>5} c{}  {}", n.label, n.lcs, n.gcs, n.cluster, n.title.chars().take(80).collect::<String>());
    }
    drop(conn);
    let _ = std::fs::remove_file(&path);
}

/// An "old" collection without cited works, completed from OpenAlex by id and by DOI.
#[tokio::test]
#[ignore]
async fn live_complete_references() {
    use crate::db::enrich;
    use crate::openalex::refs::{fetch, BATCH};
    let path = std::env::temp_dir().join(format!("celah-enrich-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let q = SearchQuery { text: "tuberculosis deep learning chest x-ray".into(), year_from: 2015, year_to: 2025, doc_type: None, max_results: 300 };
    let mut items = openalex::search(&client, &q, None).await.unwrap();
    for (i, p) in items.iter_mut().enumerate() {
        p.cites.clear();
        p.affiliations.clear();
        // Every third record poses as a Scopus import, so only its DOI can be used.
        if i % 3 == 0 && p.doi.is_some() {
            p.ext_id = format!("scopus:{i}");
        }
    }
    insert_many(&mut conn, &items, "OpenAlex").unwrap();
    println!("pending before: {}", enrich::remaining(&conn).unwrap());
    let todo = enrich::pending(&conn, 2000).unwrap();
    let (mut gained, start) = (0, std::time::Instant::now());
    for (field, list) in [("ids.openalex", &todo.by_id), ("doi", &todo.by_doi)] {
        let entries: Vec<(String, i64)> = list.iter().map(|(k, v)| (k.clone(), *v)).collect();
        for chunk in entries.chunks(BATCH) {
            let batch = if field == "doi" { enrich::Pending { by_doi: chunk.iter().cloned().collect(), ..Default::default() } }
                else { enrich::Pending { by_id: chunk.iter().cloned().collect(), ..Default::default() } };
            let values: Vec<String> = chunk.iter().map(|(k, _)| k.clone()).collect();
            let records = fetch(&client, field, &values, None).await.unwrap();
            gained += enrich::save(&mut conn, &batch, &records).unwrap();
        }
    }
    println!("{:?}: {} by id, {} by doi, {gained} gained, pending after {}", start.elapsed(), todo.by_id.len(), todo.by_doi.len(), enrich::remaining(&conn).unwrap());
    let a = crate::analysis::affiliations::compute(&conn).unwrap();
    println!("affiliations: {} of {} docs, {} institutions", a.with_affiliation, a.documents, a.rows.len());
    for r in a.rows.iter().take(8) {
        println!("  {:>3} {:?} {}", r.documents, r.country, r.label);
    }
    let h = compute(&conn, &HistParams { nodes: 10 }).unwrap();
    println!("historiograph: {} links (id {}), top {:?}", h.links, h.by_id, h.nodes.iter().map(|n| (&n.label, n.lcs)).collect::<Vec<_>>());
    drop(conn);
    let _ = std::fs::remove_file(&path);
}
