//! Matching key for cited references, so the same work cited by different
//! papers is counted once. References arrive as free text in source-specific
//! formats ("CHEN L, 2019, J DIGIT IMAGING, V32" in WoS, "Chen L., Title, (2019)
//! J Digit Imaging" in Scopus), so the key is: first-author surname + year +
//! first long word after the author part. Matching is reliable within one
//! export format and approximate across formats.

/// Returns (key, display label) or None when no surname or year can be found.
pub fn ref_key(raw: &str) -> Option<(String, String)> {
    let year = find_year(raw)?;
    let surname: String = raw
        .split(|c: char| c == ',' || c.is_whitespace())
        .find(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 2)?
        .chars()
        .filter(|c| c.is_alphabetic() || *c == '-')
        .collect();
    if surname.is_empty() {
        return None;
    }
    let rest = raw.split_once(',').map(|(_, r)| r).unwrap_or("");
    let word: String = rest
        .split(|c: char| !c.is_alphanumeric())
        .find(|w| w.chars().count() >= 4 && w.chars().all(char::is_alphabetic))
        .unwrap_or("")
        .to_lowercase();
    let key = format!("{} {year} {word}", surname.to_lowercase());
    Some((key, format!("{} {year}", title_case(&surname))))
}

/// First plausible publication year (1800..=2100) appearing as a standalone 4-digit number.
pub(crate) fn find_year(raw: &str) -> Option<u16> {
    let bytes = raw.as_bytes();
    for (i, w) in bytes.windows(4).enumerate() {
        let before = i.checked_sub(1).map(|j| bytes[j]);
        let after = bytes.get(i + 4);
        let digits = w.iter().all(u8::is_ascii_digit);
        let bounded = !before.is_some_and(|b| b.is_ascii_digit()) && !after.is_some_and(|b| b.is_ascii_digit());
        if digits && bounded {
            let y: u16 = std::str::from_utf8(w).ok()?.parse().ok()?;
            if (1800..=2100).contains(&y) {
                return Some(y);
            }
        }
    }
    None
}

fn title_case(word: &str) -> String {
    let lower = word.to_lowercase();
    let mut chars = lower.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::ref_key;

    #[test]
    fn same_wos_reference_gets_same_key() {
        let a = ref_key("CHEN L, 2019, J DIGIT IMAGING, V32, P1").unwrap();
        let b = ref_key("Chen L, 2019, J DIGIT IMAGING, V32, P1, DOI 10.1007/s1").unwrap();
        assert_eq!(a.0, b.0);
        assert_eq!(a.1, "Chen 2019");
    }

    #[test]
    fn handles_scopus_style_and_rejects_noise() {
        let (key, label) = ref_key("Lakhani P., Sundaram B., Deep learning at chest radiography, (2017) Radiology, 284").unwrap();
        assert_eq!(label, "Lakhani 2017");
        assert!(key.starts_with("lakhani 2017"));
        assert!(ref_key("no year here").is_none());
        assert!(ref_key("12345 67890").is_none());
    }
}
