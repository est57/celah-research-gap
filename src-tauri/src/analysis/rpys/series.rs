//! The RPYS curve: deviation of each year's citations from the 5-year median, and its peaks.

/// Citations minus the median of the 5-year window centered on each year
/// (fewer years at the edges), as in Marx et al. and bibliometrix.
pub fn deviation(counts: &[usize]) -> Vec<i64> {
    (0..counts.len())
        .map(|i| {
            let lo = i.saturating_sub(2);
            let hi = (i + 2).min(counts.len() - 1);
            let mut window: Vec<usize> = counts[lo..=hi].to_vec();
            window.sort_unstable();
            let n = window.len();
            let median = if n % 2 == 1 { window[n / 2] as f64 } else { (window[n / 2 - 1] + window[n / 2]) as f64 / 2.0 };
            (counts[i] as f64 - median).round() as i64
        })
        .collect()
}

/// Indices of local maxima of `dev` at least 2 above the median (a lone extra
/// citation is noise), largest first, at most `limit`.
pub fn peaks(dev: &[i64], limit: usize) -> Vec<usize> {
    let mut out: Vec<usize> = (0..dev.len())
        .filter(|&i| {
            let left = i == 0 || dev[i] >= dev[i - 1];
            let right = i + 1 == dev.len() || dev[i] > dev[i + 1];
            dev[i] >= 2 && left && right
        })
        .collect();
    out.sort_by_key(|&i| (std::cmp::Reverse(dev[i]), i));
    out.truncate(limit);
    out
}

/// The year at quantile `p` (0..1) of a year -> count distribution, scanning upward.
pub fn quantile_year(years: &[(u16, usize)], p: f64) -> Option<u16> {
    let total: usize = years.iter().map(|y| y.1).sum();
    let target = (total as f64 * p).ceil().max(1.0) as usize;
    let mut seen = 0;
    for &(y, c) in years {
        seen += c;
        if seen >= target {
            return Some(y);
        }
    }
    years.last().map(|y| y.0)
}
