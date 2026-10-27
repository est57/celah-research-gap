//! Links between themes of consecutive periods by the keywords they share: the
//! inclusion index weighted by word occurrences (Cobo et al. 2011, as in bibliometrix),
//! sum over shared words of min(f_a, f_b) / min(total f_A, total f_B). A rare keyword
//! that happens to recur barely counts, so incidental links stay out.

use super::super::cooccur::Net;
use super::super::thematic::Theme;
use serde::Serialize;
use std::collections::HashMap;

/// Links weaker than this are noise from incidental shared keywords.
const MIN_INCLUSION: f64 = 0.1;
const SHARED_SHOWN: usize = 5;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeLink {
    /// Index of the earlier period; the link goes to period + 1.
    pub period: usize,
    pub from: usize,
    pub to: usize,
    pub inclusion: f64,
    /// Occurrences of the shared keywords in the earlier and the later theme.
    pub out_value: i64,
    pub in_value: i64,
    pub shared: Vec<String>,
}

pub fn between(period: usize, a: (&Net, &[Theme]), b: (&Net, &[Theme])) -> Vec<ThemeLink> {
    let (net_a, themes_a) = a;
    let (net_b, themes_b) = b;
    let mut out = Vec::new();
    for ta in themes_a {
        let keys: HashMap<&str, usize> = ta.members.iter().map(|&w| (net_a.keys[w].as_str(), w)).collect();
        for tb in themes_b {
            let shared: Vec<(usize, usize)> = tb.members.iter().filter_map(|&w| keys.get(net_b.keys[w].as_str()).map(|&v| (v, w))).collect();
            let smaller = ta.frequency.min(tb.frequency);
            if shared.is_empty() || smaller == 0 {
                continue;
            }
            let overlap: i64 = shared.iter().map(|&(v, w)| net_a.freq[v].min(net_b.freq[w])).sum();
            let inclusion = overlap as f64 / smaller as f64;
            if inclusion < MIN_INCLUSION {
                continue;
            }
            let mut ranked = shared.clone();
            ranked.sort_by(|x, y| (net_a.freq[y.0] + net_b.freq[y.1]).cmp(&(net_a.freq[x.0] + net_b.freq[x.1])));
            out.push(ThemeLink {
                period,
                from: ta.id,
                to: tb.id,
                inclusion,
                out_value: shared.iter().map(|&(v, _)| net_a.freq[v]).sum(),
                in_value: shared.iter().map(|&(_, w)| net_b.freq[w]).sum(),
                shared: ranked.iter().take(SHARED_SHOWN).map(|&(_, w)| net_b.labels[w].clone()).collect(),
            });
        }
    }
    out
}
