//! Scimago Journal Rank export (scimagojr.com, "Download data"): semicolon-separated,
//! comma decimals, ISSNs without hyphens, the year in the "Total Docs. (2024)" header.

use crate::error::{AppError, AppResult};
use crate::text::{clean, normalize_issns};

const MAX_JOURNALS: usize = 80_000;

pub struct Journal {
    /// Normalized ISSNs ("09252312; 18728286").
    pub issns: String,
    pub title: String,
    pub sjr: Option<f64>,
    /// "Q1".."Q4", or None when Scimago gives "-".
    pub quartile: Option<String>,
    /// "Computer Vision and Pattern Recognition (Q1); Artificial Intelligence (Q2)".
    pub categories: Option<String>,
}

/// Lowercase title with single spaces, compared with stored source names.
pub fn title_key(title: &str) -> String {
    title.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

/// (journals, year of the ranking). Fails when the file is not an SJR export.
pub fn parse(text: &str) -> AppResult<(Vec<Journal>, Option<i32>)> {
    let not_sjr = || AppError::Invalid(crate::tr!("file ini bukan unduhan data SJR dari scimagojr.com", "this file is not an SJR data download from scimagojr.com").into());
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').flexible(true).from_reader(text.trim_start_matches('\u{feff}').as_bytes());
    let headers = rdr.headers().map_err(|_| not_sjr())?.clone();
    let col = |name: &str| headers.iter().position(|h| h.trim().eq_ignore_ascii_case(name));
    let (title, issn, sjr, quartile) = (col("Title"), col("Issn"), col("SJR"), col("SJR Best Quartile"));
    let (Some(title), Some(issn), Some(quartile)) = (title, issn, quartile) else { return Err(not_sjr()) };
    let categories = col("Categories");
    let year = headers.iter().find_map(|h| h.trim().strip_prefix("Total Docs. (")?.strip_suffix(')')?.parse().ok());
    let mut out = Vec::new();
    for rec in rdr.records().filter_map(Result::ok).take(MAX_JOURNALS) {
        let get = |i: usize| rec.get(i).map(str::trim).filter(|v| !v.is_empty() && *v != "-");
        let (Some(issns), Some(name)) = (get(issn).and_then(|v| normalize_issns([v])), get(title)) else { continue };
        let q = get(quartile).filter(|q| matches!(*q, "Q1" | "Q2" | "Q3" | "Q4")).map(str::to_string);
        out.push(Journal {
            issns,
            title: clean(name, 300),
            sjr: sjr.and_then(get).and_then(|v| v.replace(',', ".").parse().ok()).filter(|v: &f64| v.is_finite() && *v >= 0.0),
            quartile: q,
            categories: categories.and_then(get).map(|c| clean(c, 1_000)),
        });
    }
    if out.is_empty() {
        return Err(not_sjr());
    }
    Ok((out, year.filter(|y| (1999..=2100).contains(y))))
}

#[cfg(test)]
mod tests {
    #[test]
    fn reads_the_scimago_download_format() {
        let text = "\u{feff}Rank;Sourceid;Title;Type;Issn;SJR;SJR Best Quartile;H index;Total Docs. (2024);Categories\n\
            1;21100;\"Neurocomputing\";journal;\"18728286, 09252312\";\"1,471\";Q1;199;2000;\"Artificial Intelligence (Q1); Computer Science Applications (Q1)\"\n\
            2;21101;\"Some Bulletin\";journal;\"-\";\"0,100\";Q4;5;10;\"Medicine (Q4)\"\n\
            3;21102;\"New Journal\";journal;\"12345678\";\"-\";-;1;3;\"Oncology\"\n";
        let (rows, year) = super::parse(text).unwrap();
        assert_eq!(year, Some(2024));
        assert_eq!(rows.len(), 2, "a journal without ISSN cannot be matched");
        assert_eq!((rows[0].issns.as_str(), rows[0].sjr, rows[0].quartile.as_deref()), ("18728286; 09252312", Some(1.471), Some("Q1")));
        assert_eq!(rows[1].quartile, None);
        assert!(super::parse("a,b\n1,2\n").is_err());
    }
}
