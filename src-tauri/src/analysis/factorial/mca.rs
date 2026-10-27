//! Multiple correspondence analysis of Q binary keyword variables (present / absent)
//! over n documents, computed from the Burt matrix B = Z'Z, which follows directly from
//! keyword frequencies and co-occurrence counts. With N = nQ and column masses c,
//! M = D_c^-1/2 (B / (n Q^2) - c c') D_c^-1/2 has the squared singular values of the
//! indicator matrix as eigenvalues; column principal coordinates are sigma v / sqrt(c).

use super::jacobi;
use std::collections::HashMap;

pub struct Mca {
    /// Principal coordinates of each keyword's "present" category on axes 1 and 2.
    pub words: Vec<(f64, f64)>,
    /// Same for the "absent" categories, used to place documents.
    pub absent: Vec<(f64, f64)>,
    pub sigma: (f64, f64),
    /// Benzecri-corrected share of inertia of axes 1 and 2, in 0..1.
    pub explained: (f64, f64),
}

/// `freq[q]` documents contain keyword q, `pairs` counts documents with both, `n` documents in total.
pub fn compute(freq: &[i64], pairs: &HashMap<(usize, usize), i64>, n: i64) -> Option<Mca> {
    let q = freq.len();
    if q < 3 || n < 3 {
        return None;
    }
    let dim = 2 * q;
    let (nf, qf) = (n as f64, q as f64);
    let co = |a: usize, b: usize| if a == b { freq[a] } else { *pairs.get(&(a.min(b), a.max(b))).unwrap_or(&0) };
    // Category 2a = keyword a present, 2a + 1 = absent.
    let burt = |i: usize, j: usize| -> f64 {
        let (a, pa, b, pb) = (i / 2, i % 2 == 0, j / 2, j % 2 == 0);
        let (fa, fb, c) = (freq[a], freq[b], co(a, b));
        let v = match (a == b, pa, pb) {
            (true, true, true) => fa,
            (true, false, false) => n - fa,
            (true, _, _) => 0,
            (false, true, true) => c,
            (false, true, false) => fa - c,
            (false, false, true) => fb - c,
            (false, false, false) => n - fa - fb + c,
        };
        v as f64
    };
    let mass: Vec<f64> = (0..dim).map(|i| burt(i, i) / (nf * qf)).collect();
    if mass.iter().any(|&m| m <= 0.0) {
        return None;
    }
    let mut m = vec![0.0; dim * dim];
    for i in 0..dim {
        for j in 0..dim {
            m[i * dim + j] = (burt(i, j) / (nf * qf * qf) - mass[i] * mass[j]) / (mass[i] * mass[j]).sqrt();
        }
    }
    let (values, vectors) = jacobi::eigen(m, dim);
    let sigma = |k: usize| values.get(k).copied().unwrap_or(0.0).max(0.0).sqrt();
    let coord = |cat: usize, k: usize| sigma(k) * vectors[k][cat] / mass[cat].sqrt();
    // Benzecri: only eigenvalues above 1/Q carry information.
    let adjusted: Vec<f64> = values.iter().map(|&l| if l > 1.0 / qf { (qf / (qf - 1.0) * (l - 1.0 / qf)).powi(2) } else { 0.0 }).collect();
    let total: f64 = adjusted.iter().sum();
    let share = |k: usize| if total > 0.0 { adjusted.get(k).copied().unwrap_or(0.0) / total } else { 0.0 };
    Some(Mca {
        words: (0..q).map(|a| (coord(2 * a, 0), coord(2 * a, 1))).collect(),
        absent: (0..q).map(|a| (coord(2 * a + 1, 0), coord(2 * a + 1, 1))).collect(),
        sigma: (sigma(0), sigma(1)),
        explained: (share(0), share(1)),
    })
}

impl Mca {
    /// Row principal coordinates of a document from the keywords it contains
    /// (transition formula: the average of its categories, divided by sigma).
    pub fn document(&self, present: &[usize]) -> (f64, f64) {
        let q = self.words.len() as f64;
        let mut sum = self.absent.iter().fold((0.0, 0.0), |s, p| (s.0 + p.0, s.1 + p.1));
        for &w in present {
            sum.0 += self.words[w].0 - self.absent[w].0;
            sum.1 += self.words[w].1 - self.absent[w].1;
        }
        let div = |v: f64, s: f64| if s > 0.0 { v / (q * s) } else { 0.0 };
        (div(sum.0, self.sigma.0), div(sum.1, self.sigma.1))
    }
}
