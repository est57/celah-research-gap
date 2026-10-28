//! Three-field plot (Sankey of three fields, as in bibliometrix). Each column can show
//! authors, keywords, sources, countries or cited references; links count documents
//! that have both items.

mod fields;
#[cfg(test)]
mod tests;

pub use fields::Field;
use super::cooccur;
use crate::error::{AppError, AppResult};
use fields::Items;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Node {
    pub label: String,
    pub count: i64,
}

#[derive(Serialize)]
pub struct Link {
    pub from: usize,
    pub to: usize,
    pub value: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeField {
    /// Column headings, left to right.
    pub labels: [&'static str; 3],
    /// "countries" when a column holds ISO codes the UI shows as country names.
    pub kinds: [&'static str; 3],
    pub left: Vec<Node>,
    pub middle: Vec<Node>,
    pub right: Vec<Node>,
    pub left_links: Vec<Link>,
    pub right_links: Vec<Link>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThreeFieldParams {
    pub left: String,
    pub middle: String,
    pub right: String,
    pub items: u32,
}

/// Top `n` items by documents (ties alphabetical): nodes and key -> index.
fn top(items: &Items, n: usize) -> (Vec<Node>, HashMap<String, usize>) {
    let mut count: HashMap<&str, (i64, &str)> = HashMap::new();
    for list in items.values() {
        for (k, l) in list {
            count.entry(k.as_str()).or_insert((0, l.as_str())).0 += 1;
        }
    }
    let mut ranked: Vec<(&str, (i64, &str))> = count.into_iter().collect();
    ranked.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(b.0)));
    ranked.truncate(n);
    let index = ranked.iter().enumerate().map(|(i, (k, _))| (k.to_string(), i)).collect();
    (ranked.iter().map(|(_, (c, l))| Node { label: l.to_string(), count: *c }).collect(), index)
}

fn links(a: &Items, ai: &HashMap<String, usize>, b: &Items, bi: &HashMap<String, usize>) -> Vec<Link> {
    let mut map: HashMap<(usize, usize), i64> = HashMap::new();
    for (paper, left) in a {
        let Some(right) = b.get(paper) else { continue };
        for x in left.iter().filter_map(|(k, _)| ai.get(k)) {
            for y in right.iter().filter_map(|(k, _)| bi.get(k)) {
                *map.entry((*x, *y)).or_default() += 1;
            }
        }
    }
    let mut out: Vec<Link> = map.into_iter().map(|((from, to), value)| Link { from, to, value }).collect();
    out.sort_by_key(|l| (l.from, l.to));
    out
}

pub fn compute_with(conn: &Connection, cols: [Field; 3], n: usize) -> AppResult<ThreeField> {
    let items = [cols[0].items(conn)?, cols[1].items(conn)?, cols[2].items(conn)?];
    let (left, li) = top(&items[0], n);
    let (middle, mi) = top(&items[1], n);
    let (right, ri) = top(&items[2], n);
    let kind = |f: Field| if f == Field::Countries { "countries" } else { "" };
    Ok(ThreeField {
        labels: cols.map(Field::label),
        kinds: cols.map(kind),
        left_links: links(&items[0], &li, &items[1], &mi),
        right_links: links(&items[1], &mi, &items[2], &ri),
        left,
        middle,
        right,
    })
}

/// The overview default: authors, the better-filled keyword field, sources.
pub fn compute(conn: &Connection, n: usize) -> AppResult<ThreeField> {
    let kw = match cooccur::best_field(conn)? {
        cooccur::Field::AuthorKeywords => Field::AuthorKeywords,
        cooccur::Field::KeywordsPlus => Field::KeywordsPlus,
    };
    compute_with(conn, [Field::Authors, kw, Field::Sources], n)
}

pub fn compute_params(conn: &Connection, p: &ThreeFieldParams) -> AppResult<ThreeField> {
    let n = super::conceptual::in_range(p.items, 5, 30, crate::tr!("jumlah item", "number of items"))? as usize;
    let cols = [Field::parse(&p.left)?, Field::parse(&p.middle)?, Field::parse(&p.right)?];
    if cols[0] == cols[1] || cols[1] == cols[2] || cols[0] == cols[2] {
        return Err(AppError::Invalid(crate::tr!("tiap kolom harus memakai bidang yang berbeda", "each column must use a different field").into()));
    }
    compute_with(conn, cols, n)
}
