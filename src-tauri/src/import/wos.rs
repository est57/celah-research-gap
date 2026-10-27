//! Web of Science "Plain text" (tagged) export: two-letter tags, continuation
//! lines indented by three spaces, each record closed by "ER".

use super::limits::{MAX_FIELD, MAX_RECORDS, MAX_REFS, MAX_REF_LEN, MAX_TITLE, MAX_ABSTRACT};
use crate::db::papers::NewPaper;
use crate::geo::{self, affil};
use crate::text::{authors_display, clean, clean_opt, join_list, normalize_doc_type, normalize_doi, split_list};
use std::collections::HashMap;

type Fields = HashMap<String, Vec<String>>;

fn tag_of(line: &str) -> Option<&str> {
    let b = line.as_bytes();
    let is_tag = b.len() >= 2
        && b[0].is_ascii_uppercase()
        && (b[1].is_ascii_uppercase() || b[1].is_ascii_digit())
        && (b.len() == 2 || b[2] == b' ');
    is_tag.then(|| &line[..2])
}

pub fn parse(text: &str) -> Vec<NewPaper> {
    let mut out = Vec::new();
    let mut fields = Fields::new();
    let mut last = String::new();
    for line in text.lines() {
        if line.trim_end() == "ER" {
            out.extend(to_paper(&fields));
            fields.clear();
            last.clear();
            if out.len() >= MAX_RECORDS {
                break;
            }
        } else if let Some(tag) = tag_of(line) {
            let value = line.get(3..).unwrap_or("").trim().to_string();
            fields.entry(tag.to_string()).or_default().push(value);
            last = tag.to_string();
        } else if line.starts_with("   ") && !last.is_empty() {
            if let Some(v) = fields.get_mut(&last) {
                v.push(line.trim().to_string());
            }
        }
    }
    out
}

fn text(f: &Fields, tag: &str, max: usize) -> Option<String> {
    f.get(tag).and_then(|v| clean_opt(Some(&v.join(" ")), max))
}

fn lines(f: &Fields, tag: &str, max_items: usize, max_len: usize) -> Vec<String> {
    f.get(tag)
        .map(|v| v.iter().map(|s| clean(s, max_len)).filter(|s| !s.is_empty()).take(max_items).collect())
        .unwrap_or_default()
}

fn keywords(f: &Fields, tag: &str) -> Option<String> {
    let raw = text(f, tag, 5_000)?;
    join_list(&split_list(&raw, ';', 100, 80), MAX_FIELD)
}

fn to_paper(f: &Fields) -> Option<NewPaper> {
    let title = text(f, "TI", MAX_TITLE)?;
    let doi = text(f, "DI", 300).and_then(|d| normalize_doi(&d));
    let ext_id = match (text(f, "UT", 64), &doi) {
        (Some(ut), _) => format!("wos:{ut}"),
        (None, Some(d)) => format!("doi:{d}"),
        (None, None) => return None,
    };
    let names = lines(f, "AU", 2_000, 120);
    let refs = lines(f, "CR", MAX_REFS, MAX_REF_LEN);
    let countries = geo::countries_from(f.get("C1").into_iter().flatten().map(String::as_str));
    let affiliations = affil::collect(f.get("C1").into_iter().flatten().map(|a| affil::from_text(a, false)));
    let number = |tag: &str| text(f, tag, 12).and_then(|v| v.parse::<i64>().ok());
    Some(NewPaper {
        ext_id,
        doi,
        title,
        year: number("PY").map(|y| y as i32).filter(|y| (1500..=2100).contains(y)),
        source_name: text(f, "SO", MAX_FIELD),
        doc_type: text(f, "DT", 80).and_then(|t| normalize_doc_type(&t)),
        cited_by: number("TC").or_else(|| number("Z9")).unwrap_or(0).max(0),
        authors: authors_display(&names),
        author_count: names.len() as i64,
        author_names: names.clone(),
        has_affiliation: f.get("C1").is_some_and(|v| !v.is_empty()),
        corresponding_country: text(f, "RP", 2_000)
            .and_then(|rp| geo::country_of(&rp).map(str::to_string))
            .or_else(|| countries.first().cloned()),
        countries,
        abstract_text: text(f, "AB", MAX_ABSTRACT),
        keywords: keywords(f, "DE"),
        keywords_plus: keywords(f, "ID"),
        ref_count: number("NR").unwrap_or(refs.len() as i64).max(0),
        cites: Vec::new(),
        affiliations,
        refs,
    })
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "FN Clarivate Analytics Web of Science\nVR 1.0\nPT J\nAU Rahman, M\n   Chen, L\n\
TI Deep learning for TB screening on chest
   radiographs
SO INTERNATIONAL JOURNAL OF TUBERCULOSIS AND LUNG DISEASE\nDT Article\n\
DE deep learning; tuberculosis\nID CLASSIFICATION; DIAGNOSIS\nC1 [Rahman, M] Univ X, Dhaka, Bangladesh.\n   [Chen, L] Stanford Univ, Stanford, CA 94305 USA.\n\
RP Chen, L (corresponding author), Stanford Univ, Stanford, CA 94305 USA.\n\
CR Chen L, 2019, J DIGIT IMAGING, V32, P1\n   Lakhani P, 2017, RADIOLOGY, V284, P574\nNR 2\nTC 12\n\
PY 2023\nDI 10.5588/IJTLD.23.0001\nUT WOS:000123456700001\nER\n\nPT J\nAU X\nER\nEF\n";

    #[test]
    fn parses_tagged_records() {
        let papers = super::parse(SAMPLE);
        assert_eq!(papers.len(), 1, "record without title or id is skipped");
        let p = &papers[0];
        assert_eq!(p.ext_id, "wos:WOS:000123456700001");
        assert_eq!(p.title, "Deep learning for TB screening on chest radiographs");
        assert_eq!(p.authors, "Rahman, M; Chen, L");
        assert_eq!(p.refs.len(), 2);
        assert_eq!(p.cited_by, 12);
        assert_eq!(p.year, Some(2023));
        assert_eq!(p.doi.as_deref(), Some("10.5588/ijtld.23.0001"));
        assert_eq!(p.keywords_plus.as_deref(), Some("CLASSIFICATION; DIAGNOSIS"));
        assert!(p.has_affiliation);
        assert_eq!(p.countries, vec!["BD", "US"]);
        assert_eq!(p.corresponding_country.as_deref(), Some("US"));
    }
}
