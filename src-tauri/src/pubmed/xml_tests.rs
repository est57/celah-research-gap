const SAMPLE: &str = r#"<?xml version="1.0" ?>
<!DOCTYPE PubmedArticleSet PUBLIC "-//NLM//DTD PubMedArticle, 1st January 2025//EN" "https://dtd.nlm.nih.gov/ncbi/pubmed/out/pubmed_250101.dtd">
<PubmedArticleSet><PubmedArticle><MedlineCitation Status="MEDLINE"><PMID Version="1">42902181</PMID>
<DateCompleted><Year>2026</Year></DateCompleted>
<Article><Journal><JournalIssue><PubDate><Year>2025</Year><Month>Jul</Month></PubDate></JournalIssue>
<Title>BMC infectious diseases</Title></Journal>
<ArticleTitle>Explainable <i>AI</i> in TB diagnosis.</ArticleTitle>
<ELocationID EIdType="doi" ValidYN="Y">10.1186/S12879-026-1</ELocationID>
<Abstract><AbstractText Label="BACKGROUND">Deep learning &amp; trust.</AbstractText><AbstractText>Second part.</AbstractText></Abstract>
<AuthorList><Author><LastName>Valishirin</LastName><ForeName>Hossein</ForeName>
<AffiliationInfo><Affiliation>Washington State University, Pullman, WA, 99164, USA. Electronic address: x@wsu.edu</Affiliation></AffiliationInfo></Author>
<Author><CollectiveName>TB AI Consortium</CollectiveName></Author></AuthorList>
<PublicationTypeList><PublicationType>Journal Article</PublicationType><PublicationType>Review</PublicationType></PublicationTypeList>
</Article><MeshHeadingList><MeshHeading><DescriptorName>Tuberculosis</DescriptorName></MeshHeading></MeshHeadingList>
<KeywordList><Keyword>XAI</Keyword><Keyword>deep learning</Keyword></KeywordList></MedlineCitation>
<PubmedData><ReferenceList><Reference><Citation>Attai K, et al. Trop Med Infect Dis. 2024;9(9):216.</Citation></Reference></ReferenceList></PubmedData>
</PubmedArticle>
<PubmedArticle><MedlineCitation><PMID>abc</PMID><Article><ArticleTitle>Bad id</ArticleTitle></Article></MedlineCitation></PubmedArticle>
</PubmedArticleSet>"#;

#[test]
fn parses_pubmed_articles() {
    let papers = super::parse(SAMPLE);
    assert_eq!(papers.len(), 1, "record with a non-numeric PMID is skipped");
    let p = &papers[0];
    assert_eq!(p.ext_id, "pmid:42902181");
    assert_eq!(p.title, "Explainable AI in TB diagnosis.");
    assert_eq!(p.year, Some(2025), "uses PubDate, not DateCompleted");
    assert_eq!(p.doi.as_deref(), Some("10.1186/s12879-026-1"));
    assert_eq!(p.authors, "Valishirin, Hossein; TB AI Consortium");
    assert!(p.has_affiliation);
    assert_eq!(p.countries, vec!["US"]);
    assert_eq!(p.corresponding_country.as_deref(), Some("US"));
    assert_eq!(p.abstract_text.as_deref(), Some("Deep learning & trust. Second part."));
    assert_eq!(p.doc_type.as_deref(), Some("review"));
    assert_eq!(p.keywords_plus.as_deref(), Some("Tuberculosis"));
    assert_eq!(p.refs.len(), 1);
}

#[test]
fn rejects_malformed_xml() {
    assert!(super::parse("<PubmedArticleSet><oops").is_empty());
}
