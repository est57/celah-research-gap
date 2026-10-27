//! Institution names from free-text affiliations ("Dept CSE, Univ Dhaka, Dhaka 1000,
//! Bangladesh") and a key that matches WoS abbreviations to full names
//! ("Univ Malaya" and "University of Malaya" share "university malaya").

use super::country_of;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Affiliation {
    pub name: String,
    pub key: String,
    /// ISO alpha-2, when known.
    pub country: Option<String>,
}

const MAX_NAME: usize = 200;

/// Word stems that mark the institution part of an address, not the department or city.
const MARKERS: [&str; 33] = [
    "univ", "universit", "institut", "inst", "hosp", "hospital", "college", "coll", "school", "academ", "center", "centre",
    "ctr", "laborator", "polytech", "politekni", "ministry", "minist", "foundation", "fdn", "agency", "council", "corp",
    "clinic", "klinik", "rumah", "sekolah", "akademi", "research", "organi", "observator", "authority", "museum",
];

const ABBREVIATIONS: [(&str, &str); 14] = [
    ("univ", "university"), ("inst", "institute"), ("hosp", "hospital"), ("coll", "college"), ("ctr", "center"),
    ("cent", "center"), ("centre", "center"), ("natl", "national"), ("technol", "technology"), ("sci", "science"),
    ("med", "medical"), ("res", "research"), ("acad", "academy"), ("lab", "laboratory"),
];

const STOP: [&str; 10] = ["of", "the", "and", "de", "di", "del", "la", "le", "for", "und"];

/// Matching key: lowercase words, abbreviations expanded, small words dropped.
pub fn key(name: &str) -> String {
    let lower = name.to_lowercase();
    let words = lower.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty() && !STOP.contains(w));
    let full: Vec<&str> = words.map(|w| ABBREVIATIONS.iter().find(|(a, _)| *a == w).map_or(w, |(_, f)| *f)).collect();
    full.join(" ")
}

fn is_institution(segment: &str) -> bool {
    let lower = segment.to_lowercase();
    lower.split(|c: char| !c.is_alphanumeric()).any(|w| MARKERS.iter().any(|m| w.starts_with(m)))
}

fn make(name: &str, country: Option<String>) -> Option<Affiliation> {
    let name: String = name.trim().trim_end_matches('.').chars().filter(|c| !c.is_control()).take(MAX_NAME).collect();
    let key = key(&name);
    (!key.is_empty()).then_some(Affiliation { name, key, country })
}

/// The institution in an address. `lenient` also accepts a short marker-less name
/// (Crossref lists bare institution names such as "WHO").
pub fn from_text(raw: &str, lenient: bool) -> Option<Affiliation> {
    // WoS prefixes the authors in brackets: "[Rahman, M; Chen, L] Univ X, ...".
    let text = raw.rsplit_once(']').map_or(raw, |(_, rest)| rest);
    let text = text.split(';').next().unwrap_or("");
    let lower = text.to_lowercase();
    let cut = ["electronic address", "email:", "e-mail:"].iter().filter_map(|m| lower.find(m)).min().unwrap_or(text.len());
    let address = &text[..cut];
    let country = country_of(address).map(str::to_string);
    let segments: Vec<&str> = address.split(',').map(str::trim).filter(|s| !s.is_empty()).collect();
    let chosen = segments.iter().find(|s| is_institution(s)).copied();
    let chosen = chosen.or_else(|| (lenient && segments.len() == 1 && address.len() <= 80 && !address.chars().any(|c| c.is_ascii_digit())).then(|| segments[0]));
    make(chosen?, country)
}

/// An institution name given as such by an API (OpenAlex).
pub fn from_named(name: &str, country: Option<&str>) -> Option<Affiliation> {
    make(name, country.and_then(super::normalize_code))
}

/// Distinct institutions, first-seen order.
pub fn collect(items: impl IntoIterator<Item = Option<Affiliation>>) -> Vec<Affiliation> {
    let mut out: Vec<Affiliation> = Vec::new();
    for a in items.into_iter().flatten() {
        if !out.iter().any(|x| x.key == a.key) {
            out.push(a);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_the_institution_and_matches_abbreviations() {
        let wos = from_text("[Rahman, M] Univ Malaya, Dept Comp Sci, Kuala Lumpur 50603, Malaysia.", false).unwrap();
        let scopus = from_text("Department of Computer Science, University of Malaya, Kuala Lumpur, Malaysia", false).unwrap();
        assert_eq!((wos.name.as_str(), wos.country.as_deref()), ("Univ Malaya", Some("MY")));
        assert_eq!(scopus.name, "University of Malaya");
        assert_eq!(wos.key, scopus.key);
        let pubmed = from_text("Faculty of Medicine, Universitas Indonesia, Jakarta, Indonesia. Electronic address: a@ui.ac.id", false).unwrap();
        assert_eq!((pubmed.name.as_str(), pubmed.country.as_deref()), ("Universitas Indonesia", Some("ID")));
        assert_eq!(from_text("Dept Radiol, Seoul Natl Univ Hosp, Seoul, South Korea", false).unwrap().key, "seoul national university hospital");
        assert!(from_text("Department of Surgery, Jakarta, Indonesia", false).is_none(), "no institution named");
        assert_eq!(from_text("World Health Organization", true).unwrap().name, "World Health Organization");
        assert!(from_text("WHO", false).is_none());
        assert_eq!(from_text("WHO", true).unwrap().key, "who");
        assert_eq!(collect([from_named("MIT", Some("us")), from_named("mit", None)]).len(), 1);
    }
}
