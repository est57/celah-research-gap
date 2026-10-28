//! Coupling map (bibliometrix couplingMap): clusters of bibliographically coupled
//! documents placed by centrality (links to other clusters, Callon) and impact
//! (citations normalized by publication year), labeled by their most specific keywords.

mod describe;
#[cfg(test)]
mod tests;

use super::conceptual::in_range;
use super::coupling::{self, Level};
use super::thematic::ranks;
use crate::error::AppResult;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const MAX_CLUSTERS: usize = 15;
const LABELS: usize = 3;
const TOP_DOCS: usize = 5;

#[derive(Deserialize)]
pub struct CouplingMapParams {
    /// Most coupled documents considered.
    pub documents: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterDoc {
    pub id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub cited_by: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CouplingCluster {
    pub id: usize,
    pub size: usize,
    pub keywords: Vec<String>,
    pub centrality: f64,
    /// Mean year-normalized citations; 1.0 is the collection average.
    pub impact: f64,
    /// Rank positions in 0..1 for the map.
    pub x: f64,
    pub y: f64,
    /// "motor", "niche", "basic" or "peripheral".
    pub quadrant: &'static str,
    pub documents: Vec<ClusterDoc>,
    /// Ids of every document in the cluster (internal, for gap detection).
    #[serde(skip)]
    pub members: Vec<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CouplingMap {
    /// Documents that share a cited work with another document.
    pub coupled: usize,
    /// Documents in the clustered network.
    pub used: usize,
    pub clusters: Vec<CouplingCluster>,
}

fn quadrant(x: f64, y: f64) -> &'static str {
    match (x >= 0.5, y >= 0.5) {
        (true, true) => "motor",
        (false, true) => "niche",
        (true, false) => "basic",
        (false, false) => "peripheral",
    }
}

pub fn compute(conn: &Connection, p: &CouplingMapParams) -> AppResult<CouplingMap> {
    let top = in_range(p.documents, 50, 500, crate::tr!("jumlah dokumen", "number of documents"))? as usize;
    let (coupled, net) = coupling::compute(conn, Level::Documents, top)?;
    let info = describe::load(conn)?;

    let mut groups: BTreeMap<usize, Vec<i64>> = BTreeMap::new();
    for n in &net.nodes {
        if let Some(id) = n.key.strip_prefix('d').and_then(|s| s.parse::<i64>().ok()) {
            groups.entry(n.cluster).or_default().push(id);
        }
    }
    let mut external: BTreeMap<usize, f64> = BTreeMap::new();
    for e in &net.edges {
        let (a, b) = (net.nodes[e.from].cluster, net.nodes[e.to].cluster);
        if a != b {
            *external.entry(a).or_default() += e.weight;
            *external.entry(b).or_default() += e.weight;
        }
    }
    let mut kept: Vec<(usize, Vec<i64>)> = groups.into_iter().filter(|(_, ids)| ids.len() >= 2).collect();
    kept.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(a.0.cmp(&b.0)));
    kept.truncate(MAX_CLUSTERS);

    let docs: Vec<Vec<&describe::DocInfo>> = kept.iter().map(|(_, ids)| ids.iter().filter_map(|id| info.get(id)).collect()).collect();
    let keywords = describe::labels(&docs, LABELS);
    let centrality: Vec<f64> = kept.iter().map(|(c, _)| external.get(c).copied().unwrap_or(0.0) * 10.0).collect();
    let impact: Vec<f64> = docs.iter().map(|d| d.iter().map(|x| x.normalized).sum::<f64>() / d.len().max(1) as f64).collect();
    let (xs, ys) = (ranks(&centrality), ranks(&impact));

    let clusters = kept
        .iter()
        .enumerate()
        .map(|(i, (_, ids))| {
            let mut list: Vec<(i64, &describe::DocInfo)> = ids.iter().filter_map(|id| info.get(id).map(|d| (*id, d))).collect();
            list.sort_by(|a, b| b.1.cited_by.cmp(&a.1.cited_by).then(a.0.cmp(&b.0)));
            CouplingCluster {
                id: i,
                size: ids.len(),
                keywords: keywords[i].clone(),
                centrality: centrality[i],
                impact: impact[i],
                x: xs[i],
                y: ys[i],
                quadrant: quadrant(xs[i], ys[i]),
                documents: list
                    .into_iter()
                    .take(TOP_DOCS)
                    .map(|(id, d)| ClusterDoc { id, title: d.title.clone(), year: d.year, cited_by: d.cited_by })
                    .collect(),
                members: ids.clone(),
            }
        })
        .collect();
    Ok(CouplingMap { coupled, used: net.nodes.len(), clusters })
}
