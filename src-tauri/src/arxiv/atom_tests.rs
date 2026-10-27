const FEED: &str = r#"<?xml version='1.0' encoding='UTF-8'?>
<feed xmlns:arxiv="http://arxiv.org/schemas/atom" xmlns="http://www.w3.org/2005/Atom">
  <entry>
    <id>http://arxiv.org/abs/2009.04160v1</id>
    <title>Revealing Lung Affections
      from CTs</title>
    <summary>Deep learning approaches for TB lesions.</summary>
    <category term="eess.IV" scheme="http://arxiv.org/schemas/atom"/>
    <category term="cs.CV" scheme="http://arxiv.org/schemas/atom"/>
    <published>2020-09-09T08:34:18Z</published>
    <arxiv:journal_ref>Working Notes of CLEF 2020</arxiv:journal_ref>
    <arxiv:doi>10.1000/XYZ</arxiv:doi>
    <author><name>Radu Miron</name><arxiv:affiliation>UAIC</arxiv:affiliation></author>
    <author><name>Cosmin Moisii</name></author>
  </entry>
  <entry>
    <id>http://arxiv.org/api/errors#incorrect_id_format</id>
    <title>Error</title>
  </entry>
</feed>"#;

#[test]
fn parses_atom_entries() {
    let (papers, seen) = super::parse(FEED);
    assert_eq!(seen, 2);
    assert_eq!(papers.len(), 1, "error entries are dropped");
    let p = &papers[0];
    assert_eq!(p.ext_id, "arxiv:2009.04160");
    assert_eq!(p.title, "Revealing Lung Affections from CTs");
    assert_eq!(p.year, Some(2020));
    assert_eq!(p.doi.as_deref(), Some("10.1000/xyz"));
    assert_eq!(p.source_name.as_deref(), Some("Working Notes of CLEF 2020"));
    assert_eq!(p.keywords_plus.as_deref(), Some("eess.IV; cs.CV"));
    assert_eq!(p.authors, "Radu Miron; Cosmin Moisii");
    assert!(p.has_affiliation);
}

#[test]
fn normalizes_ids() {
    use super::canonical_id;
    assert_eq!(canonical_id("http://arxiv.org/abs/hep-th/9901001v2").as_deref(), Some("hep-th/9901001"));
    assert_eq!(canonical_id("http://arxiv.org/abs/2301.00001").as_deref(), Some("2301.00001"));
    assert!(canonical_id("http://arxiv.org/abs/../../etc").is_none());
    assert!(canonical_id("http://example.org/x").is_none());
}
