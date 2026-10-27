//! Louvain community detection (Blondel et al. 2008) on a weighted undirected graph.
//! Deterministic: nodes are visited in index order and ties go to the lowest community id.

use std::collections::BTreeMap;

const MAX_PASSES: usize = 50;
const MIN_GAIN: f64 = 1e-10;

struct Graph {
    adj: Vec<Vec<(usize, f64)>>,
    /// Weight of edges inside an aggregated node.
    self_w: Vec<f64>,
    degree: Vec<f64>,
    /// Sum of all degrees (2m).
    total: f64,
}

impl Graph {
    fn new(n: usize, edges: &[(usize, usize, f64)], self_w: Vec<f64>) -> Self {
        let mut adj = vec![Vec::new(); n];
        for &(i, j, w) in edges {
            if i != j && w > 0.0 {
                adj[i].push((j, w));
                adj[j].push((i, w));
            }
        }
        for a in &mut adj {
            a.sort_by_key(|&(j, _)| j);
        }
        let degree: Vec<f64> = (0..n).map(|i| adj[i].iter().map(|(_, w)| w).sum::<f64>() + 2.0 * self_w[i]).collect();
        let total = degree.iter().sum();
        Graph { adj, self_w, degree, total }
    }

    /// Moves nodes between communities while modularity improves. Returns a renumbered
    /// community per node and whether anything moved.
    fn one_level(&self) -> (Vec<usize>, bool) {
        let n = self.adj.len();
        let mut comm: Vec<usize> = (0..n).collect();
        let mut tot: Vec<f64> = self.degree.clone();
        let mut moved_any = false;
        for _ in 0..MAX_PASSES {
            let mut moved = false;
            for i in 0..n {
                let ci = comm[i];
                let ki = self.degree[i];
                tot[ci] -= ki;
                let mut links: BTreeMap<usize, f64> = BTreeMap::new();
                links.insert(ci, 0.0);
                for &(j, w) in &self.adj[i] {
                    *links.entry(comm[j]).or_default() += w;
                }
                let gain = |c: usize, w: f64| w - tot[c] * ki / self.total;
                let mut best = (ci, gain(ci, links[&ci]));
                for (&c, &w) in &links {
                    let g = gain(c, w);
                    if g > best.1 + MIN_GAIN {
                        best = (c, g);
                    }
                }
                tot[best.0] += ki;
                if best.0 != ci {
                    comm[i] = best.0;
                    moved = true;
                    moved_any = true;
                }
            }
            if !moved {
                break;
            }
        }
        let mut renumber = BTreeMap::new();
        let out = comm
            .iter()
            .map(|c| {
                let next = renumber.len();
                *renumber.entry(*c).or_insert(next)
            })
            .collect();
        (out, moved_any)
    }

    fn aggregate(&self, comm: &[usize]) -> Graph {
        let k = comm.iter().max().map_or(0, |m| m + 1);
        let mut self_w = vec![0.0; k];
        let mut weights: BTreeMap<(usize, usize), f64> = BTreeMap::new();
        for (i, row) in self.adj.iter().enumerate() {
            self_w[comm[i]] += self.self_w[i];
            for &(j, w) in row.iter().filter(|(j, _)| *j > i) {
                let (a, b) = (comm[i].min(comm[j]), comm[i].max(comm[j]));
                if a == b {
                    self_w[a] += w;
                } else {
                    *weights.entry((a, b)).or_default() += w;
                }
            }
        }
        let edges: Vec<(usize, usize, f64)> = weights.into_iter().map(|((a, b), w)| (a, b, w)).collect();
        Graph::new(k, &edges, self_w)
    }
}

/// Community id (0-based, contiguous) for each of the `n` nodes.
pub fn communities(n: usize, edges: &[(usize, usize, f64)]) -> Vec<usize> {
    let mut graph = Graph::new(n, edges, vec![0.0; n]);
    let mut membership: Vec<usize> = (0..n).collect();
    if graph.total <= 0.0 {
        return membership;
    }
    loop {
        let (local, moved) = graph.one_level();
        if !moved {
            break;
        }
        for m in membership.iter_mut() {
            *m = local[*m];
        }
        graph = graph.aggregate(&local);
        if graph.adj.len() <= 1 {
            break;
        }
    }
    membership
}

#[cfg(test)]
mod tests {
    #[test]
    fn separates_two_cliques_joined_by_one_edge() {
        let mut edges = Vec::new();
        for (a, b) in [(0, 1), (0, 2), (1, 2), (3, 4), (3, 5), (4, 5)] {
            edges.push((a, b, 1.0));
        }
        edges.push((2, 3, 0.1));
        let c = super::communities(6, &edges);
        assert_eq!(c[0], c[1]);
        assert_eq!(c[1], c[2]);
        assert_eq!(c[3], c[4]);
        assert_eq!(c[4], c[5]);
        assert_ne!(c[0], c[3]);
        assert_eq!(super::communities(6, &edges), c, "deterministic");
    }
}
