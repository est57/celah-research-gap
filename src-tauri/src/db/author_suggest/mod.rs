//! Author names that may be one person: same family name and compatible initials
//! ("Rahman M", "Rahman MA", "Muhammad Rahman"), with shared co-authors and sources as
//! evidence. Different people with similar names usually share neither.

use crate::error::AppResult;
mod rules;
#[cfg(test)]
mod tests;

use crate::report::names::{initials, split};
use rules::{compatible, full_first, suggestion, Person};
use rusqlite::Connection;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

const MAX_GROUP: usize = 8;
const MAX_SUGGESTIONS: usize = 60;

#[derive(Serialize, Clone)]
pub struct Variant {
    pub key: String,
    pub name: String,
    pub documents: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Suggestion {
    pub variants: Vec<Variant>,
    /// Co-authors and sources that at least two variants have in common.
    pub shared_coauthors: usize,
    pub shared_sources: usize,
    /// Every variant has the same words in a different order ("Antani, Sameer" and
    /// "Sameer Antani"): almost certainly one person written by two sources.
    pub reordered: bool,
}

/// "M. A." -> "ma": initials only, lowercase.
pub(super) fn initials_of(given: &str) -> String {
    initials(given).chars().filter(|c| c.is_alphabetic()).flat_map(char::to_lowercase).collect()
}

/// Ids shared by at least two of the sets.
pub(super) fn shared(sets: &[&HashSet<String>]) -> usize {
    let mut seen: HashMap<&String, usize> = HashMap::new();
    for s in sets {
        for x in s.iter() {
            *seen.entry(x).or_default() += 1;
        }
    }
    seen.values().filter(|&&n| n >= 2).count()
}

/// Name, papers and sources per name key.
pub type Names = HashMap<String, (String, HashSet<i64>, HashSet<String>)>;

pub fn suggest(conn: &Connection) -> AppResult<Vec<Suggestion>> {
    let mut stmt = conn.prepare(
        "SELECT a.name_key, a.name, a.paper_id, lower(trim(COALESCE(p.source_name, ''))) FROM paper_authors a JOIN papers p ON p.id = a.paper_id",
    )?;
    let mut names: Names = HashMap::new();
    let mut by_paper: HashMap<i64, Vec<String>> = HashMap::new();
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let (key, name, paper, source): (String, String, i64, String) = (r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?);
        let e = names.entry(key.clone()).or_insert_with(|| (name, HashSet::new(), HashSet::new()));
        e.1.insert(paper);
        if !source.is_empty() {
            e.2.insert(source);
        }
        by_paper.entry(paper).or_default().push(key);
    }
    // Group by family name + first initial, then pick clusters of mutually compatible names.
    let mut groups: HashMap<(String, char), Vec<Person>> = HashMap::new();
    for (key, (name, _, _)) in &names {
        let (family, given) = split(name);
        let ini = initials_of(&given);
        let fam: String = family.chars().filter(|c| c.is_alphabetic()).flat_map(char::to_lowercase).collect();
        if let (false, Some(first)) = (fam.is_empty(), ini.chars().next()) {
            groups.entry((fam, first)).or_default().push(Person { key: key.clone(), initials: ini, first: full_first(&given) });
        }
    }
    let mut out = Vec::new();
    for mut members in groups.into_values().filter(|m| m.len() >= 2) {
        members.sort_by(|a, b| names[&b.key].1.len().cmp(&names[&a.key].1.len()).then(a.key.cmp(&b.key)));
        while members.len() >= 2 {
            let mut chosen: Vec<Person> = vec![members.remove(0)];
            let mut rest = Vec::new();
            for m in members.drain(..) {
                let ok = chosen.len() < MAX_GROUP && chosen.iter().all(|c| compatible(c, &m, &names[&c.key].1, &names[&m.key].1));
                if ok { chosen.push(m) } else { rest.push(m) }
            }
            members = rest;
            if chosen.len() >= 2 {
                out.push(suggestion(&chosen, &names, &by_paper));
            }
        }
    }
    // Strongest evidence first: same words reordered, then shared co-authors and sources.
    let strength = |g: &Suggestion| (usize::from(g.reordered) * 3 + g.shared_coauthors + g.shared_sources, g.variants.iter().map(|v| v.documents).sum::<usize>());
    out.sort_by(|a, b| strength(b).cmp(&strength(a)).then(a.variants[0].name.cmp(&b.variants[0].name)));
    out.truncate(MAX_SUGGESTIONS);
    Ok(out)
}
