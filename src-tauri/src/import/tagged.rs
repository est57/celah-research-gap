//! Line-tagged exports shared by RIS and PubMed (MEDLINE): a tag of up to four
//! characters padded to column 4, then "- " and the value. A line without a tag
//! continues the value above it.

use super::limits::MAX_FIELD;
use crate::text::{clean, clean_opt, join_list, split_list};
use std::collections::HashMap;

pub type Fields = HashMap<String, Vec<String>>;

/// One value never grows past this, however many wrapped lines follow it.
const MAX_VALUE: usize = 60_000;

/// ("TI", "Deep learning") from "TI  - Deep learning"; ("ER", "") from "ER  -".
fn tag_of(line: &str) -> Option<(&str, &str)> {
    let b = line.as_bytes();
    if b.len() < 5 || b[4] != b'-' || (b.len() > 5 && b[5] != b' ') {
        return None;
    }
    let head = line.get(..4)?;
    let tag = head.trim_end();
    let ok = !tag.is_empty() && tag.bytes().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
    ok.then(|| (tag, line.get(6..).unwrap_or("").trim()))
}

/// Records opened by `start` and closed by `end` (or by the next `start`), at most `max`.
pub fn records(text: &str, start: &str, end: &str, max: usize) -> Vec<Fields> {
    let mut out = Vec::new();
    let mut cur = Fields::new();
    let mut last: Option<String> = None;
    for line in text.lines().map(|l| l.trim_end_matches('\r')) {
        if out.len() >= max {
            return out;
        }
        if let Some((tag, value)) = tag_of(line) {
            if tag == end || (tag == start && !cur.is_empty()) {
                out.extend((!cur.is_empty()).then(|| std::mem::take(&mut cur)));
                if tag == end {
                    last = None;
                    continue;
                }
            }
            cur.entry(tag.to_string()).or_default().push(value.to_string());
            last = Some(tag.to_string());
        } else if !line.trim().is_empty() {
            let value = last.as_ref().and_then(|t| cur.get_mut(t)).and_then(|v| v.last_mut());
            if let Some(v) = value.filter(|v| v.len() < MAX_VALUE) {
                v.push(' ');
                v.push_str(line.trim());
            }
        }
    }
    if out.len() < max && !cur.is_empty() {
        out.push(cur);
    }
    out
}

pub fn values<'a>(f: &'a Fields, tags: &'a [&str]) -> impl Iterator<Item = &'a str> {
    tags.iter().filter_map(|t| f.get(*t)).flatten().map(String::as_str)
}

/// The first of `tags` that has a value, joined and cleaned.
pub fn first(f: &Fields, tags: &[&str], max: usize) -> Option<String> {
    tags.iter().find_map(|t| f.get(*t).and_then(|v| clean_opt(Some(&v.join(" ")), max)))
}

/// Every value of `tags`, one item each.
pub fn list(f: &Fields, tags: &[&str], max_items: usize, max_len: usize) -> Vec<String> {
    values(f, tags).map(|s| clean(s, max_len)).filter(|s| !s.is_empty()).take(max_items).collect()
}

/// Keywords given one per line or separated by ";", without repeats.
pub fn keywords<'a>(items: impl Iterator<Item = &'a str>) -> Option<String> {
    let mut out: Vec<String> = Vec::new();
    for k in items.flat_map(|v| split_list(v, ';', 100, 80)) {
        if out.len() < 100 && !out.iter().any(|o| o.eq_ignore_ascii_case(&k)) {
            out.push(k);
        }
    }
    join_list(&out, MAX_FIELD)
}

/// The first four-digit year in "2023/01/05/", "2023 Jan 5" or "Jan 2023".
pub fn year(raw: &str) -> Option<i32> {
    raw.split(|c: char| !c.is_ascii_digit()).filter(|p| p.len() == 4).find_map(|p| p.parse().ok()).filter(|y| (1500..=2100).contains(y))
}

#[cfg(test)]
mod tests {
    #[test]
    fn splits_records_and_joins_wrapped_lines() {
        let text = "TY  - JOUR\nTI  - Deep learning\n      for screening\nKW  - a\nKW  - b; A\nER  -\n\nTY  - BOOK\nTI  - Second\nER  - \nnoise\n";
        let recs = super::records(text, "TY", "ER", 10);
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0]["TI"], vec!["Deep learning for screening"]);
        assert_eq!(super::keywords(super::values(&recs[0], &["KW"])).as_deref(), Some("a; b"));
        assert_eq!(super::records(text, "TY", "ER", 1).len(), 1);
        assert_eq!(super::year("2023/01/05/"), Some(2023));
        assert_eq!(super::year("Jan 1999"), Some(1999));
        assert_eq!(super::year("12"), None);
    }

    #[test]
    fn pubmed_records_start_at_each_pmid() {
        let text = "PMID- 1\nTI  - One\n\nPMID- 2\nTI  - Two\n";
        let recs = super::records(text, "PMID", "", 10);
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[1]["PMID"], vec!["2"]);
    }
}
