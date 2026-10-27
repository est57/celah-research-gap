//! Text sanitizing shared by every importer. All external strings pass through here.

/// Removes control characters, collapses whitespace and bounds the length.
pub fn clean(raw: &str, max: usize) -> String {
    let text: String = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    text.chars().filter(|c| !c.is_control()).take(max).collect()
}

pub fn clean_opt(raw: Option<&str>, max: usize) -> Option<String> {
    raw.map(|s| clean(s, max)).filter(|s| !s.is_empty())
}

/// Removes markup such as JATS `<jats:p>` tags and decodes the basic entities.
pub fn strip_tags(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut in_tag = false;
    for c in raw.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => {
                in_tag = false;
                out.push(' ');
            }
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&#39;", "'").replace("&amp;", "&")
}

/// Lowercase bare DOI ("10.xxxx/..."), accepting doi.org URLs.
pub fn normalize_doi(raw: &str) -> Option<String> {
    let lower = raw.trim().to_lowercase();
    let doi = lower
        .trim_start_matches("https://doi.org/")
        .trim_start_matches("http://dx.doi.org/")
        .trim_start_matches("doi:");
    (doi.starts_with("10.") && doi.len() <= 300).then(|| clean(doi, 300))
}

/// "A; B" for one or two authors, "A; B; +n" for more.
pub fn authors_display(names: &[String]) -> String {
    match names.len() {
        0 => String::new(),
        1 | 2 => names.join("; "),
        n => format!("{}; {}; +{}", names[0], names[1], n - 2),
    }
}

/// Splits on `sep`, cleans each part and drops empty ones.
pub fn split_list(raw: &str, sep: char, max_items: usize, max_len: usize) -> Vec<String> {
    raw.split(sep).map(|s| clean(s, max_len)).filter(|s| !s.is_empty()).take(max_items).collect()
}

pub fn join_list(items: &[String], max: usize) -> Option<String> {
    clean_opt(Some(&items.join("; ")), max)
}

/// Maps source-specific document types onto the app's small vocabulary.
pub fn normalize_doc_type(raw: &str) -> Option<String> {
    let t = raw.trim().to_lowercase();
    // Order matters: "proceedings-article" must not fall through to "article".
    let mapped = if t.contains("review") {
        "review"
    } else if t.contains("conference") || t.contains("proceedings") {
        "proceedings"
    } else if t.contains("chapter") {
        "book-chapter"
    } else if t.contains("preprint") || t == "posted-content" {
        "preprint"
    } else if t == "j" || t.contains("article") {
        "article"
    } else {
        return clean_opt(Some(&t), 40);
    };
    Some(mapped.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_common_values() {
        assert_eq!(clean("a\u{0000}  b\n c", 10), "a b c");
        assert_eq!(normalize_doi("https://doi.org/10.1/ABC").as_deref(), Some("10.1/abc"));
        assert_eq!(normalize_doi("not a doi"), None);
        assert_eq!(normalize_doc_type("Proceedings Paper").as_deref(), Some("proceedings"));
        assert_eq!(normalize_doc_type("Article; Early Access").as_deref(), Some("article"));
        assert_eq!(normalize_doc_type("journal-article").as_deref(), Some("article"));
        assert_eq!(normalize_doc_type("proceedings-article").as_deref(), Some("proceedings"));
        assert_eq!(normalize_doc_type("posted-content").as_deref(), Some("preprint"));
        assert_eq!(split_list("a; ;b ", ';', 10, 5), vec!["a", "b"]);
        assert_eq!(clean(&strip_tags("<jats:p>A &amp; B</jats:p>"), 50), "A & B");
    }
}
