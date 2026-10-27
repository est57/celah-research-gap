//! Timing of every analysis on a synthetic 10,000-document collection.
//! Run with: cargo test --release live_scale -- --ignored --nocapture

use crate::analysis::{self, *};
use crate::db::{insert::insert_many, open, papers::NewPaper};
use std::time::Instant;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn pick(&mut self, n: usize) -> usize { (self.next() * n as f64) as usize % n }
    /// Skewed towards small indices, like citation counts.
    fn skew(&mut self, n: usize) -> usize { (self.next().powi(3) * n as f64) as usize % n }
}

const COUNTRIES: [&str; 12] = ["ID", "IN", "CN", "US", "GB", "KR", "BR", "ZA", "NG", "DE", "SA", "MY"];

fn collection(n: usize) -> Vec<NewPaper> {
    let mut r = Rng(42);
    let vocab: Vec<String> = (0..400).map(|i| format!("topic{} method{}", i % 40, i)).collect();
    (0..n)
        .map(|i| {
            let cluster = r.pick(8);
            let kw: Vec<String> = (0..5).map(|_| vocab[cluster * 50 + r.skew(50)].clone()).collect();
            let words: Vec<String> = (0..120).map(|_| vocab[cluster * 50 + r.skew(50)].replace(' ', " the ")).collect();
            let refs: Vec<String> = (0..30).map(|_| { let k = r.skew(20_000); format!("AUTHOR{k} X, {}, JOURNAL{} {}, V{}, P{}", 1980 + k % 44, k % 300, k, k % 50, k) }).collect();
            let countries: Vec<String> = (0..1 + r.pick(3)).map(|_| COUNTRIES[r.pick(12)].to_string()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
            NewPaper {
                ext_id: format!("s{i}"),
                title: format!("Study {i} on {} and {}", kw[0], kw[1]),
                year: Some(2010 + r.pick(16) as i32),
                cited_by: r.skew(500) as i64,
                author_names: (0..4).map(|_| format!("Author{} A", r.skew(3000))).collect(),
                keywords: Some(kw.join("; ")),
                abstract_text: Some(words.join(" ")),
                source_name: Some(format!("Journal {}", r.skew(300))),
                doc_type: Some("article".into()),
                corresponding_country: countries.first().cloned(),
                countries,
                refs,
                ..Default::default()
            }
        })
        .collect()
}

fn time<T>(label: &str, f: impl FnOnce() -> T) -> T {
    let t = Instant::now();
    let out = f();
    println!("{label:<28} {:>8.0} ms", t.elapsed().as_secs_f64() * 1000.0);
    out
}

#[test]
#[ignore]
fn live_scale() {
    let path = std::env::temp_dir().join(format!("celah-scale-{}.sqlite", std::process::id()));
    let mut conn = open(&path).unwrap();
    let items = time("generate", || collection(10_000));
    time("insert", || insert_many(&mut conn, &items, "Scale").unwrap());
    let c = &conn;
    time("overview", || analysis::overview(c).unwrap());
    time("sources_authors", || sources_authors::compute(c).unwrap());
    time("author_table", || author_table::compute(c).unwrap());
    time("country_table", || country_table::compute(c).unwrap());
    time("cited", || cited::compute(c, &cited::CitedParams { sort: "global".into(), limit: 50 }).unwrap());
    time("conceptual", || conceptual::compute(c, &conceptual::ConceptualParams { field: "auto".into(), words: 250, min_cluster_freq: 5, labels: 3, trend_min_freq: 5 }).unwrap());
    time("evolution", || evolution::compute(c, &evolution::EvolutionParams { field: "auto".into(), words: 250, min_cluster_freq: 5, labels: 2, cuts: vec![] }).unwrap());
    time("factorial", || factorial::compute(c, &factorial::FactorialParams { field: "auto".into(), words: 50, clusters: 0 }).unwrap());
    time("words title", || words::compute(c, &words::WordParams { field: "title".into(), ngram: 2, top: 50 }).unwrap());
    time("words abstract", || words::compute(c, &words::WordParams { field: "abstract".into(), ngram: 2, top: 50 }).unwrap());
    for unit in ["references", "authors", "keywords", "coupling_documents"] {
        time(&format!("structure {unit}"), || structure::compute(c, &structure::StructureParams { unit: unit.into(), nodes: 50 }).unwrap());
    }
    time("historiograph", || historiograph::compute(c, &historiograph::HistParams { nodes: 30 }).unwrap());
    time("rpys", || rpys::compute(c, &rpys::RpysParams { from: 0, to: 0 }).unwrap());
    time("top_refs", || top_refs::compute(c, &top_refs::TopRefParams { top: 100 }).unwrap());
    time("cited_entities", || cited_entities::compute(c).unwrap());
    time("coupling_map", || coupling_map::compute(c, &coupling_map::CouplingMapParams { documents: 200 }).unwrap());
    time("gaps", || gaps::compute(c).unwrap());
    time("duplicates", || crate::db::duplicates::list(c).unwrap());
    time("synonym suggestions", || crate::db::synonyms::suggest::suggest(c).unwrap());
    time("author suggestions", || crate::db::author_suggest::suggest(c).unwrap());
    time("report summary", || crate::report::summary::build(c, &crate::report::summary::SummaryParams { style: "apa".into(), language: "id".into() }).unwrap());
    drop(conn);
    let _ = std::fs::remove_file(&path);
}
