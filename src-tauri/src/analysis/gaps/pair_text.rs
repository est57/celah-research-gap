//! Text and evidence of a "matrix" gap: two keywords rarely studied together.

use super::model::{slug, Draft, EvidenceRef, Kind, ScorePart, Signal};
use super::Ctx;
use crate::tr;

pub(super) fn draft(ctx: &Ctx, i: usize, j: usize, observed: i64, expected: f64, parts: Vec<ScorePart>) -> Draft {
    let net = &ctx.net;
    let (a, b) = (&net.labels[i], &net.labels[j]);
    let both: Vec<usize> = net.docs[i].iter().filter(|d| net.docs[j].contains(d)).copied().collect();
    let mut evidence: Vec<EvidenceRef> = ctx.top(&both, 3).into_iter().map(|id| EvidenceRef { paper_id: id, role: "hampir menjawab", quote: None }).collect();
    for id in ctx.top(&net.docs[i], 2).into_iter().chain(ctx.top(&net.docs[j], 2)) {
        if !evidence.iter().any(|e| e.paper_id == id) {
            evidence.push(EvidenceRef { paper_id: id, role: "konteks", quote: None });
        }
    }
    Draft {
        key: format!("matrix:{}:{}", slug(a), slug(b)),
        kind: Kind::Matrix,
        terms: vec![a.clone(), b.clone()],
        title: tr!(format!("{a} dan {b} jarang diteliti bersama"), format!("{a} and {b} are rarely studied together")),
        statement: tr!(
            format!(
                "\"{a}\" muncul di {} dokumen dan \"{b}\" di {} dokumen, tetapi keduanya hanya muncul bersama di {observed} dokumen, padahal menurut frekuensinya diperkirakan sekitar {expected:.1}.",
                net.freq[i], net.freq[j]
            ),
            format!(
                "\"{a}\" appears in {} documents and \"{b}\" in {} documents, but they appear together in only {observed} documents, while their frequencies predict about {expected:.1}.",
                net.freq[i], net.freq[j]
            )
        ),
        parts,
        signals: vec![Signal {
            analysis: tr!("Ko-kemunculan kata kunci", "Keyword co-occurrence"),
            finding: tr!(
                format!("{a} dengan {b}: {observed} dokumen, perkiraan {expected:.1} (rasio {:.2})", observed as f64 / expected),
                format!("{a} with {b}: {observed} documents, expected {expected:.1} (ratio {:.2})", observed as f64 / expected)
            ),
            screen: "conceptual",
            tab: None,
        }],
        evidence,
        questions: vec![
            tr!(
                format!("Bagaimana pendekatan yang umum pada studi {a} dapat diterapkan untuk {b}?"),
                format!("How can approaches common in {a} studies be applied to {b}?")
            ),
            tr!(
                format!("Apa yang berubah jika {a} dan {b} diteliti dalam satu desain studi?"),
                format!("What changes when {a} and {b} are studied in one study design?")
            ),
        ],
    }
}
