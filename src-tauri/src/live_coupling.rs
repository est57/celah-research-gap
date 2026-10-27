//! Bibliographic coupling on real Crossref records. Run with: cargo test live_coupling -- --ignored --nocapture

use crate::analysis::coupling::{compute, Level};
use crate::db::{insert::insert_many, open};
use crate::search::SearchQuery;
use crate::{crossref, http};

#[tokio::test]
#[ignore]
async fn live_coupling() {
    let path = std::env::temp_dir().join(format!("celah-coupling-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let client = http::client().unwrap();
    let q = SearchQuery { text: "tuberculosis deep learning chest radiograph".into(), year_from: 2018, year_to: 2025, doc_type: None, max_results: 300 };
    println!("crossref {:?}", insert_many(&mut conn, &crossref::search(&client, &q).await.unwrap(), "Crossref").unwrap());
    for level in [Level::Documents, Level::Sources, Level::Authors] {
        let t = std::time::Instant::now();
        let (coupled, net) = compute(&conn, level, 50).unwrap();
        println!("{level:?} {:?}: coupled {coupled}, nodes {}, edges {}, clusters {}", t.elapsed(), net.nodes.len(), net.edges.len(), net.clusters.len());
        for c in net.clusters.iter().take(4) {
            println!("  cluster {} ({}): {}", c.id, c.size, c.top.join("; "));
        }
    }
    let t = std::time::Instant::now();
    let m = crate::analysis::coupling_map::compute(&conn, &crate::analysis::coupling_map::CouplingMapParams { documents: 200 }).unwrap();
    println!("map {:?}: coupled {} used {} clusters {}", t.elapsed(), m.coupled, m.used, m.clusters.len());
    for c in &m.clusters {
        println!("  {} n={} {} impact {:.2} cen {:.1}: {}", c.quadrant, c.size, c.keywords.join(", "), c.impact, c.centrality, c.documents.first().map(|d| d.title.chars().take(60).collect::<String>()).unwrap_or_default());
    }
    let t = std::time::Instant::now();
    let list = crate::analysis::gaps::compute(&conn).unwrap();
    println!("gaps {:?}: {} total", t.elapsed(), list.gaps.len());
    for g in list.gaps.iter().filter(|g| g.kind == crate::analysis::gaps::model::Kind::Front) {
        println!("  FRONT {} [{}] {}", g.score, g.confidence, g.title);
        println!("    {}", g.statement);
    }
    let p = crate::report::summary::SummaryParams { style: "apa".into(), language: "id".into() };
    let s = crate::report::summary::build(&conn, &p).unwrap();
    for sec in &s.paragraphs {
        println!("## {}\n{}\n", sec.heading, sec.text);
    }
    println!("refs {}", s.references.len());
    drop(conn);
    let _ = std::fs::remove_file(&path);
}
