//! Finds the country in a free-text affiliation such as
//! "Dept. of CS, Univ X, Dhaka 1000, Bangladesh." or "Stanford Univ, Stanford, CA 94305 USA".

pub mod affil;
pub(crate) mod names;

/// Text after these markers is contact detail, not address.
const CUT_MARKERS: [&str; 3] = ["electronic address", "email:", "e-mail:"];

fn is_boundary(c: Option<char>) -> bool {
    c.is_none_or(|c| !c.is_alphanumeric())
}

/// ISO 3166-1 alpha-2 code of the country named last in the text, if any.
pub fn country_of(affiliation: &str) -> Option<&'static str> {
    let mut text = affiliation.to_lowercase();
    for marker in CUT_MARKERS {
        if let Some(i) = text.find(marker) {
            text.truncate(i);
        }
    }
    // The country normally closes the address, so the match that ends last wins;
    // on a tie the longer name wins ("south korea" over "korea").
    let mut best: Option<(usize, usize, &'static str)> = None;
    for &(name, code) in names::NAMES {
        let mut search_end = text.len();
        while let Some(start) = text[..search_end].rfind(name) {
            let end = start + name.len();
            let ok = is_boundary(text[..start].chars().next_back()) && is_boundary(text[end..].chars().next());
            if ok {
                if best.is_none_or(|(e, len, _)| end > e || (end == e && name.len() > len)) {
                    best = Some((end, name.len(), code));
                }
                break;
            }
            search_end = start;
        }
    }
    best.map(|(_, _, code)| code)
}

/// A valid-looking ISO alpha-2 code from an API (OpenAlex), uppercased.
pub fn normalize_code(code: &str) -> Option<String> {
    let c = code.trim().to_ascii_uppercase();
    (c.len() == 2 && c.chars().all(|ch| ch.is_ascii_uppercase())).then_some(c)
}

/// Adds a code to a list keeping first-seen order and no duplicates.
pub fn push_unique(list: &mut Vec<String>, code: &str) {
    if !list.iter().any(|c| c == code) {
        list.push(code.to_string());
    }
}

/// Distinct countries found in affiliation texts, in first-seen order.
pub fn countries_from<'a>(texts: impl IntoIterator<Item = &'a str>) -> Vec<String> {
    let mut out = Vec::new();
    for code in texts.into_iter().filter_map(country_of) {
        push_unique(&mut out, code);
    }
    out
}

/// For exports with a "; "-separated affiliation list and a separate
/// correspondence address (Scopus). Falls back to the first country.
pub fn from_list(affiliations: Option<&str>, correspondence: Option<&str>) -> (Vec<String>, Option<String>) {
    let countries = countries_from(affiliations.unwrap_or("").split(';'));
    let corresponding = correspondence
        .and_then(country_of)
        .map(str::to_string)
        .or_else(|| countries.first().cloned());
    (countries, corresponding)
}

#[cfg(test)]
mod tests {
    use super::country_of;

    #[test]
    fn reads_common_address_styles() {
        assert_eq!(country_of("[Rahman, M] Univ Dhaka, Dept CSE, Dhaka 1000, Bangladesh."), Some("BD"));
        assert_eq!(country_of("Stanford Univ, Stanford, CA 94305 USA"), Some("US"));
        assert_eq!(country_of("Peoples R China"), Some("CN"));
        assert_eq!(country_of("Seoul Natl Univ, Seoul, South Korea"), Some("KR"));
        assert_eq!(country_of("Univ Indonesia, Depok, Indonesia. Electronic address: a@ui.ac.id"), Some("ID"));
        assert_eq!(country_of("Imperial Coll London, London, England"), Some("GB"));
        // Seen in real PubMed/Europe PMC data: DR Congo without "the" must not become the Republic of Congo.
        assert_eq!(country_of("University of Kinshasa, Kinshasa, Democratic Republic of Congo."), Some("CD"));
        assert_eq!(country_of("Univ Marien Ngouabi, Brazzaville, Republic of the Congo"), Some("CG"));
        assert_eq!(country_of("Arab American University, Ramallah P600, Palestine."), Some("PS"));
        assert_eq!(country_of("Atlanta, Georgia, United States of America."), Some("US"));
    }

    #[test]
    fn avoids_partial_words() {
        assert_eq!(country_of("Indiana University, Bloomington, IN"), None, "'india' inside 'indiana' is not India");
        assert_eq!(country_of("Univ of Georgia, Athens, GA, USA"), Some("US"), "the country closes the address");
        assert_eq!(country_of(""), None);
    }
}
