//! Structural gaps: keyword clusters that are linked far less than their size predicts.

use super::model::{part, slug, Draft, EvidenceRef, Kind, Signal};
use super::Ctx;
use crate::tr;
use crate::analysis::network;
use std::collections::HashMap;

const WORDS: usize = 80;
const MAX_RATIO: f64 = 0.5;

pub fn detect(ctx: &Ctx) -> Vec<Draft> {
    let net = &ctx.net;
    let n = net.labels.len().min(WORDS);
    let mut docs: Vec<Vec<String>> = vec![Vec::new(); net.years.len()];
    for w in 0..n {
        for &d in &net.docs[w] {
            docs[d].push(net.labels[w].clone());
        }
    }
    let labels: HashMap<String, String> = net.labels[..n].iter().map(|l| (l.clone(), l.clone())).collect();
    let g = network::build(&docs, &labels, n);
    let word_index: HashMap<&str, usize> = net.labels[..n].iter().enumerate().map(|(i, l)| (l.as_str(), i)).collect();
    let members = |c: usize| -> Vec<usize> {
        g.nodes.iter().filter(|x| x.cluster == c).filter_map(|x| word_index.get(x.label.as_str()).copied()).collect()
    };
    let docs_of = |words: &[usize]| -> Vec<usize> {
        let mut d: Vec<usize> = words.iter().flat_map(|&w| net.docs[w].iter().copied()).collect();
        d.sort_unstable();
        d.dedup();
        d
    };

    g.bridges
        .iter()
        .filter(|b| b.ratio < MAX_RATIO)
        .filter_map(|b| {
            let (wa, wb) = (members(b.a), members(b.b));
            let (name_a, name_b) = (&net.labels[*wa.first()?], &net.labels[*wb.first()?]);
            let (da, db) = (docs_of(&wa), docs_of(&wb));
            let both: Vec<usize> = da.iter().filter(|d| db.binary_search(d).is_ok()).copied().collect();
            let union: Vec<usize> = da.iter().chain(&db).copied().collect();
            let parts = vec![part(tr!("Kebaruan", "Novelty"), 1.0 - b.ratio), part(tr!("Bukti", "Evidence"), (wa.len() + wb.len()) as f64 / 12.0), part("Momentum", ctx.momentum(&union))];
            let mut evidence: Vec<EvidenceRef> = ctx.top(&both, 2).into_iter().map(|id| EvidenceRef { paper_id: id, role: "hampir menjawab", quote: None }).collect();
            for id in ctx.top(&da, 2).into_iter().chain(ctx.top(&db, 2)) {
                if !evidence.iter().any(|e| e.paper_id == id) {
                    evidence.push(EvidenceRef { paper_id: id, role: "konteks", quote: None });
                }
            }
            let words = |ws: &[usize]| ws.iter().take(4).map(|&w| net.labels[w].as_str()).collect::<Vec<_>>().join(", ");
            Some(Draft {
                key: format!("structural:{}:{}", slug(name_a), slug(name_b)),
                kind: Kind::Structural,
                terms: vec![name_a.clone(), name_b.clone()],
                title: tr!(
                    format!("Kelompok topik {name_a} dan {name_b} belum saling terhubung"),
                    format!("Topic clusters {name_a} and {name_b} are not yet connected")
                ),
                statement: tr!(
                    format!(
                        "Kelompok kata kunci ({}) dan ({}) hanya terhubung lewat {} kemunculan bersama, sekitar {:.0}% dari yang diharapkan menurut ukuran keduanya. Dua bidang ini tampaknya berkembang terpisah.",
                        words(&wa), words(&wb), b.links, b.ratio * 100.0
                    ),
                    format!(
                        "Keyword clusters ({}) and ({}) are linked by only {} co-occurrences, about {:.0}% of what their sizes predict. The two areas seem to develop separately.",
                        words(&wa), words(&wb), b.links, b.ratio * 100.0
                    )
                ),
                signals: vec![Signal {
                    analysis: tr!("Jaringan kata kunci", "Keyword network"),
                    finding: tr!(
                        format!("{} tautan antar-kelompok, rasio {:.2} terhadap perkiraan", b.links, b.ratio),
                        format!("{} links between clusters, ratio {:.2} to expected", b.links, b.ratio)
                    ),
                    screen: "conceptual",
                    tab: None,
                }],
                questions: vec![
                    tr!(
                        format!("Apakah metode atau temuan dari kelompok {name_a} dapat menjawab masalah di kelompok {name_b}?"),
                        format!("Can methods or findings from the {name_a} cluster address problems in the {name_b} cluster?")
                    ),
                    tr!(
                        format!("Apa hambatan yang membuat studi {name_a} dan {name_b} jarang bertemu?"),
                        format!("What keeps {name_a} and {name_b} studies from meeting?")
                    ),
                ],
                parts,
                evidence: evidence.into_iter().take(6).collect(),
            })
        })
        .collect()
}
