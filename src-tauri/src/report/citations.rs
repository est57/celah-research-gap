//! In-text citation groups, numbered by first citation for IEEE.

use super::cite::{self, Style, Work};
use crate::analysis::gaps::model::Gap;
use std::collections::HashMap;

/// Keeps cited works in order of first citation (IEEE numbering).
pub struct Citations<'a> {
    pub style: Style,
    pub works: &'a HashMap<i64, Work>,
    pub order: Vec<i64>,
}

impl Citations<'_> {
    /// Narrative citation: "Rahman et al. (2023)" in APA, "[2]" in IEEE.
    pub fn narrative(&mut self, id: i64) -> String {
        let group = self.group(&[id]);
        match (self.style, self.works.get(&id)) {
            (Style::Apa, Some(w)) => {
                let text = cite::apa_in_text(w);
                match text.rsplit_once(", ") {
                    Some((who, year)) => format!("{who} ({year})"),
                    None => group,
                }
            }
            _ => group,
        }
    }

    pub fn group(&mut self, ids: &[i64]) -> String {
        let ids: Vec<i64> = ids.iter().copied().filter(|id| self.works.contains_key(id)).collect();
        if ids.is_empty() {
            return String::new();
        }
        for id in &ids {
            if !self.order.contains(id) {
                self.order.push(*id);
            }
        }
        match self.style {
            Style::Apa => format!("({})", ids.iter().map(|id| cite::apa_in_text(&self.works[id])).collect::<Vec<_>>().join("; ")),
            Style::Ieee => ids.iter().map(|id| format!("[{}]", self.order.iter().position(|o| o == id).unwrap_or(0) + 1)).collect::<Vec<_>>().join(", "),
        }
    }
}

pub fn evidence_ids(g: &Gap, near: bool, n: usize) -> Vec<i64> {
    g.evidence.iter().filter(|e| (e.role == "hampir menjawab") == near).take(n).map(|e| e.paper_id).collect()
}


/// Reference list entries in citation order (IEEE) or by first author and year (APA),
/// and the cited works for BibTeX and RIS.
pub fn references(cites: &Citations) -> (Vec<String>, Vec<Work>) {
    let mut cited: Vec<&Work> = cites.order.iter().filter_map(|id| cites.works.get(id)).collect();
    if cites.style == Style::Apa {
        cited.sort_by_key(|w| (w.authors.first().map(|a| cite::surname(a).to_lowercase()).unwrap_or_default(), w.year));
    }
    let list = cited
        .iter()
        .enumerate()
        .map(|(i, w)| match cites.style {
            Style::Apa => cite::reference(w, cites.style),
            Style::Ieee => format!("[{}] {}", i + 1, cite::reference(w, cites.style)),
        })
        .collect();
    (list, cited.into_iter().cloned().collect())
}
