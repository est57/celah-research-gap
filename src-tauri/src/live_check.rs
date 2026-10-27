//! End-to-end check against the real APIs: search several sources, store the
//! results in a throwaway database and print the country analysis.
//! Run with: cargo test live_check -- --ignored --nocapture

use crate::analysis::{countries, overview};
use crate::db::{insert::insert_many, open};
use crate::search::SearchQuery;
use crate::{crossref, europepmc, http, openalex, pubmed};

fn query(max: u32) -> SearchQuery {
    SearchQuery { text: "tuberculosis deep learning".into(), year_from: 2019, year_to: 2025, doc_type: None, max_results: max }
}

#[tokio::test]
#[ignore]
async fn countries_from_real_sources() {
    let path = std::env::temp_dir().join(format!("celah-live-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();

    let batches = [
        ("OpenAlex", openalex::search(&client, &query(200), None).await),
        ("PubMed", pubmed::search(&client, &query(100), None).await),
        ("Europe PMC", europepmc::search(&client, &query(100)).await),
        ("Crossref", crossref::search(&client, &query(100)).await),
    ];
    for (label, result) in batches {
        let items = result.unwrap_or_else(|e| panic!("{label}: {e}"));
        let with_country = items.iter().filter(|p| !p.countries.is_empty()).count();
        let (inserted, dup) = insert_many(&mut conn, &items, label).unwrap();
        println!("{label:<11} {:>4} fetched, {with_country:>4} with country, {inserted:>4} new, {dup:>4} duplicate", items.len());
    }

    let c = countries::compute(&conn).unwrap();
    let info = overview(&conn).unwrap().info;
    println!("\ndocuments: {}, with country: {}", info.documents, c.documents_with_country);
    println!("international co-authorship: {:?}", info.international_coauthorship.map(|s| (s * 1000.0).round() / 10.0));
    println!("\ncorresponding country   articles  SCP  MCP");
    for r in c.corresponding.iter().take(10) {
        println!("{:<22} {:>9} {:>4} {:>4}", r.code, r.articles, r.scp, r.mcp);
    }
    println!("\ntop pairs:");
    for p in c.pairs.iter().take(8) {
        println!("  {}-{} {}", p.a, p.b, p.count);
    }
    println!("\nnetwork: {} nodes, {} clusters", c.network.nodes.len(), c.network.clusters.len());

    let codes: Vec<String> = conn
        .prepare("SELECT DISTINCT code FROM paper_countries ORDER BY code")
        .unwrap()
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    println!("\nall codes: {}", codes.join(" "));
    drop(conn);
    let _ = std::fs::remove_file(&path);

    assert!(c.documents_with_country > 0);
    assert!(codes.iter().all(|c| c.len() == 2 && c.chars().all(|ch| ch.is_ascii_uppercase())));
}

/// Prints the gap candidates found in real data from OpenAlex and PubMed.
/// Run with: cargo test live_gaps -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn live_gaps() {
    let path = std::env::temp_dir().join(format!("celah-gaps-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    for (label, items) in [
        ("OpenAlex", openalex::search(&client, &query(400), None).await.unwrap()),
        ("PubMed", pubmed::search(&client, &query(200), None).await.unwrap()),
    ] {
        insert_many(&mut conn, &items, label).unwrap();
    }
    let list = crate::analysis::gaps::compute(&conn).unwrap();
    println!("{} documents, field: {}, {} gaps", list.documents, list.field, list.gaps.len());
    for g in list.gaps.iter().take(12) {
        println!("\n[{:?}] {:.2} {} | {}", g.kind, g.score, g.confidence, g.title);
        println!("  {}", g.statement.chars().take(220).collect::<String>());
        for e in g.evidence.iter().take(2) {
            println!("  - ({}) {} {}", e.role, e.year.unwrap_or(0), e.title.chars().take(90).collect::<String>());
            if let Some(q) = &e.quote {
                println!("    \"{}\"", q.chars().take(160).collect::<String>());
            }
        }
    }
    drop(conn);
    let _ = std::fs::remove_file(&path);
    assert!(list.gaps.iter().all(|g| !g.evidence.is_empty()));
}

/// Accepts the top gaps of a real collection and verifies them against recent OpenAlex papers.
/// Run with: cargo test live_verify -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn live_verify() {
    let path = std::env::temp_dir().join(format!("celah-verify-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let items = openalex::search(&client, &SearchQuery { year_to: 2022, ..query(400) }, None).await.unwrap();
    insert_many(&mut conn, &items, "OpenAlex").unwrap();
    let list = crate::analysis::gaps::compute(&conn).unwrap();
    for g in list.gaps.iter().take(2) {
        let q = crate::verify::query_text(&g.terms);
        let recent = SearchQuery { text: q.clone(), year_from: 2024, year_to: 2026, doc_type: None, max_results: 50 };
        let found = openalex::search(&client, &recent, None).await.unwrap();
        let papers = crate::verify::candidates(&conn, &found, &g.terms).unwrap();
        crate::db::checks::save(&mut conn, &g.key, &q, &papers).unwrap();
        println!("\n{} | terms {:?}\n  before score {:.2}", g.title, g.terms, g.score);
    }
    let after = crate::analysis::gaps::compute(&conn).unwrap();
    for g in after.gaps.iter().filter(|g| g.check.is_some()) {
        let c = g.check.as_ref().unwrap();
        println!("\n{} -> verdict {}, {} new papers, {} match all terms, score now {:.2}", g.key, c.verdict, c.found, c.matching, g.score);
        for p in c.papers.iter().take(3) {
            println!("  [{}] {} {}", if p.matches { "cocok" } else { "-" }, p.year.unwrap_or(0), p.title.chars().take(100).collect::<String>());
        }
    }
    drop(conn);
    let _ = std::fs::remove_file(&path);
}

/// Builds a report from the top three gaps of a real collection and prints it.
/// Run with: cargo test live_report -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn live_report() {
    let path = std::env::temp_dir().join(format!("celah-report-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    insert_many(&mut conn, &openalex::search(&client, &query(400), None).await.unwrap(), "OpenAlex").unwrap();
    for g in crate::analysis::gaps::compute(&conn).unwrap().gaps.iter().take(3) {
        crate::db::reviews::set(&conn, &g.key, "accepted", "", &g.title).unwrap();
    }
    for (style, lang) in [("apa", "id"), ("ieee", "en")] {
        let p = crate::report::ReportParams { style: style.into(), language: lang.into(), include_verification: true };
        let r = crate::report::build(&conn, &p).unwrap();
        println!("\n===== {style} / {lang} =====\n{}", r.markdown.chars().take(2600).collect::<String>());
    }
    drop(conn);
    let _ = std::fs::remove_file(&path);
}
