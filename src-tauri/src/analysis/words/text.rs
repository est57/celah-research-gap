//! Words and short phrases of titles and abstracts.

use super::stop::is_stop;

/// Punctuation that ends a phrase: words on either side never form an n-gram.
fn breaks(c: char) -> bool {
    matches!(c, '.' | ',' | ';' | ':' | '(' | ')' | '[' | ']' | '!' | '?' | '"' | '/' | '|')
}

/// Kept words have 3+ characters, at least one letter, and are not stopwords.
fn keep(w: &str) -> bool {
    w.chars().count() >= 3 && w.chars().any(char::is_alphabetic) && !is_stop(w)
}

fn flush(run: &mut Vec<String>, n: usize, out: &mut Vec<String>) {
    if run.len() >= n {
        out.extend(run.windows(n).map(|w| w.join(" ")));
    }
    run.clear();
}

/// Distinct n-grams (n = 1 to 3) of `text`, lowercased, in first-seen order.
/// A stopword or punctuation mark splits phrases, so "deep learning for cancer"
/// gives "deep learning" but never "learning cancer".
pub fn grams(text: &str, n: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut run: Vec<String> = Vec::new();
    for segment in text.split(breaks) {
        for raw in segment.split(|c: char| !(c.is_alphanumeric() || c == '-' || c == '\'')) {
            let lower = raw.to_lowercase();
            let w = lower.trim_end_matches("'s").trim_matches(|c| c == '-' || c == '\'');
            if keep(w) {
                run.push(w.to_string());
            } else if !w.is_empty() || !raw.is_empty() {
                flush(&mut run, n, &mut out);
            }
        }
        flush(&mut run, n, &mut out);
    }
    let mut seen = std::collections::HashSet::new();
    out.retain(|g| seen.insert(g.clone()));
    out
}

/// Counting key that folds plurals: "neural networks" and "neural network" share one.
/// Words ending in "ss", "us" or "is" (class, virus, analysis) keep their "s".
pub fn key(gram: &str) -> String {
    let fold = |w: &str| -> String {
        let keep = w.len() < 4 || w.ends_with("ss") || w.ends_with("us") || w.ends_with("is");
        if keep { w.to_string() } else { w.strip_suffix('s').unwrap_or(w).to_string() }
    };
    gram.split(' ').map(fold).collect::<Vec<_>>().join(" ")
}
