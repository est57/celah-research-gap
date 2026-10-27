//! Country names for the draft text, from ISO alpha-2 codes.

use crate::geo::names::NAMES;

/// Indonesian names that differ from the English ones, for the most frequent countries.
const INDONESIAN: &[(&str, &str)] = &[
    ("US", "Amerika Serikat"), ("GB", "Inggris Raya"), ("CN", "Tiongkok"), ("KR", "Korea Selatan"), ("JP", "Jepang"),
    ("DE", "Jerman"), ("FR", "Prancis"), ("NL", "Belanda"), ("SA", "Arab Saudi"), ("ZA", "Afrika Selatan"),
    ("BR", "Brasil"), ("ES", "Spanyol"), ("IT", "Italia"), ("CH", "Swiss"), ("SE", "Swedia"), ("TR", "Turki"),
    ("EG", "Mesir"), ("AE", "Uni Emirat Arab"), ("NZ", "Selandia Baru"), ("PH", "Filipina"), ("SG", "Singapura"),
    ("BE", "Belgia"), ("DK", "Denmark"), ("NO", "Norwegia"), ("FI", "Finlandia"), ("PL", "Polandia"), ("MX", "Meksiko"),
];

const SMALL: [&str; 4] = ["and", "of", "the", "da"];

/// "united states" -> "United States"; small joining words stay lowercase.
fn title(name: &str) -> String {
    name.split(' ')
        .enumerate()
        .map(|(i, w)| {
            if i > 0 && SMALL.contains(&w) {
                return w.to_string();
            }
            let mut c = w.chars();
            c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn name(code: &str, indonesian: bool) -> String {
    if indonesian {
        if let Some((_, n)) = INDONESIAN.iter().find(|(c, _)| *c == code) {
            return (*n).to_string();
        }
    }
    NAMES.iter().find(|(_, c)| *c == code).map(|(n, _)| title(n)).unwrap_or_else(|| code.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn names_countries_in_both_languages() {
        assert_eq!(super::name("US", false), "United States");
        assert_eq!(super::name("US", true), "Amerika Serikat");
        assert_eq!(super::name("ID", true), "Indonesia");
        assert_eq!(super::name("BA", false), "Bosnia and Herzegovina");
        assert_eq!(super::name("XX", true), "XX");
    }
}
