//! Shared context of the gap detectors: the keyword network, citations and recency.

use super::evidence;
use crate::analysis::cooccur::Net;
use std::collections::HashMap;

const RECENT_YEARS: i32 = 3;

pub struct Ctx {
    pub net: Net,
    pub current_year: i32,
    cited: HashMap<i64, i64>,
    collection_recent: f64,
}

impl Ctx {
    pub fn new(net: Net, current_year: i32, cited: HashMap<i64, i64>) -> Ctx {
        let mut ctx = Ctx { net, current_year, cited, collection_recent: 0.0 };
        ctx.collection_recent = ctx.recent_share(&(0..ctx.net.years.len()).collect::<Vec<_>>());
        ctx
    }

    fn recent_share(&self, docs: &[usize]) -> f64 {
        let years: Vec<i32> = docs.iter().filter_map(|&d| self.net.years[d]).collect();
        self.recent_share_of(&years)
    }

    fn recent_share_of(&self, years: &[i32]) -> f64 {
        let recent = years.iter().filter(|&&y| y >= self.current_year - RECENT_YEARS && y < self.current_year).count();
        if years.is_empty() { 0.0 } else { recent as f64 / years.len() as f64 }
    }

    /// 0.5 means as recent as the whole collection; 1 means at least twice as recent.
    pub fn momentum(&self, docs: &[usize]) -> f64 {
        (self.recent_share(docs) / self.collection_recent.max(0.05)).min(2.0) / 2.0
    }

    /// As `momentum`, for documents given by their publication years.
    pub fn momentum_of_years(&self, years: &[i32]) -> f64 {
        (self.recent_share_of(years) / self.collection_recent.max(0.05)).min(2.0) / 2.0
    }

    /// Most cited documents among `docs`, as paper ids.
    pub fn top(&self, docs: &[usize], n: usize) -> Vec<i64> {
        evidence::top_docs(docs, &self.net.paper_ids, &self.net.years, &self.cited, n)
    }

    /// Newest documents among `docs`, as paper ids.
    pub fn newest(&self, docs: &[usize], n: usize) -> Vec<i64> {
        let mut list = docs.to_vec();
        list.sort_by(|&a, &b| self.net.years[b].cmp(&self.net.years[a]));
        list.into_iter().take(n).map(|d| self.net.paper_ids[d]).collect()
    }
}
