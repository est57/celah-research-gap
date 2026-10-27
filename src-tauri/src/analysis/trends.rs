//! Trend topics: the publication-year spread of each frequent keyword (bibliometrix fieldByYear).

use super::cooccur::Net;
use serde::Serialize;
use std::collections::BTreeMap;

const MAX_TRENDS: usize = 40;

#[derive(Serialize)]
pub struct Trend {
    pub label: String,
    pub freq: i64,
    pub q1: i32,
    pub median: i32,
    pub q3: i32,
}

/// Quantile with linear interpolation (R type 7), rounded to a whole year.
fn quantile(sorted: &[i32], p: f64) -> i32 {
    let h = (sorted.len() - 1) as f64 * p;
    let (lo, hi) = (h.floor() as usize, h.ceil() as usize);
    let v = f64::from(sorted[lo]) + (f64::from(sorted[hi]) - f64::from(sorted[lo])) * (h - h.floor());
    v.round() as i32
}

/// Keywords with at least `min_freq` dated documents; at most `per_year` per median year.
pub fn compute(net: &Net, min_freq: usize, per_year: usize) -> Vec<Trend> {
    let mut by_median: BTreeMap<i32, Vec<Trend>> = BTreeMap::new();
    for (w, docs) in net.docs.iter().enumerate() {
        let mut years: Vec<i32> = docs.iter().filter_map(|&d| net.years[d]).collect();
        if years.len() < min_freq.max(1) {
            continue;
        }
        years.sort_unstable();
        let t = Trend {
            label: net.labels[w].clone(),
            freq: years.len() as i64,
            q1: quantile(&years, 0.25),
            median: quantile(&years, 0.5),
            q3: quantile(&years, 0.75),
        };
        by_median.entry(t.median).or_default().push(t);
    }
    let mut out = Vec::new();
    for (_, mut group) in by_median {
        group.sort_by(|a, b| b.freq.cmp(&a.freq).then(a.label.cmp(&b.label)));
        out.extend(group.into_iter().take(per_year));
    }
    // Keep the newest topics when there are too many to show.
    let skip = out.len().saturating_sub(MAX_TRENDS);
    out.into_iter().skip(skip).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn interpolates_quantiles() {
        let years = [2018, 2019, 2020, 2021];
        assert_eq!(super::quantile(&years, 0.5), 2020, "2019.5 rounds up");
        assert_eq!(super::quantile(&years, 0.25), 2019, "2018.75 rounds to 2019");
        assert_eq!(super::quantile(&[2022], 0.75), 2022);
    }
}
