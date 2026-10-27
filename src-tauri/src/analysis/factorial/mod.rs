//! Factorial analysis (conceptual structure map): MCA of the top keywords, with the
//! words clustered by Ward on the first two axes, as in bibliometrix.

mod docs;
mod jacobi;
mod mca;
#[cfg(test)]
mod tests;
mod ward;

use super::conceptual::{field_choice, in_range};
use super::cooccur;
use crate::error::AppResult;
pub use docs::FactorDoc;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MAX_CLUSTERS: usize = 8;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FactorialParams {
    /// "auto", "author" or "plus".
    pub field: String,
    pub words: u32,
    /// 0 picks the number of clusters automatically.
    pub clusters: u32,
}

#[derive(Serialize)]
pub struct FactorWord {
    pub label: String,
    pub freq: i64,
    pub x: f64,
    pub y: f64,
    pub cluster: usize,
}

#[derive(Serialize)]
pub struct FactorCluster {
    pub id: usize,
    pub size: usize,
    pub top: Vec<String>,
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Factorial {
    pub field: &'static str,
    /// Documents with at least one of the keywords.
    pub documents: usize,
    /// Benzecri-corrected inertia of axes 1 and 2, in 0..1.
    pub explained: [f64; 2],
    pub words: Vec<FactorWord>,
    pub clusters: Vec<FactorCluster>,
    pub top_documents: Vec<FactorDoc>,
}

pub fn compute(conn: &Connection, p: &FactorialParams) -> AppResult<Factorial> {
    // The eigen problem has 2 x words rows; 100 keeps it well under a second.
    let words = in_range(p.words, 10, 100, "jumlah kata")? as usize;
    let k = in_range(p.clusters, 0, MAX_CLUSTERS as u32, "jumlah klaster")? as usize;
    let field = field_choice(conn, &p.field)?;
    let net = cooccur::build(conn, field, words)?;
    let mut doc_terms: HashMap<usize, Vec<usize>> = HashMap::new();
    for (w, docs) in net.docs.iter().enumerate() {
        for &d in docs {
            doc_terms.entry(d).or_default().push(w);
        }
    }
    let n = doc_terms.len() as i64;
    let empty = Factorial { field: field.label(), documents: doc_terms.len(), explained: [0.0, 0.0], words: vec![], clusters: vec![], top_documents: vec![] };
    // A keyword in every document has no "absent" category and carries no contrast.
    if net.freq.iter().any(|&f| f >= n) {
        return Ok(empty);
    }
    let Some(m) = mca::compute(&net.freq, &net.pairs, n) else { return Ok(empty) };
    let labels = ward::cluster(&m.words, (k >= 2).then_some(k), MAX_CLUSTERS.min(net.labels.len().saturating_sub(1)).max(2));

    let mut clusters: Vec<FactorCluster> = Vec::new();
    for c in 0..=labels.iter().copied().max().unwrap_or(0) {
        let mut ids: Vec<usize> = (0..labels.len()).filter(|&i| labels[i] == c).collect();
        if ids.is_empty() {
            continue;
        }
        let mean = |f: fn(&(f64, f64)) -> f64| ids.iter().map(|&i| f(&m.words[i])).sum::<f64>() / ids.len() as f64;
        let (x, y) = (mean(|p| p.0), mean(|p| p.1));
        ids.sort_by(|&a, &b| net.freq[b].cmp(&net.freq[a]));
        clusters.push(FactorCluster { id: c, size: ids.len(), top: ids.iter().take(4).map(|&i| net.labels[i].clone()).collect(), x, y });
    }
    let top_documents = docs::top(conn, &m, &doc_terms, &net.paper_ids, &clusters)?;
    Ok(Factorial {
        field: field.label(),
        documents: doc_terms.len(),
        explained: [m.explained.0, m.explained.1],
        words: (0..net.labels.len())
            .map(|i| FactorWord { label: net.labels[i].clone(), freq: net.freq[i], x: m.words[i].0, y: m.words[i].1, cluster: labels[i] })
            .collect(),
        clusters,
        top_documents,
    })
}
