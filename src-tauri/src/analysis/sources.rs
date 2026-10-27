//! Most relevant sources and Bradford's law of scattering.

use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;

/// Sources shown individually on the Bradford chart.
const BRADFORD_SHOWN: usize = 40;

#[derive(Serialize, Clone)]
pub struct NamedCount {
    pub label: String,
    pub count: i64,
}

#[derive(Serialize)]
pub struct BradfordItem {
    pub rank: usize,
    pub label: String,
    pub count: i64,
    /// 1 = core zone.
    pub zone: u8,
}

#[derive(Serialize, Default, Clone, Copy)]
pub struct ZoneSummary {
    pub sources: usize,
    pub documents: i64,
}

#[derive(Serialize)]
pub struct Bradford {
    pub ranked: Vec<BradfordItem>,
    pub zones: [ZoneSummary; 3],
}

/// Every source with its document count, most productive first.
pub fn ranked_sources(conn: &Connection) -> AppResult<Vec<NamedCount>> {
    let mut stmt = conn.prepare(
        "SELECT MIN(trim(source_name)), COUNT(*) FROM papers
         WHERE source_name IS NOT NULL AND trim(source_name) <> ''
         GROUP BY lower(trim(source_name)) ORDER BY 2 DESC, 1 ASC",
    )?;
    let rows = stmt
        .query_map([], |r| Ok(NamedCount { label: r.get(0)?, count: r.get(1)? }))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

/// Bradford zone of every ranked source, as (label, zone).
pub fn bradford_all(ranked: &[NamedCount]) -> Vec<(String, u8)> {
    let total: i64 = ranked.iter().map(|s| s.count).sum();
    let mut cumulative = 0;
    ranked
        .iter()
        .enumerate()
        .map(|(i, s)| {
            // Same cut points as bibliometrix (cumulative <= n/3, <= 2n/3), except the
            // most productive source always counts as core.
            cumulative += s.count;
            let zone = if i == 0 || cumulative * 3 <= total {
                1
            } else if cumulative * 3 <= total * 2 {
                2
            } else {
                3
            };
            (s.label.clone(), zone)
        })
        .collect()
}

/// Splits ranked sources into three zones holding about a third of the documents each.
pub fn bradford(ranked: &[NamedCount]) -> Bradford {
    let mut zones = [ZoneSummary::default(); 3];
    let mut items = Vec::new();
    for (i, (s, (_, zone))) in ranked.iter().zip(bradford_all(ranked)).enumerate() {
        let z = &mut zones[usize::from(zone - 1)];
        z.sources += 1;
        z.documents += s.count;
        if i < BRADFORD_SHOWN {
            items.push(BradfordItem { rank: i + 1, label: s.label.clone(), count: s.count, zone });
        }
    }
    Bradford { ranked: items, zones }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_documents_into_thirds() {
        let counts = [30, 20, 10, 10, 10, 5, 5, 5, 5];
        let ranked: Vec<NamedCount> = counts.iter().enumerate().map(|(i, c)| NamedCount { label: format!("S{i}"), count: *c }).collect();
        let b = bradford(&ranked);
        assert_eq!(b.zones[0].sources, 1, "the first source alone covers a third of 100 documents");
        assert_eq!(b.zones[1].sources, 2);
        assert_eq!(b.zones[2].sources, 6);
        assert_eq!(b.zones.iter().map(|z| z.documents).sum::<i64>(), 100);
        assert_eq!(b.ranked[0].zone, 1);
    }
}
