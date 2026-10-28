//! Explicit gaps: sentences in abstracts where authors name what is still unknown.

use super::model::{part, slug, Draft, EvidenceRef, Kind, Signal};
use super::Ctx;
use crate::tr;
use crate::error::AppResult;
use rusqlite::Connection;
use std::collections::{BTreeMap, HashMap};

const CUES: [&str; 18] = [
    "future research", "future work", "future studies", "further research", "further studies",
    "further investigation", "remains unclear", "remain unclear", "remains unknown", "little is known",
    "not been explored", "not yet been", "limited evidence", "knowledge gap", "research gap",
    "warrant further", "should be explored", "needs to be validated",
];
const MAX_QUOTE: usize = 400;
const MIN_SENTENCES: usize = 2;
const MAX_RESULTS: usize = 6;

fn gap_sentences(text: &str) -> Vec<String> {
    text.split_inclusive(['.', '?', '!'])
        .map(str::trim)
        .filter(|s| {
            let low = s.to_lowercase();
            CUES.iter().any(|c| low.contains(c))
        })
        .map(|s| s.chars().take(MAX_QUOTE).collect())
        .collect()
}

pub fn detect(conn: &Connection, ctx: &Ctx) -> AppResult<Vec<Draft>> {
    let net = &ctx.net;
    let doc_of: HashMap<i64, usize> = net.paper_ids.iter().enumerate().map(|(d, &id)| (id, d)).collect();
    let mut words_of: Vec<Vec<usize>> = vec![Vec::new(); net.years.len()];
    for (w, docs) in net.docs.iter().enumerate() {
        for &d in docs {
            words_of[d].push(w);
        }
    }
    // Group each quote under the document's most specific keyword (rarest, but used at least 3 times).
    let mut groups: BTreeMap<usize, Vec<(usize, String)>> = BTreeMap::new();
    let mut stmt = conn.prepare("SELECT id, abstract FROM papers WHERE abstract IS NOT NULL")?;
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let (id, text): (i64, String) = (r.get(0)?, r.get(1)?);
        let Some(&d) = doc_of.get(&id) else { continue };
        let Some(&w) = words_of[d].iter().filter(|&&w| net.freq[w] >= 3).max_by_key(|&&w| (std::cmp::Reverse(net.freq[w]), w)) else {
            continue;
        };
        if let Some(sentence) = gap_sentences(&text).into_iter().next() {
            groups.entry(w).or_default().push((d, sentence));
        }
    }

    let max_freq = net.freq.first().copied().unwrap_or(1).max(1) as f64;
    let mut out: Vec<Draft> = groups
        .into_iter()
        .filter(|(_, quotes)| quotes.len() >= MIN_SENTENCES)
        .map(|(w, quotes)| {
            let kw = &net.labels[w];
            let docs: Vec<usize> = quotes.iter().map(|(d, _)| *d).collect();
            Draft {
                key: format!("explicit:{}", slug(kw)),
                kind: Kind::Explicit,
                terms: vec![kw.clone()],
                title: tr!(format!("Penulis menyebut {kw} masih perlu diteliti lebih lanjut"), format!("Authors say {kw} needs further study")),
                statement: tr!(
                    format!(
                        "{} dokumen tentang \"{kw}\" menyatakan sendiri di abstraknya bahwa ada hal yang belum terjawab atau perlu riset lanjutan.",
                        quotes.len()
                    ),
                    format!("{} documents on \"{kw}\" state in their abstracts that open questions remain or further research is needed.", quotes.len())
                ),
                parts: vec![part(tr!("Kebaruan", "Novelty"), 1.0 - net.freq[w] as f64 / max_freq), part(tr!("Bukti", "Evidence"), quotes.len() as f64 / 5.0), part("Momentum", ctx.momentum(&docs))],
                signals: vec![Signal {
                    analysis: tr!("Kalimat celah di abstrak", "Gap sentences in abstracts"),
                    finding: tr!(
                        format!("{} kalimat seperti \"future research\" atau \"remains unclear\"", quotes.len()),
                        format!("{} sentences such as \"future research\" or \"remains unclear\"", quotes.len())
                    ),
                    screen: "collection",
                    tab: None,
                }],
                evidence: quotes
                    .iter()
                    .take(4)
                    .map(|(d, q)| EvidenceRef { paper_id: net.paper_ids[*d], role: "mendukung", quote: Some(q.clone()) })
                    .collect(),
                questions: vec![tr!(
                    format!("Pertanyaan apa tentang {kw} yang disebut belum terjawab oleh studi-studi ini, dan bagaimana mengujinya?"),
                    format!("Which questions about {kw} do these studies leave open, and how could they be tested?")
                )],
            }
        })
        .collect();
    out.sort_by_key(|d| std::cmp::Reverse(d.evidence.len()));
    out.truncate(MAX_RESULTS);
    Ok(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn finds_cue_sentences() {
        let s = super::gap_sentences("CNNs work well. Robustness to phone microphones remains unclear. Future research should test field data!");
        assert_eq!(s.len(), 2);
        assert!(s[0].starts_with("Robustness"));
    }
}
