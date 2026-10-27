//! Ward hierarchical clustering of 2D points (Lance-Williams update).

/// Cluster label per point (0.. by size, largest first) for `k` clusters, or for the
/// k in 2..=max_k after which the next merge jumps most when `k` is None.
pub fn cluster(points: &[(f64, f64)], k: Option<usize>, max_k: usize) -> Vec<usize> {
    let n = points.len();
    if n < 3 {
        return vec![0; n];
    }
    let mut d = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let (dx, dy) = (points[i].0 - points[j].0, points[i].1 - points[j].1);
            d[i][j] = dx * dx + dy * dy;
        }
    }
    let mut size = vec![1.0; n];
    let mut alive: Vec<bool> = vec![true; n];
    let mut members: Vec<Vec<usize>> = (0..n).map(|i| vec![i]).collect();
    let mut heights = Vec::new();
    let mut snapshots: Vec<Vec<Vec<usize>>> = Vec::new();
    for _ in 0..n - 1 {
        let (mut bi, mut bj, mut best) = (0, 0, f64::INFINITY);
        for i in (0..n).filter(|&i| alive[i]) {
            for j in (i + 1..n).filter(|&j| alive[j]) {
                if d[i][j] < best {
                    (bi, bj, best) = (i, j, d[i][j]);
                }
            }
        }
        heights.push(best);
        for m in (0..n).filter(|&m| alive[m] && m != bi && m != bj) {
            let (si, sj, sm) = (size[bi], size[bj], size[m]);
            d[bi][m] = ((si + sm) * d[bi][m] + (sj + sm) * d[bj][m] - sm * best) / (si + sj + sm);
            d[m][bi] = d[bi][m];
        }
        size[bi] += size[bj];
        alive[bj] = false;
        let moved = std::mem::take(&mut members[bj]);
        members[bi].extend(moved);
        snapshots.push((0..n).filter(|&i| alive[i]).map(|i| members[i].clone()).collect());
    }
    let top = max_k.min(n - 1).max(2);
    let k = k.unwrap_or_else(|| {
        // Merge m leaves n - m - 1 clusters; going from k to k - 1 costs heights[n - k].
        (2..=top).max_by(|&a, &b| {
            let jump = |k: usize| heights[n - k] - heights[n - k - 1];
            jump(a).total_cmp(&jump(b)).then(b.cmp(&a))
        }).unwrap_or(2)
    }).clamp(1, n);
    let mut groups = if k >= n { (0..n).map(|i| vec![i]).collect() } else { snapshots[n - k - 1].clone() };
    groups.sort_by(|a, b| b.len().cmp(&a.len()).then(a.iter().min().cmp(&b.iter().min())));
    let mut labels = vec![0; n];
    for (c, g) in groups.iter().enumerate() {
        for &i in g {
            labels[i] = c;
        }
    }
    labels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separates_obvious_groups() {
        let pts = [(0.0, 0.0), (0.1, 0.0), (0.0, 0.1), (10.0, 0.0), (10.1, 0.0), (5.0, 8.66), (5.1, 8.66)];
        let auto = cluster(&pts, None, 6);
        assert_eq!(auto, vec![0, 0, 0, 1, 1, 2, 2]);
        let two = cluster(&pts, Some(2), 6);
        assert_eq!(two[0], two[1]);
        assert_ne!(two[0], two[5]);
    }
}
