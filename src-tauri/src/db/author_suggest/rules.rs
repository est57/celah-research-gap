//! When two author names may belong to one person, and the evidence for it.

use super::{shared, initials_of, Names, Suggestion, Variant};
use crate::report::names::split;
use std::collections::{HashMap, HashSet};

pub struct Person {
    pub key: String,
    pub initials: String,
    /// First given name when written out ("yanyan"), None for initials only.
    pub first: Option<String>,
}

pub fn full_first(given: &str) -> Option<String> {
    let first = given.split_whitespace().next()?;
    let letters: String = first.chars().filter(|c| c.is_alphabetic()).flat_map(char::to_lowercase).collect();
    let is_initials = first.chars().filter(|c| c.is_alphabetic()).all(char::is_uppercase) && letters.len() <= 3;
    (letters.len() >= 2 && !is_initials).then_some(letters)
}

/// Same person is possible only if the names never share a paper, one set of initials
/// extends the other, and written-out first names agree.
pub fn compatible(a: &Person, b: &Person, pa: &HashSet<i64>, pb: &HashSet<i64>) -> bool {
    let initials_ok = a.initials.starts_with(&b.initials) || b.initials.starts_with(&a.initials);
    let first_ok = match (&a.first, &b.first) {
        (Some(x), Some(y)) => x == y,
        _ => true,
    };
    initials_ok && first_ok && pa.is_disjoint(pb)
}

/// "Antani, Sameer" and "Sameer Antani" -> "antani s", so co-authors match across name orders.
fn canonical(name: &str) -> String {
    let (family, given) = split(name);
    let fam: String = family.chars().filter(|c| c.is_alphabetic()).flat_map(char::to_lowercase).collect();
    format!("{fam} {}", initials_of(&given).chars().next().map(String::from).unwrap_or_default())
}

/// Lowercase words, sorted: equal for the same name in any order.
fn word_set(name: &str) -> Vec<String> {
    let lower = name.to_lowercase();
    let mut w: Vec<String> = lower.split(|c: char| !c.is_alphabetic()).filter(|x| !x.is_empty()).map(str::to_string).collect();
    w.sort();
    w
}

pub fn suggestion(chosen: &[Person], names: &Names, by_paper: &HashMap<i64, Vec<String>>) -> Suggestion {
    let keys: Vec<&String> = chosen.iter().map(|p| &p.key).collect();
    let own: HashSet<String> = keys.iter().map(|k| canonical(&names[*k].0)).collect();
    let coauthors: Vec<HashSet<String>> = keys
        .iter()
        .map(|k| {
            let others = names[*k].1.iter().flat_map(|p| by_paper[p].iter()).filter(|c| !keys.contains(c));
            others.map(|c| canonical(&names[c].0)).filter(|c| !own.contains(c)).collect()
        })
        .collect();
    let words: Vec<Vec<String>> = keys.iter().map(|k| word_set(&names[*k].0)).collect();
    let variants = keys.iter().map(|k| Variant { key: (*k).clone(), name: names[*k].0.clone(), documents: names[*k].1.len() }).collect();
    Suggestion {
        shared_coauthors: shared(&coauthors.iter().collect::<Vec<_>>()),
        shared_sources: shared(&keys.iter().map(|k| &names[*k].2).collect::<Vec<_>>()),
        reordered: words.windows(2).all(|w| w[0] == w[1]),
        variants,
    }
}
