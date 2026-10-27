//! Generic co-occurrence network: items that appear together in documents
//! (references cited together, authors writing together), clustered with Louvain.

use super::louvain;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize)]
pub struct Node {
    /// Item key the node was built from (internal, not sent to the UI).
    #[serde(skip)]
    pub key: String,
    pub label: String,
    /// Documents containing the item (local citations or papers written).
    pub count: i64,
    pub cluster: usize,
}

#[derive(Serialize)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    /// Raw co-occurrence count.
    pub count: i64,
    /// Association strength c_ij / (c_i * c_j), scaled so the strongest edge is 1.
    pub weight: f64,
}

#[derive(Serialize)]
pub struct Cluster {
    pub id: usize,
    pub size: usize,
    pub top: Vec<String>,
}

/// Two sizeable clusters linked far less than their sizes predict.
#[derive(Serialize)]
pub struct Bridge {
    pub a: usize,
    pub b: usize,
    pub links: i64,
    /// Observed / expected co-occurrence between the two clusters, 0..1+.
    pub ratio: f64,
}

#[derive(Serialize)]
pub struct Network {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub clusters: Vec<Cluster>,
    pub bridges: Vec<Bridge>,
}

/// `docs` lists, per document, the item keys it contains (duplicates allowed).
/// `labels` gives a display label per key.
pub fn build(docs: &[Vec<String>], labels: &HashMap<String, String>, top: usize) -> Network {
    let mut count: HashMap<&str, i64> = HashMap::new();
    let mut unique_docs: Vec<Vec<&str>> = Vec::with_capacity(docs.len());
    for d in docs {
        let mut keys: Vec<&str> = d.iter().map(String::as_str).collect();
        keys.sort_unstable();
        keys.dedup();
        for k in &keys {
            *count.entry(k).or_default() += 1;
        }
        unique_docs.push(keys);
    }
    let mut ranked: Vec<(&str, i64)> = count.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    ranked.truncate(top);
    let index: HashMap<&str, usize> = ranked.iter().enumerate().map(|(i, (k, _))| (*k, i)).collect();

    let mut pairs: BTreeMap<(usize, usize), i64> = BTreeMap::new();
    for keys in &unique_docs {
        let ids: Vec<usize> = keys.iter().filter_map(|k| index.get(k).copied()).collect();
        for (a, &i) in ids.iter().enumerate() {
            for &j in &ids[a + 1..] {
                *pairs.entry((i.min(j), i.max(j))).or_default() += 1;
            }
        }
    }
    let assoc = |i: usize, j: usize, c: i64| c as f64 / (ranked[i].1 * ranked[j].1) as f64;
    let max_assoc = pairs.iter().map(|(&(i, j), &c)| assoc(i, j, c)).fold(0.0, f64::max);
    let edges: Vec<Edge> = pairs
        .iter()
        .map(|(&(i, j), &c)| Edge { from: i, to: j, count: c, weight: if max_assoc > 0.0 { assoc(i, j, c) / max_assoc } else { 0.0 } })
        .collect();
    let membership = louvain::communities(ranked.len(), &edges.iter().map(|e| (e.from, e.to, e.weight)).collect::<Vec<_>>());

    let nodes: Vec<Node> = ranked
        .iter()
        .enumerate()
        .map(|(i, (k, c))| Node {
            key: (*k).to_string(),
            label: labels.get(*k).cloned().unwrap_or_else(|| (*k).to_string()),
            count: *c,
            cluster: membership[i],
        })
        .collect();
    let clusters = summarize(&nodes);
    let bridges = weak_bridges(&nodes, &edges, &clusters);
    Network { nodes, edges, clusters, bridges }
}

fn summarize(nodes: &[Node]) -> Vec<Cluster> {
    let mut groups: BTreeMap<usize, Vec<&Node>> = BTreeMap::new();
    for n in nodes {
        groups.entry(n.cluster).or_default().push(n);
    }
    let mut out: Vec<Cluster> = groups
        .into_iter()
        .map(|(id, ns)| Cluster { id, size: ns.len(), top: ns.iter().take(3).map(|n| n.label.clone()).collect() })
        .collect();
    out.sort_by(|a, b| b.size.cmp(&a.size).then(a.id.cmp(&b.id)));
    out
}

/// Ratio of observed to expected links between clusters with at least three nodes,
/// using the same null model as modularity. The weakest three pairs are returned.
fn weak_bridges(nodes: &[Node], edges: &[Edge], clusters: &[Cluster]) -> Vec<Bridge> {
    let mut degree: HashMap<usize, i64> = HashMap::new();
    let mut between: HashMap<(usize, usize), i64> = HashMap::new();
    let mut total = 0;
    for e in edges {
        let (ca, cb) = (nodes[e.from].cluster, nodes[e.to].cluster);
        *degree.entry(ca).or_default() += e.count;
        *degree.entry(cb).or_default() += e.count;
        total += 2 * e.count;
        if ca != cb {
            *between.entry((ca.min(cb), ca.max(cb))).or_default() += e.count;
        }
    }
    if total == 0 {
        return Vec::new();
    }
    let big: Vec<usize> = clusters.iter().filter(|c| c.size >= 3).map(|c| c.id).collect();
    let mut out = Vec::new();
    for (x, &a) in big.iter().enumerate() {
        for &b in &big[x + 1..] {
            let links = between.get(&(a.min(b), a.max(b))).copied().unwrap_or(0);
            let expected = (degree.get(&a).copied().unwrap_or(0) * degree.get(&b).copied().unwrap_or(0)) as f64 / total as f64;
            if expected > 0.0 {
                out.push(Bridge { a, b, links, ratio: links as f64 / expected });
            }
        }
    }
    out.sort_by(|p, q| p.ratio.total_cmp(&q.ratio));
    out.truncate(3);
    out
}
