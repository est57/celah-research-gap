use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;

/// The chart shows at most this many years, ending at the newest one.
const MAX_SPAN: i32 = 60;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct YearPoint {
    pub year: i32,
    pub documents: i64,
    /// Mean citations per article (bibliometrix MeanTCperArt).
    pub mean_tc: f64,
    /// Years the documents could be cited, including the publication year.
    pub citable_years: i32,
    /// Mean citations per article divided by the years it could be cited
    /// (bibliometrix MeanTCperYear).
    pub mean_tc_per_year: f64,
    /// True for the current year, which is not complete yet.
    pub partial: bool,
}

pub fn compute(conn: &Connection, current_year: i32) -> AppResult<Vec<YearPoint>> {
    let mut stmt = conn.prepare(
        "SELECT year, COUNT(*), AVG(cited_by) FROM papers
         WHERE year IS NOT NULL AND year <= ?1 GROUP BY year ORDER BY year",
    )?;
    let rows: HashMap<i32, (i64, f64)> = stmt
        .query_map([current_year], |r| Ok((r.get::<_, i32>(0)?, (r.get(1)?, r.get(2)?))))?
        .collect::<Result<_, _>>()?;
    let (Some(&min), Some(&max)) = (rows.keys().min(), rows.keys().max()) else {
        return Ok(Vec::new());
    };
    let start = min.max(max - MAX_SPAN + 1);
    Ok((start..=max)
        .map(|year| {
            let (documents, avg_tc) = rows.get(&year).copied().unwrap_or((0, 0.0));
            let citable_years = (current_year - year + 1).max(1);
            YearPoint { year, documents, mean_tc: avg_tc, citable_years, mean_tc_per_year: avg_tc / f64::from(citable_years), partial: year == current_year }
        })
        .collect())
}

/// Compound annual growth between the first and last complete years, in percent.
pub fn growth_rate(points: &[YearPoint]) -> Option<f64> {
    let complete: Vec<&YearPoint> = points.iter().filter(|p| !p.partial && p.documents > 0).collect();
    let (first, last) = (complete.first()?, complete.last()?);
    let span = last.year - first.year;
    if span <= 0 {
        return None;
    }
    let ratio = last.documents as f64 / first.documents as f64;
    Some((ratio.powf(1.0 / f64::from(span)) - 1.0) * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn growth_skips_partial_year() {
        let p = |year, documents, partial| YearPoint { year, documents, mean_tc: 0.0, citable_years: 1, mean_tc_per_year: 0.0, partial };
        let points = vec![p(2020, 10, false), p(2021, 0, false), p(2022, 40, false), p(2023, 1, true)];
        let g = growth_rate(&points).unwrap();
        assert!((g - 100.0).abs() < 1e-9, "10 -> 40 over two years doubles each year");
        assert_eq!(growth_rate(&[p(2020, 5, false)]), None);
    }
}
