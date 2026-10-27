//! Eigen decomposition of a small symmetric matrix by the cyclic Jacobi method.

const MAX_SWEEPS: usize = 60;
const EPS: f64 = 1e-12;

/// `a` is n x n, row-major, symmetric. Returns eigenvalues in descending order and
/// the matching eigenvectors (each of length n).
pub fn eigen(mut a: Vec<f64>, n: usize) -> (Vec<f64>, Vec<Vec<f64>>) {
    let mut v = vec![0.0; n * n];
    for i in 0..n {
        v[i * n + i] = 1.0;
    }
    for _ in 0..MAX_SWEEPS {
        let off: f64 = (0..n).flat_map(|i| (i + 1..n).map(move |j| (i, j))).map(|(i, j)| a[i * n + j].powi(2)).sum();
        if off < EPS {
            break;
        }
        for p in 0..n {
            for q in p + 1..n {
                let apq = a[p * n + q];
                if apq.abs() < 1e-300 {
                    continue;
                }
                let theta = (a[q * n + q] - a[p * n + p]) / (2.0 * apq);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let t = if theta == 0.0 { 1.0 } else { t };
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                // Symmetric update: only rows/columns p and q change.
                let (app, aqq) = (a[p * n + p], a[q * n + q]);
                for k in (0..n).filter(|&k| k != p && k != q) {
                    let (akp, akq) = (a[k * n + p], a[k * n + q]);
                    let (np, nq) = (c * akp - s * akq, s * akp + c * akq);
                    (a[k * n + p], a[p * n + k], a[k * n + q], a[q * n + k]) = (np, np, nq, nq);
                }
                a[p * n + p] = c * c * app - 2.0 * c * s * apq + s * s * aqq;
                a[q * n + q] = s * s * app + 2.0 * c * s * apq + c * c * aqq;
                a[p * n + q] = 0.0;
                a[q * n + p] = 0.0;
                for k in 0..n {
                    let (vkp, vkq) = (v[k * n + p], v[k * n + q]);
                    v[k * n + p] = c * vkp - s * vkq;
                    v[k * n + q] = s * vkp + c * vkq;
                }
            }
        }
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&x, &y| a[y * n + y].total_cmp(&a[x * n + x]));
    let values = order.iter().map(|&i| a[i * n + i]).collect();
    let vectors = order.iter().map(|&i| (0..n).map(|k| v[k * n + i]).collect()).collect();
    (values, vectors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decomposes_a_known_matrix() {
        // Eigenvalues of [[2,1,0],[1,2,0],[0,0,5]] are 5, 3, 1.
        let (values, vectors) = eigen(vec![2.0, 1.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0, 5.0], 3);
        for (got, want) in values.iter().zip([5.0, 3.0, 1.0]) {
            assert!((got - want).abs() < 1e-9, "{values:?}");
        }
        let v = &vectors[1];
        assert!((v[0].abs() - 0.5f64.sqrt()).abs() < 1e-9 && (v[0] - v[1]).abs() < 1e-9 && v[2].abs() < 1e-9);
    }
}
