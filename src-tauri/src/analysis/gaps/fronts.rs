//! Isolated research fronts: coupling clusters that are cited well above their
//! publication years' average but share few references with the other fronts.

use super::model::{part, slug, Draft, EvidenceRef, Kind, Signal};
use super::Ctx;
use crate::analysis::coupling_map::{self, CouplingCluster, CouplingMapParams};
use crate::error::AppResult;
use rusqlite::{params, Connection, OptionalExtension};

const DOCUMENTS: u32 = 200;
const MIN_SIZE: usize = 3;
const MIN_IMPACT: f64 = 1.0;

fn years(conn: &Connection, ids: &[i64]) -> AppResult<Vec<i32>> {
    let mut stmt = conn.prepare("SELECT year FROM papers WHERE id = ?1")?;
    let mut out = Vec::new();
    for id in ids {
        if let Some(Some(y)) = stmt.query_row(params![id], |r| r.get::<_, Option<i32>>(0)).optional()? {
            out.push(y);
        }
    }
    Ok(out)
}

pub fn detect(conn: &Connection, ctx: &Ctx) -> AppResult<Vec<Draft>> {
    let map = coupling_map::compute(conn, &CouplingMapParams { documents: DOCUMENTS })?;
    // The front every other cluster is compared with: the most central one.
    let Some(main) = map.clusters.iter().filter(|c| !c.keywords.is_empty()).max_by(|a, b| a.centrality.total_cmp(&b.centrality)) else {
        return Ok(Vec::new());
    };
    if map.clusters.len() < 3 {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for c in map.clusters.iter().filter(|c| c.quadrant == "niche" && c.size >= MIN_SIZE && c.impact >= MIN_IMPACT && c.id != main.id) {
        let Some(a) = c.keywords.first() else { continue };
        let b = &main.keywords[0];
        out.push(draft(c, main, a, b, ctx.momentum_of_years(&years(conn, &c.members)?)));
    }
    Ok(out)
}

fn draft(c: &CouplingCluster, main: &CouplingCluster, a: &str, b: &str, momentum: f64) -> Draft {
    let mut evidence: Vec<EvidenceRef> = c.documents.iter().take(3).map(|d| EvidenceRef { paper_id: d.id, role: "mendukung", quote: None }).collect();
    evidence.extend(main.documents.iter().take(2).map(|d| EvidenceRef { paper_id: d.id, role: "konteks", quote: None }));
    Draft {
        key: format!("front:{}:{}", slug(a), slug(b)),
        kind: Kind::Front,
        title: format!("Front riset {a} berdampak tinggi tetapi terpisah dari front utama"),
        statement: format!(
            "{} dokumen tentang ({}) saling merujuk karya yang sama dan disitasi {:.1} kali rata-rata tahun terbitnya, tetapi jarang berbagi rujukan dengan front utama ({}). Pendekatan dari front ini belum banyak dibawa ke arus utama.",
            c.size, c.keywords.join(", "), c.impact, main.keywords.join(", ")
        ),
        terms: vec![a.to_string(), b.to_string()],
        parts: vec![part("Kebaruan", 1.0 - c.x), part("Bukti", c.size as f64 / 10.0), part("Momentum", momentum)],
        signals: vec![Signal {
            analysis: "Peta kopling",
            finding: format!("kuadran niche berdampak, dampak {:.2}, sentralitas {:.1}", c.impact, c.centrality),
            screen: "intellectual",
            tab: Some("coupling"),
        }],
        evidence,
        questions: vec![
            format!("Apakah pendekatan dari front {a} dapat menjawab masalah di front {b}?"),
            format!("Mengapa studi {a} jarang merujuk literatur yang sama dengan studi {b}?"),
        ],
    }
}
