//! Emerging and niche themes from the thematic map.

use super::model::{part, slug, Draft, EvidenceRef, Kind, Signal};
use super::Ctx;
use crate::analysis::thematic::{self, Theme};
use std::collections::HashMap;

const MIN_PER_THOUSAND: u32 = 5;
const LABELS: usize = 3;

fn docs_of(ctx: &Ctx, theme: &Theme, index: &HashMap<&str, usize>) -> Vec<usize> {
    let mut d: Vec<usize> = theme
        .words
        .iter()
        .filter_map(|w| index.get(w.label.as_str()))
        .flat_map(|&i| ctx.net.docs[i].iter().copied())
        .collect();
    d.sort_unstable();
    d.dedup();
    d
}

pub fn detect(ctx: &Ctx) -> Vec<Draft> {
    let map = thematic::compute(&ctx.net, MIN_PER_THOUSAND, LABELS, ctx.current_year);
    let index: HashMap<&str, usize> = ctx.net.labels.iter().enumerate().map(|(i, l)| (l.as_str(), i)).collect();
    let period = format!("{}-{}", map.recent_from, map.recent_to);
    let pct = |v: f64| format!("{:.0}%", v * 100.0);
    // The gap is the missing link to the main themes, so verification looks for
    // papers that combine this theme with the leading motor theme.
    let main = map
        .themes
        .iter()
        .filter(|m| m.quadrant == "motor")
        .max_by_key(|m| m.frequency)
        .or_else(|| map.themes.iter().max_by(|a, b| a.x.total_cmp(&b.x)));
    let main_lead = main.and_then(|m| m.words.first()).map(|w| w.label.clone());
    let mut out = Vec::new();
    for t in &map.themes {
        // A small margin keeps themes that merely match the collection's pace out of the list.
        let emerging = t.quadrant == "emerging" && t.recent_share > map.collection_recent_share + 0.05;
        if !emerging && t.quadrant != "niche" {
            continue;
        }
        let docs = docs_of(ctx, t, &index);
        let lead = t.words.first().map(|w| w.label.clone()).unwrap_or_else(|| t.name.clone());
        // Newest evidence comes from the theme's leading keyword, so it is on topic.
        let lead_docs = t.words.first().and_then(|w| index.get(w.label.as_str())).map(|&i| ctx.net.docs[i].clone()).unwrap_or_default();
        let mut evidence: Vec<EvidenceRef> = ctx.newest(&lead_docs, 2).into_iter().map(|id| EvidenceRef { paper_id: id, role: "mendukung", quote: None }).collect();
        for id in ctx.top(&docs, 3) {
            if !evidence.iter().any(|e| e.paper_id == id) {
                evidence.push(EvidenceRef { paper_id: id, role: "konteks", quote: None });
            }
        }
        let (kind, title, statement) = if emerging {
            (
                Kind::Emerging,
                format!("Tema {lead} baru tumbuh dan belum terhubung dengan tema utama"),
                format!(
                    "Tema {} ({} dokumen) berada di kuadran baru muncul: {} dokumennya terbit {period}, di atas rata-rata koleksi ({}). Sentralitasnya rendah, artinya tema ini masih jarang dikaitkan dengan tema utama.",
                    t.name, t.documents, pct(t.recent_share), pct(map.collection_recent_share)
                ),
            )
        } else {
            (
                Kind::Niche,
                format!("Tema {lead} berkembang sendiri, terpisah dari tema utama"),
                format!(
                    "Tema {} ({} dokumen) adalah tema niche: rapat di dalam kelompoknya (densitas tinggi) tetapi jarang terhubung dengan tema lain (sentralitas rendah). Kaitannya dengan tema motor bisa menjadi celah riset.",
                    t.name, t.documents
                ),
            )
        };
        out.push(Draft {
            key: format!("{}:{}", if emerging { "emerging" } else { "niche" }, slug(&lead)),
            kind,
            terms: std::iter::once(lead.clone()).chain(main_lead.clone().filter(|m| *m != lead)).collect(),
            title,
            statement,
            parts: vec![part("Kebaruan", 1.0 - t.x), part("Bukti", t.documents as f64 / 30.0), part("Momentum", ctx.momentum(&docs))],
            signals: vec![Signal {
                analysis: "Peta tematik",
                finding: format!("Sentralitas {:.2}, densitas {:.2}, {} dokumen terbit {period}", t.centrality, t.density, pct(t.recent_share)),
                screen: "conceptual",
                tab: None,
            }],
            evidence,
            questions: {
                let main = main_lead.clone().filter(|m| *m != lead).unwrap_or_else(|| "tema-tema utama".into());
                vec![
                    format!("Bagaimana {lead} dapat dikaitkan dengan {main}?"),
                    format!("Apa yang dibutuhkan agar temuan tentang {lead} dapat diterapkan pada {main}?"),
                ]
            },
        });
    }
    out
}
