//! Country production and collaboration: SCP/MCP by corresponding author,
//! papers per country, country pairs and the collaboration network.

use super::network::{self, Network};
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

const TOP_COUNTRIES: usize = 20;
const TOP_PAIRS: usize = 15;
const NETWORK_NODES: usize = 30;

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CountryProd {
    pub code: String,
    pub articles: i64,
    /// Single-country publications.
    pub scp: i64,
    /// Multiple-country publications.
    pub mcp: i64,
}

#[derive(Serialize)]
pub struct CountryCount {
    pub code: String,
    pub count: i64,
}

#[derive(Serialize)]
pub struct Pair {
    pub a: String,
    pub b: String,
    pub count: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Countries {
    pub documents_with_country: usize,
    /// Share of those documents with authors from two or more countries.
    pub international_share: Option<f64>,
    pub corresponding: Vec<CountryProd>,
    pub production: Vec<CountryCount>,
    pub pairs: Vec<Pair>,
    pub network: Network,
}

struct PaperCountries {
    codes: Vec<String>,
    corresponding: Option<String>,
}

fn load(conn: &Connection) -> AppResult<Vec<PaperCountries>> {
    let mut stmt = conn.prepare("SELECT paper_id, code, corresponding FROM paper_countries ORDER BY paper_id, code")?;
    let mut by_paper: BTreeMap<i64, PaperCountries> = BTreeMap::new();
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        let (id, code, corr): (i64, String, bool) = (r.get(0)?, r.get(1)?, r.get(2)?);
        let p = by_paper.entry(id).or_insert(PaperCountries { codes: Vec::new(), corresponding: None });
        if corr {
            p.corresponding = Some(code.clone());
        }
        p.codes.push(code);
    }
    Ok(by_paper.into_values().collect())
}

fn top<T>(mut items: Vec<T>, n: usize, key: impl Fn(&T) -> (i64, String)) -> Vec<T> {
    items.sort_by(|a, b| {
        let (ca, la) = key(a);
        let (cb, lb) = key(b);
        cb.cmp(&ca).then(la.cmp(&lb))
    });
    items.truncate(n);
    items
}

pub fn compute(conn: &Connection) -> AppResult<Countries> {
    let papers = load(conn)?;
    let mut corr: HashMap<String, CountryProd> = HashMap::new();
    let mut prod: HashMap<String, i64> = HashMap::new();
    let mut pairs: HashMap<(String, String), i64> = HashMap::new();
    let mut international = 0;
    for p in &papers {
        let multi = p.codes.len() > 1;
        international += i64::from(multi);
        if let Some(c) = &p.corresponding {
            let e = corr.entry(c.clone()).or_insert_with(|| CountryProd { code: c.clone(), ..Default::default() });
            e.articles += 1;
            if multi { e.mcp += 1 } else { e.scp += 1 }
        }
        for (i, a) in p.codes.iter().enumerate() {
            *prod.entry(a.clone()).or_default() += 1;
            for b in &p.codes[i + 1..] {
                *pairs.entry((a.clone(), b.clone())).or_default() += 1;
            }
        }
    }
    let docs: Vec<Vec<String>> = papers.iter().map(|p| p.codes.clone()).collect();
    let labels: HashMap<String, String> = prod.keys().map(|c| (c.clone(), c.clone())).collect();
    Ok(Countries {
        documents_with_country: papers.len(),
        international_share: (!papers.is_empty()).then(|| international as f64 / papers.len() as f64),
        corresponding: top(corr.into_values().collect(), TOP_COUNTRIES, |c| (c.articles, c.code.clone())),
        production: top(prod.into_iter().map(|(code, count)| CountryCount { code, count }).collect(), TOP_COUNTRIES, |c| (c.count, c.code.clone())),
        pairs: top(pairs.into_iter().map(|((a, b), count)| Pair { a, b, count }).collect(), TOP_PAIRS, |p| (p.count, format!("{}{}", p.a, p.b))),
        network: network::build(&docs, &labels, NETWORK_NODES),
    })
}

#[cfg(test)]
mod tests {
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    fn paper(id: usize, countries: &[&str], corr: &str) -> NewPaper {
        NewPaper {
            ext_id: format!("p{id}"),
            title: format!("Paper {id}"),
            countries: countries.iter().map(|c| c.to_string()).collect(),
            corresponding_country: Some(corr.into()),
            ..Default::default()
        }
    }

    #[test]
    fn splits_scp_and_mcp() {
        let mut conn = open_memory();
        let items = vec![paper(1, &["IN"], "IN"), paper(2, &["IN", "US"], "IN"), paper(3, &["US"], "GB"), paper(4, &["ID", "NL"], "ID")];
        insert_many(&mut conn, &items, "Test").unwrap();
        let c = super::compute(&conn).unwrap();
        let india = c.corresponding.iter().find(|x| x.code == "IN").unwrap();
        assert_eq!((india.articles, india.scp, india.mcp), (2, 1, 1));
        let uk = c.corresponding.iter().find(|x| x.code == "GB").unwrap();
        assert_eq!(uk.mcp, 1, "corresponding country outside the affiliation list still counts, as MCP");
        assert_eq!(c.documents_with_country, 4);
        assert_eq!(c.international_share, Some(0.75));
        assert!(c.pairs.iter().any(|p| p.a == "IN" && p.b == "US" && p.count == 1));
    }
}
