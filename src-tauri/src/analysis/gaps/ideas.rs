//! Saved researcher ideas as gap candidates, recomputed on the current (filtered,
//! synonym-aware) collection so their numbers stay current.

use super::model::{part, Draft, EvidenceRef, Kind, Signal};
use crate::analysis::idea::{compute, IdeaParams};
use crate::db::ideas;
use crate::error::AppResult;
use rusqlite::Connection;

pub fn detect(conn: &Connection) -> AppResult<Vec<Draft>> {
    let mut out = Vec::new();
    for saved in ideas::list(conn)? {
        let Ok(r) = compute(conn, &IdeaParams { a: saved.a.clone(), b: saved.b.clone() }) else { continue };
        let (a, b) = (r.a[0].clone(), r.b[0].clone());
        let ratio = if r.expected > 0.0 { r.with_both as f64 / r.expected } else { 1.0 };
        let momentum = ((r.recent_a + r.recent_b) / 2.0 / r.recent_all.max(0.05)).min(2.0) / 2.0;
        let mut evidence: Vec<EvidenceRef> = r.both_docs.iter().map(|d| EvidenceRef { paper_id: d.id, role: "hampir menjawab", quote: None }).collect();
        let context = r.a_only.iter().take(2).chain(r.b_only.iter().take(2));
        evidence.extend(context.map(|d| EvidenceRef { paper_id: d.id, role: "konteks", quote: None }));
        out.push(Draft {
            key: saved.key,
            kind: Kind::Idea,
            title: format!("{a} dengan {b}: ide yang Anda uji"),
            statement: format!(
                "\"{a}\" muncul di {} dokumen dan \"{b}\" di {} dokumen, tetapi keduanya hanya muncul bersama di {} dokumen, sementara bila tidak saling terkait diperkirakan sekitar {:.1}.",
                r.with_a, r.with_b, r.with_both, r.expected
            ),
            terms: vec![a.clone(), b.clone()],
            parts: vec![
                part("Kebaruan", 1.0 - ratio.min(1.0)),
                part("Bukti", ((r.with_a.min(r.with_b)) as f64 / 15.0).min(1.0)),
                part("Momentum", momentum),
            ],
            signals: vec![Signal {
                analysis: "Uji ide celah",
                finding: format!("{} dari {} dokumen menggabungkan keduanya (perkiraan {:.1})", r.with_both, r.documents, r.expected),
                screen: "gaps",
                tab: None,
            }],
            evidence,
            questions: vec![
                format!("Bagaimana {a} dapat diterapkan untuk {b}?"),
                format!("Apa yang berubah bila {a} dan {b} diteliti dalam satu desain studi?"),
            ],
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::analysis::gaps::{compute, model::Kind};
    use crate::db::{ideas, insert::insert_many, open_memory, papers::NewPaper, reviews};
    use crate::report::{build, ReportParams};

    fn paper(id: usize, title: &str) -> NewPaper {
        NewPaper { ext_id: format!("p{id}"), title: format!("{title} {id}"), year: Some(2020 + id as i32 % 5), cited_by: id as i64, ..Default::default() }
    }

    #[test]
    fn saved_ideas_become_gaps_and_reach_the_report() {
        let mut conn = open_memory();
        let mut items: Vec<NewPaper> = (0..8).map(|i| paper(i, "Vision transformers for chest radiographs")).collect();
        items.extend((10..18).map(|i| paper(i, "Automated sputum smear microscopy")));
        items.push(paper(30, "ViT on sputum smear images"));
        insert_many(&mut conn, &items, "Test").unwrap();
        let key = ideas::save(&conn, "Vision Transformer, ViT", "sputum smear").unwrap();
        assert_eq!(key, "idea:vision-transformer:sputum-smear");

        let list = compute(&conn).unwrap();
        let gap = list.gaps.iter().find(|g| g.kind == Kind::Idea).expect("the saved idea is a candidate");
        assert_eq!(gap.key, key);
        assert!(gap.evidence.iter().any(|e| e.role == "hampir menjawab"));
        assert!(gap.parts[0].value > 0.5, "1 document against an expectation of about 4 is novel");

        reviews::set(&conn, &key, "accepted", "", &gap.title).unwrap();
        let report = build(&conn, &ReportParams { style: "apa".into(), language: "id".into(), include_verification: false }).unwrap();
        assert!(report.paragraphs[0].text.contains("vision transformer") && report.paragraphs[0].text.contains("sputum smear"));

        ideas::remove(&conn, &key).unwrap();
        assert!(compute(&conn).unwrap().gaps.iter().all(|g| g.kind != Kind::Idea));
        assert!(ideas::remove(&conn, &key).is_err());
        assert!(ideas::save(&conn, "", "x y").is_err());
    }
}
