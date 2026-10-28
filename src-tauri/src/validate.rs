use crate::error::{AppError, AppResult};
use crate::search::Source;

pub const DOC_TYPES: [&str; 5] = ["article", "review", "preprint", "book-chapter", "proceedings"];
const MAX_QUERY: usize = 300;
const MAX_FILTER: usize = 200;

fn has_control(s: &str) -> bool {
    s.chars().any(|c| c.is_control())
}

/// Search text sent to OpenAlex: trimmed, 2 to 300 characters, printable.
pub fn query(raw: &str) -> AppResult<String> {
    let q = raw.trim();
    let len = q.chars().count();
    if !(2..=MAX_QUERY).contains(&len) {
        return Err(AppError::Invalid(crate::tr!("kata kunci harus 2 sampai 300 karakter", "keywords must be 2 to 300 characters").into()));
    }
    if has_control(q) {
        return Err(AppError::Invalid(crate::tr!("kata kunci berisi karakter tidak sah", "keywords contain invalid characters").into()));
    }
    Ok(q.to_string())
}

pub fn year_range(from: i32, to: i32) -> AppResult<(i32, i32)> {
    let ok = |y: i32| (1900..=2100).contains(&y);
    if !ok(from) || !ok(to) || from > to {
        return Err(AppError::Invalid(crate::tr!("rentang tahun tidak sah", "invalid year range").into()));
    }
    Ok((from, to))
}

pub fn doc_type(raw: Option<&str>) -> AppResult<Option<&'static str>> {
    match raw {
        None => Ok(None),
        Some(t) => DOC_TYPES
            .iter()
            .find(|d| **d == t)
            .map(|d| Some(*d))
            .ok_or_else(|| AppError::Invalid(crate::tr!("jenis dokumen tidak dikenal", "unknown document type").into())),
    }
}

pub fn max_results(n: u32) -> AppResult<u32> {
    if (1..=1000).contains(&n) {
        Ok(n)
    } else {
        Err(AppError::Invalid(crate::tr!("jumlah maksimum harus 1 sampai 1000", "maximum count must be 1 to 1000").into()))
    }
}

/// Local table filter. Empty input means no filter.
pub fn filter(raw: Option<&str>) -> AppResult<Option<String>> {
    let Some(f) = raw.map(str::trim).filter(|f| !f.is_empty()) else {
        return Ok(None);
    };
    if f.chars().count() > MAX_FILTER || has_control(f) {
        return Err(AppError::Invalid(crate::tr!("teks saringan tidak sah", "invalid filter text").into()));
    }
    Ok(Some(f.to_string()))
}

pub fn page(limit: u32, offset: u32) -> AppResult<(u32, u32)> {
    if !(1..=200).contains(&limit) || offset > 1_000_000 {
        return Err(AppError::Invalid(crate::tr!("parameter halaman tidak sah", "invalid page parameter").into()));
    }
    Ok((limit, offset))
}

/// At least one source, duplicates removed, original order kept.
pub fn sources(list: &[Source]) -> AppResult<Vec<Source>> {
    let mut out: Vec<Source> = Vec::new();
    for s in list {
        if !out.contains(s) {
            out.push(*s);
        }
    }
    if out.is_empty() {
        return Err(AppError::Invalid(crate::tr!("pilih minimal satu sumber data", "choose at least one data source").into()));
    }
    Ok(out)
}

/// API keys (OpenAlex, Semantic Scholar): 8 to 128 characters of letters, digits, '-' or '_'.
pub fn api_key(raw: &str) -> AppResult<String> {
    let key = raw.trim();
    let ok_len = (8..=128).contains(&key.len());
    let ok_chars = key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if !ok_len || !ok_chars {
        return Err(AppError::Invalid(crate::tr!("format kunci API tidak dikenali", "unrecognized API key format").into()));
    }
    Ok(key.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_short_and_control_queries() {
        assert!(query(" a ").is_err());
        assert!(query("tb\u{0007}x").is_err());
        assert_eq!(query("  tuberculosis ").unwrap(), "tuberculosis");
    }

    #[test]
    fn checks_years_types_and_paging() {
        assert!(year_range(2020, 2015).is_err());
        assert!(doc_type(Some("article; DROP")).is_err());
        assert_eq!(doc_type(Some("review")).unwrap(), Some("review"));
        assert!(page(0, 0).is_err());
        assert_eq!(filter(Some("   ")).unwrap(), None);
    }

    #[test]
    fn checks_api_key_format() {
        assert!(api_key("short").is_err());
        assert!(api_key("abc def ghi").is_err());
        assert!(api_key("key&x=1abc").is_err());
        assert_eq!(api_key("  Ab12_cd-34  ").unwrap(), "Ab12_cd-34");
    }
}
