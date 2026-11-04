//! Matches gap terms against document text, whole words only and case-insensitive.

use super::GapTerms;

/// A term found in more than this share of documents is too generic to tell journals apart
/// (for example the collection's own topic, which gap detection adds to every gap).
const GENERIC_SHARE: f64 = 0.5;

/// True when `needle` occurs in `hay` with no letter or digit directly before or after it,
/// so "ai" does not match inside "detail". Both are expected in lowercase.
pub fn has_word(hay: &str, needle: &str) -> bool {
    !needle.is_empty()
        && hay.match_indices(needle).any(|(i, _)| {
            let before = hay[..i].chars().next_back().is_none_or(|c| !c.is_alphanumeric());
            let after = hay[i + needle.len()..].chars().next().is_none_or(|c| !c.is_alphanumeric());
            before && after
        })
}

/// For each document text (lowercase), the indices of the gaps it matches. A document matches a
/// gap when it contains any of the gap's distinctive terms. Generic terms are dropped from a gap
/// unless all of its terms are generic.
pub fn matches(texts: &[String], gaps: &[GapTerms]) -> Vec<Vec<usize>> {
    let limit = (texts.len() as f64 * GENERIC_SHARE).max(1.0);
    let gap_terms: Vec<Vec<String>> = gaps
        .iter()
        .map(|g| {
            let mut all: Vec<String> = g.terms.iter().map(|t| t.trim().to_lowercase()).filter(|t| t.chars().count() >= 2).collect();
            all.sort();
            all.dedup();
            let hits: Vec<usize> = all.iter().map(|t| texts.iter().filter(|x| has_word(x, t)).count()).collect();
            let distinct: Vec<String> = all.iter().zip(&hits).filter(|(_, &h)| h as f64 <= limit).map(|(t, _)| t.clone()).collect();
            if distinct.is_empty() { all } else { distinct }
        })
        .collect();
    texts
        .iter()
        .map(|text| (0..gaps.len()).filter(|&g| gap_terms[g].iter().any(|t| has_word(text, t))).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_whole_words_only() {
        assert!(has_word("deep learning for cough", "cough"));
        assert!(has_word("x-ray (cnn)", "cnn"));
        assert!(!has_word("a detail", "ai"));
        assert!(!has_word("coughing", "cough"));
    }

    #[test]
    fn drops_generic_terms_unless_nothing_is_left() {
        let texts: Vec<String> = ["tb cough audio", "tb x-ray", "tb x-ray cnn"].iter().map(|s| s.to_string()).collect();
        let gaps = [
            GapTerms { title: "a".into(), terms: vec!["Cough".into(), "TB".into()] },
            GapTerms { title: "b".into(), terms: vec!["tb".into()] },
        ];
        let m = matches(&texts, &gaps);
        assert_eq!(m, vec![vec![0, 1], vec![1], vec![1]], "tb is generic for gap a but the only term of gap b");
    }
}
