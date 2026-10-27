//! Author names in citation styles. Sources deliver "Family, Given" (Scopus, PubMed),
//! "Family G" (WoS) or "Given Family" (OpenAlex, Crossref), so the family name is
//! found first and the given names are reduced to initials.

fn is_initials(part: &str) -> bool {
    part.len() <= 3 && part.chars().all(|c| c.is_uppercase() || c == '.' || c == '-')
}

/// (family, given) for the formats above. Given may be empty.
pub fn split(name: &str) -> (String, String) {
    let name = name.trim();
    if let Some((family, given)) = name.split_once(',') {
        return (family.trim().to_string(), given.trim().to_string());
    }
    let parts: Vec<&str> = name.split_whitespace().collect();
    match parts.as_slice() {
        [] => (String::new(), String::new()),
        [only] => ((*only).to_string(), String::new()),
        [family, rest @ ..] if rest.iter().all(|p| is_initials(p)) => ((*family).to_string(), rest.join(" ")),
        [given @ .., family] => ((*family).to_string(), given.join(" ")),
    }
}

pub fn surname(name: &str) -> String {
    split(name).0
}

/// "Sunil Lalchand" -> "S. L.", "Jean-Paul" -> "J.-P.", "ZZ" -> "Z. Z."
pub fn initials(given: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    for part in given.split_whitespace() {
        let clean: String = part.chars().filter(|c| c.is_alphabetic() || *c == '-').collect();
        if clean.chars().all(|c| c.is_uppercase() || c == '-') && clean.len() <= 3 {
            // Already initials, as in WoS "ZZ".
            out.extend(clean.chars().filter(|c| c.is_alphabetic()).map(|c| format!("{c}.")));
        } else {
            let hyphenated: Vec<String> = clean.split('-').filter_map(|s| s.chars().next()).map(|c| format!("{}.", c.to_uppercase())).collect();
            out.push(hyphenated.join("-"));
        }
    }
    out.retain(|s| !s.is_empty());
    out.join(" ")
}

/// APA: "Bajeja, S. L."
pub fn apa(name: &str) -> String {
    let (family, given) = split(name);
    let init = initials(&given);
    if init.is_empty() { family } else { format!("{family}, {init}") }
}

/// IEEE: "S. L. Bajeja"
pub fn ieee(name: &str) -> String {
    let (family, given) = split(name);
    let init = initials(&given);
    if init.is_empty() { family } else { format!("{init} {family}") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_names_from_every_source_style() {
        assert_eq!(apa("Sunil Lalchand Bajeja"), "Bajeja, S. L.");
        assert_eq!(apa("Rahman, M."), "Rahman, M.");
        assert_eq!(apa("Chen L"), "Chen, L.");
        assert_eq!(apa("Qin ZZ"), "Qin, Z. Z.");
        assert_eq!(apa("Javier Garcia-Blas"), "Garcia-Blas, J.");
        assert_eq!(ieee("Jean-Paul Sartre"), "J.-P. Sartre");
        assert_eq!(ieee("Okafor, Chinedu"), "C. Okafor");
        assert_eq!(apa("WHO"), "WHO");
        assert_eq!(surname("Ahmed Iqbal"), "Iqbal");
    }
}
