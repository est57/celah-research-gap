//! "Matrix" gaps: two frequent keywords that appear together far less often
//! than their frequencies predict (observed / expected co-occurrence).

use super::model::{part, score, Draft};
use super::pair_text::draft;
use super::Ctx;
use crate::tr;

const TOP_WORDS: usize = 60;
const MIN_EXPECTED: f64 = 3.0;
const MAX_RATIO: f64 = 0.35;
const MAX_RESULTS: usize = 6;
/// A keyword may appear in at most this many results, so one hub does not dominate.
const MAX_PER_WORD: usize = 2;

/// Context similarity above which two keywords are treated as substitutes.
const MAX_CONTEXT_SIMILARITY: f64 = 0.25;

/// Keywords sharing a main word ("tuberculosis detection" / "tuberculosis diagnosis")
/// are variants of one concept, not two fields that fail to meet.
fn share_word(a: &str, b: &str) -> bool {
    let words = |s: &str| -> Vec<String> {
        s.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| w.len() >= 4).map(str::to_string).collect()
    };
    let wb = words(b);
    words(a).iter().any(|w| wb.contains(w))
}

/// Cosine similarity of two keywords' co-occurrence profiles with the other top keywords.
/// Synonyms ("chest radiography" / "chest x-ray") rarely appear together but share the same
/// neighbours; real gaps (a method and a field that never met) have different neighbours.
fn context_similarity(co: &[Vec<f64>], i: usize, j: usize) -> f64 {
    let (mut dot, mut ni, mut nj) = (0.0, 0.0, 0.0);
    for (k, (a, b)) in co[i].iter().zip(&co[j]).enumerate() {
        if k != i && k != j {
            dot += a * b;
            ni += a * a;
            nj += b * b;
        }
    }
    if ni == 0.0 || nj == 0.0 { 0.0 } else { dot / (ni.sqrt() * nj.sqrt()) }
}

pub fn detect(ctx: &Ctx) -> Vec<Draft> {
    let net = &ctx.net;
    let n_docs = net.years.len() as f64;
    let n = net.labels.len().min(TOP_WORDS);
    let mut co = vec![vec![0.0; n]; n];
    for (&(i, j), &c) in &net.pairs {
        if i < n && j < n {
            co[i][j] = c as f64;
            co[j][i] = c as f64;
        }
    }
    let mut candidates = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let expected = (net.freq[i] * net.freq[j]) as f64 / n_docs;
            let observed = net.pairs.get(&(i, j)).copied().unwrap_or(0);
            let ratio = observed as f64 / expected;
            if expected < MIN_EXPECTED || ratio > MAX_RATIO || share_word(&net.labels[i], &net.labels[j]) {
                continue;
            }
            if context_similarity(&co, i, j) > MAX_CONTEXT_SIMILARITY {
                continue;
            }
            candidates.push((i, j, observed, expected, ratio));
        }
    }
    let mut drafts: Vec<(f64, Draft, usize, usize)> = candidates
        .into_iter()
        .map(|(i, j, observed, expected, ratio)| {
            let union: Vec<usize> = net.docs[i].iter().chain(&net.docs[j]).copied().collect();
            let parts = vec![part(tr!("Kebaruan", "Novelty"), 1.0 - ratio), part(tr!("Bukti", "Evidence"), expected / 10.0), part("Momentum", ctx.momentum(&union))];
            let s = score(&parts, &[0.5, 0.25, 0.25]);
            (s, draft(ctx, i, j, observed, expected, parts), i, j)
        })
        .collect();
    drafts.sort_by(|a, b| b.0.total_cmp(&a.0));

    let mut used = vec![0usize; n];
    let mut out = Vec::new();
    for (_, d, i, j) in drafts {
        if out.len() >= MAX_RESULTS {
            break;
        }
        if used[i] < MAX_PER_WORD && used[j] < MAX_PER_WORD {
            used[i] += 1;
            used[j] += 1;
            out.push(d);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn spots_variants_and_substitutes() {
        assert!(super::share_word("tuberculosis detection", "Tuberculosis diagnosis"));
        assert!(super::share_word("chest radiography", "chest X-ray images"));
        assert!(!super::share_word("deep learning", "cough sound"));
        // 0 and 1 share neighbours 2 and 3; 4 only meets 5, a word 0 never meets.
        let co = vec![
            vec![0.0, 0.0, 5.0, 4.0, 0.0, 0.0],
            vec![0.0, 0.0, 6.0, 3.0, 0.0, 0.0],
            vec![5.0, 6.0, 0.0, 0.0, 0.0, 0.0],
            vec![4.0, 3.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 7.0],
            vec![0.0, 0.0, 0.0, 0.0, 7.0, 0.0],
        ];
        assert!(super::context_similarity(&co, 0, 1) > 0.9);
        assert!(super::context_similarity(&co, 0, 4) < 0.1);
    }
}
