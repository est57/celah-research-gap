//! PubMed "PubMed format" (MEDLINE) export: each record opens with "PMID- ", tags are
//! padded to four characters, wrapped lines are indented by six spaces.

use super::limits::{MAX_ABSTRACT, MAX_FIELD, MAX_RECORDS, MAX_TITLE};
use super::tagged::{self, first, keywords, list, values, Fields};
use crate::db::papers::NewPaper;
use crate::geo::{self, affil};
use crate::text::{authors_display, normalize_doi, normalize_issns};

pub fn parse(text: &str) -> Vec<NewPaper> {
    tagged::records(text, "PMID", "", MAX_RECORDS).iter().filter_map(to_paper).collect()
}

/// "10.1000/xyz [doi]" among the article ids (LID, AID).
fn doi(f: &Fields) -> Option<String> {
    values(f, &["LID", "AID"]).find_map(|v| v.strip_suffix("[doi]").and_then(|d| normalize_doi(d.trim())))
}

/// MeSH headings without the major-topic star and the subheadings: "*Tuberculosis/diagnosis" -> "Tuberculosis".
fn mesh(f: &Fields) -> Vec<&str> {
    values(f, &["MH"]).map(|m| m.split('/').next().unwrap_or("").trim_start_matches('*').trim()).collect()
}

fn doc_type(f: &Fields) -> Option<String> {
    let types: Vec<String> = values(f, &["PT"]).map(str::to_lowercase).collect();
    let has = |t: &str| types.iter().any(|x| x.contains(t));
    let t = if has("review") {
        "review"
    } else if has("preprint") {
        "preprint"
    } else if has("congress") {
        "proceedings"
    } else if types.is_empty() {
        return None;
    } else {
        "article"
    };
    Some(t.into())
}

fn to_paper(f: &Fields) -> Option<NewPaper> {
    let pmid = first(f, &["PMID"], 20).filter(|p| p.bytes().all(|c| c.is_ascii_digit()))?;
    let title = first(f, &["TI", "BTI"], MAX_TITLE)?;
    // Full names when given ("Rahman, Mohammad"), else the short form ("Rahman M").
    let mut names = list(f, &["FAU"], 2_000, 120);
    if names.is_empty() {
        names = list(f, &["AU"], 2_000, 120);
    }
    let addresses = list(f, &["AD"], 500, 1_000);
    let countries = geo::countries_from(addresses.iter().map(String::as_str));
    Some(NewPaper {
        ext_id: format!("pubmed:{pmid}"),
        doi: doi(f),
        title,
        year: values(f, &["DP", "DEP"]).find_map(tagged::year),
        source_name: first(f, &["JT", "TA"], MAX_FIELD),
        issn: normalize_issns(values(f, &["IS"])),
        doc_type: doc_type(f),
        cited_by: 0,
        authors: authors_display(&names),
        author_count: names.len() as i64,
        author_names: names.clone(),
        has_affiliation: !addresses.is_empty(),
        affiliations: affil::collect(addresses.iter().map(|a| affil::from_text(a, false))),
        corresponding_country: countries.first().cloned(),
        countries,
        abstract_text: first(f, &["AB"], MAX_ABSTRACT),
        keywords: keywords(values(f, &["OT"])),
        keywords_plus: keywords(mesh(f).into_iter()),
        // MEDLINE carries no reference lists; Crossref can fill them by DOI later.
        ref_count: 0,
        refs: Vec::new(),
        cites: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "PMID- 37000001\nOWN - NLM\nSTAT- MEDLINE\nIS  - 1027-3719 (Print)\nIS  - 1815-7920 (Electronic)\n\
DP  - 2023 Jan 1\nTI  - Deep learning for tuberculosis screening on chest\n      radiographs.\nAB  - We screen.\n\
LID - 10.5588/ijtld.23.0001 [doi]\nFAU - Rahman, Mohammad\nAU  - Rahman M\nAD  - Department of Medicine, University of Dhaka,\n\
      Dhaka, Bangladesh.\nFAU - Chen, Li\nAU  - Chen L\nAD  - Stanford University, Stanford, CA, USA.\nPT  - Journal Article\nPT  - Review\n\
JT  - The international journal of tuberculosis and lung disease\nTA  - Int J Tuberc Lung Dis\n\
MH  - Humans\nMH  - *Tuberculosis, Pulmonary/diagnostic imaging\nMH  - humans\nOT  - deep learning\nOT  - screening\n\n\
PMID- 37000002\nTI  - No title problems here\n\nPMID- bad\nTI  - Rejected id\n";

    #[test]
    fn reads_medline_records() {
        let papers = super::parse(SAMPLE);
        assert_eq!(papers.len(), 2, "a record with a malformed PMID is skipped");
        let p = &papers[0];
        assert_eq!(p.ext_id, "pubmed:37000001");
        assert_eq!(p.title, "Deep learning for tuberculosis screening on chest radiographs.");
        assert_eq!((p.year, p.doi.as_deref()), (Some(2023), Some("10.5588/ijtld.23.0001")));
        assert_eq!(p.authors, "Rahman, Mohammad; Chen, Li");
        assert_eq!(p.issn.as_deref(), Some("10273719; 18157920"));
        assert_eq!(p.doc_type.as_deref(), Some("review"));
        assert_eq!(p.keywords.as_deref(), Some("deep learning; screening"));
        assert_eq!(p.keywords_plus.as_deref(), Some("Humans; Tuberculosis, Pulmonary"));
        assert_eq!(p.countries, vec!["BD", "US"]);
        assert_eq!(p.source_name.as_deref(), Some("The international journal of tuberculosis and lung disease"));
    }
}
