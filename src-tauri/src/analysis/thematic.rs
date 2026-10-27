//! Thematic map (strategic diagram, Cobo et al. 2011) from keyword clusters.
//! Centrality = links to other themes, density = links inside the theme (Callon).

use super::cooccur::Net;
use super::louvain;
use serde::Serialize;
use std::collections::{BTreeMap, HashSet};

/// Years before the current one that count as "recent".
const RECENT_YEARS: i32 = 3;

#[derive(Serialize)]
pub struct ThemeWord {
    pub label: String,
    pub freq: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub id: usize,
    pub name: String,
    pub words: Vec<ThemeWord>,
    pub frequency: i64,
    pub documents: usize,
    pub centrality: f64,
    pub density: f64,
    /// Rank positions in 0..1 used to place the theme on the map.
    pub x: f64,
    pub y: f64,
    /// "motor", "niche", "emerging" (emerging or declining) or "basic".
    pub quadrant: &'static str,
    /// Share of the theme's dated documents from the last complete years.
    pub recent_share: f64,
    /// Every keyword of the theme, as indices into the network it was built from.
    #[serde(skip)]
    pub members: Vec<usize>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThematicMap {
    pub themes: Vec<Theme>,
    pub collection_recent_share: f64,
    pub recent_from: i32,
    pub recent_to: i32,
}

fn recent_share(docs: impl Iterator<Item = usize>, years: &[Option<i32>], from: i32, to: i32) -> f64 {
    let (mut dated, mut recent) = (0, 0);
    for y in docs.filter_map(|d| years[d]) {
        dated += 1;
        if (from..=to).contains(&y) {
            recent += 1;
        }
    }
    if dated == 0 { 0.0 } else { f64::from(recent) / f64::from(dated) }
}

/// Rank of each value in 0..1, so themes spread evenly across the map.
pub(crate) fn ranks(values: &[f64]) -> Vec<f64> {
    let mut order: Vec<usize> = (0..values.len()).collect();
    order.sort_by(|&a, &b| values[a].total_cmp(&values[b]));
    let mut out = vec![0.0; values.len()];
    for (rank, &i) in order.iter().enumerate() {
        out[i] = (rank as f64 + 0.5) / values.len() as f64;
    }
    out
}

pub fn compute(net: &Net, min_per_thousand: u32, labels: usize, current_year: i32) -> ThematicMap {
    let edges = net.equivalence();
    let membership = louvain::communities(net.labels.len(), &edges);
    let mut groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (word, &c) in membership.iter().enumerate() {
        groups.entry(c).or_default().push(word);
    }
    // Rounded up: themes need at least this many keyword occurrences.
    let threshold = (i64::from(min_per_thousand) * net.years.len() as i64 + 999) / 1000;
    let (recent_from, recent_to) = (current_year - RECENT_YEARS, current_year - 1);

    let mut themes: Vec<Theme> = Vec::new();
    for words in groups.values() {
        let frequency: i64 = words.iter().map(|&w| net.freq[w]).sum();
        if frequency < threshold.max(1) {
            continue;
        }
        let inside: HashSet<usize> = words.iter().copied().collect();
        let (mut internal, mut external) = (0.0, 0.0);
        for &(i, j, e) in &edges {
            match (inside.contains(&i), inside.contains(&j)) {
                (true, true) => internal += e,
                (true, false) | (false, true) => external += e,
                _ => {}
            }
        }
        let mut sorted = words.clone();
        sorted.sort_by(|&a, &b| net.freq[b].cmp(&net.freq[a]).then(net.labels[a].cmp(&net.labels[b])));
        let docs: HashSet<usize> = words.iter().flat_map(|&w| net.docs[w].iter().copied()).collect();
        themes.push(Theme {
            id: 0,
            name: sorted.iter().take(labels).map(|&w| net.labels[w].as_str()).collect::<Vec<_>>().join(" · "),
            words: sorted.iter().take(12).map(|&w| ThemeWord { label: net.labels[w].clone(), freq: net.freq[w] }).collect(),
            frequency,
            documents: docs.len(),
            centrality: external * 10.0,
            density: internal / words.len() as f64 * 100.0,
            x: 0.0,
            y: 0.0,
            quadrant: "",
            recent_share: recent_share(docs.into_iter(), &net.years, recent_from, recent_to),
            members: sorted,
        });
    }

    let xs = ranks(&themes.iter().map(|t| t.centrality).collect::<Vec<_>>());
    let ys = ranks(&themes.iter().map(|t| t.density).collect::<Vec<_>>());
    for (i, t) in themes.iter_mut().enumerate() {
        (t.x, t.y) = (xs[i], ys[i]);
        t.quadrant = match (t.x >= 0.5, t.y >= 0.5) {
            (true, true) => "motor",
            (false, true) => "niche",
            (false, false) => "emerging",
            (true, false) => "basic",
        };
    }
    themes.sort_by_key(|t| std::cmp::Reverse(t.frequency));
    for (i, t) in themes.iter_mut().enumerate() {
        t.id = i;
    }
    let collection_recent_share = recent_share(0..net.years.len(), &net.years, recent_from, recent_to);
    ThematicMap { themes, collection_recent_share, recent_from, recent_to }
}
