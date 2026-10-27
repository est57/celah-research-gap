//! Lotka's law: how many authors wrote 1, 2, 3, ... documents.

use serde::Serialize;
use std::collections::BTreeMap;

/// Classic Lotka constant for beta = 2 (6 / pi^2).
const CLASSIC_C: f64 = 0.6079;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LotkaRow {
    pub documents: i64,
    pub authors: i64,
    /// Share of all authors, 0..1.
    pub observed: f64,
    /// Share predicted by the classic law C / n^2.
    pub theoretical: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Lotka {
    pub total_authors: i64,
    pub rows: Vec<LotkaRow>,
    /// Exponent fitted on this collection (log-log least squares), when there are enough points.
    pub beta: Option<f64>,
}

/// `docs_per_author` holds one entry per author: the number of documents they wrote.
pub fn compute(docs_per_author: impl Iterator<Item = i64>) -> Lotka {
    let mut freq: BTreeMap<i64, i64> = BTreeMap::new();
    for n in docs_per_author.filter(|n| *n > 0) {
        *freq.entry(n).or_default() += 1;
    }
    let total: i64 = freq.values().sum();
    let rows: Vec<LotkaRow> = freq
        .iter()
        .map(|(&documents, &authors)| LotkaRow {
            documents,
            authors,
            observed: authors as f64 / total.max(1) as f64,
            theoretical: CLASSIC_C / (documents as f64).powi(2),
        })
        .collect();
    Lotka { total_authors: total, beta: fit_beta(&rows), rows }
}

fn fit_beta(rows: &[LotkaRow]) -> Option<f64> {
    if rows.len() < 3 {
        return None;
    }
    let pts: Vec<(f64, f64)> = rows.iter().map(|r| ((r.documents as f64).ln(), r.observed.ln())).collect();
    let n = pts.len() as f64;
    let (sx, sy) = pts.iter().fold((0.0, 0.0), |(a, b), (x, y)| (a + x, b + y));
    let (mx, my) = (sx / n, sy / n);
    let (num, den) = pts.iter().fold((0.0, 0.0), |(a, b), (x, y)| (a + (x - mx) * (y - my), b + (x - mx).powi(2)));
    (den > 0.0).then(|| -num / den)
}

#[cfg(test)]
mod tests {
    #[test]
    fn follows_inverse_square() {
        // 600 authors with 1 doc, 150 with 2, 67 with 3, 38 with 4: close to 1/n^2.
        let data = std::iter::repeat_n(1, 600)
            .chain(std::iter::repeat_n(2, 150))
            .chain(std::iter::repeat_n(3, 67))
            .chain(std::iter::repeat_n(4, 38));
        let l = super::compute(data);
        assert_eq!(l.total_authors, 855);
        assert_eq!(l.rows.len(), 4);
        let beta = l.beta.unwrap();
        assert!((beta - 2.0).abs() < 0.05, "fitted beta {beta} should be near 2");
    }
}
