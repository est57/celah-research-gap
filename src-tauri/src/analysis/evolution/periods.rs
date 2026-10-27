//! Splits the publication years into periods, from the researcher's cut years or
//! automatically so each period holds about the same number of documents.

use crate::error::{AppError, AppResult};
use rusqlite::Connection;

pub const MAX_CUTS: usize = 3;

/// Cut year c closes a period: [start, c], then [c + 1, ...].
pub fn validate(cuts: &[i32]) -> AppResult<()> {
    let sorted = cuts.windows(2).all(|w| w[0] < w[1]);
    if cuts.len() > MAX_CUTS || !sorted || cuts.iter().any(|y| !(1900..=2100).contains(y)) {
        return Err(AppError::Invalid("tahun potong harus 1 sampai 3 tahun berurutan antara 1900 dan 2100".into()));
    }
    Ok(())
}

/// Dated publication years, ascending, one entry per document.
pub fn years(conn: &Connection) -> AppResult<Vec<i32>> {
    let mut stmt = conn.prepare("SELECT year FROM papers WHERE year IS NOT NULL ORDER BY year")?;
    let rows = stmt.query_map([], |r| r.get::<_, i32>(0))?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

/// Cut years at the document-count terciles, skipping cuts that would leave a period empty.
pub fn auto_cuts(years: &[i32]) -> Vec<i32> {
    let (Some(&first), Some(&last)) = (years.first(), years.last()) else { return Vec::new() };
    let mut cuts: Vec<i32> = Vec::new();
    for part in 1..3 {
        let y = years[years.len() * part / 3];
        let prev = cuts.last().copied().unwrap_or(first - 1);
        if y > prev && y < last {
            cuts.push(y);
        }
    }
    cuts
}

/// (from, to) of each period covered by `cuts` within the collection's years.
pub fn ranges(years: &[i32], cuts: &[i32]) -> Vec<(i32, i32)> {
    let (Some(&first), Some(&last)) = (years.first(), years.last()) else { return Vec::new() };
    let mut out = Vec::new();
    let mut start = first;
    for &c in cuts.iter().filter(|&&c| c >= first && c < last) {
        if c >= start {
            out.push((start, c));
            start = c + 1;
        }
    }
    out.push((start, last));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_years() {
        let years = [2018, 2019, 2019, 2020, 2021, 2022, 2023, 2023, 2024];
        assert_eq!(auto_cuts(&years), vec![2020, 2023]);
        assert_eq!(ranges(&years, &[2020, 2023]), vec![(2018, 2020), (2021, 2023), (2024, 2024)]);
        assert_eq!(ranges(&years, &[2010, 2030]), vec![(2018, 2024)], "cuts outside the years are ignored");
        assert_eq!(auto_cuts(&[2020, 2020, 2020]), Vec::<i32>::new());
        assert!(validate(&[2020, 2019]).is_err());
        assert!(validate(&[2001, 2002, 2003, 2004]).is_err());
        assert!(validate(&[2020]).is_ok());
    }
}
