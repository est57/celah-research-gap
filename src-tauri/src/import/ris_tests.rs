const ZOTERO: &str = "TY  - JOUR\r\nTI  - Deep learning for TB screening\r\nAU  - Rahman, Mohammad\r\nAU  - Chen, Li\r\n\
T2  - Scientific Reports\r\nAB  - We screen radiographs.\r\nDA  - 2023/05/01/\r\nPY  - 2023\r\nDO  - 10.1038/S41598-023-0001\r\n\
SN  - 2045-2322\r\nKW  - deep learning\r\nKW  - tuberculosis\r\nM3  - Text\r\nAD  - Univ X, Dhaka, Bangladesh\r\nER  - \r\n\r\n\
TY  - BOOK\r\nTI  - A book without identifiers\r\nPY  - 2019\r\nER  - \r\n";

const SCOPUS: &str = "TY  - JOUR\nAU  - Qin, Z.Z.\nTI  - Using artificial intelligence to read chest radiographs\nPY  - 2019\n\
T2  - Scientific Reports\nVL  - 9\nDO  - 10.1038/s41598-019-51503-3\n\
UR  - https://www.scopus.com/inward/record.uri?eid=2-s2.0-85074000000&doi=10.1038%2fs41598&partnerID=40\n\
AD  - Stop TB Partnership, Geneva, Switzerland\nKW  - Tuberculosis\nM3  - Review\n\
N1  - Cited By :87\nN1  - Correspondence Address: Qin, Z.Z.; Stop TB Partnership, Geneva, Switzerland\n\
N1  - References: Lakhani P., 2017, Radiology, 284, pp. 574; Rajpurkar P., 2018, PLOS Med\nDB  - Scopus\nER  - \n";

#[test]
fn reads_a_zotero_export() {
    let papers = super::parse(ZOTERO);
    assert_eq!(papers.len(), 2);
    let p = &papers[0];
    assert_eq!(p.ext_id, "doi:10.1038/s41598-023-0001");
    assert_eq!((p.title.as_str(), p.year), ("Deep learning for TB screening", Some(2023)));
    assert_eq!(p.authors, "Rahman, Mohammad; Chen, Li");
    assert_eq!(p.source_name.as_deref(), Some("Scientific Reports"));
    assert_eq!(p.issn.as_deref(), Some("20452322"));
    assert_eq!(p.keywords.as_deref(), Some("deep learning; tuberculosis"));
    assert_eq!(p.doc_type.as_deref(), Some("article"), "free text in M3 is ignored");
    assert_eq!(p.countries, vec!["BD"]);
    assert_eq!(papers[1].ext_id, "title:abookwithoutidentifiers:2019");
    assert_eq!(papers[1].doc_type.as_deref(), Some("book"));
}

#[test]
fn reads_scopus_notes() {
    let p = &super::parse(SCOPUS)[0];
    assert_eq!(p.ext_id, "scopus:2-s2.0-85074000000");
    assert_eq!(p.cited_by, 87);
    assert_eq!(p.refs.len(), 2);
    assert_eq!(p.doc_type.as_deref(), Some("review"));
    assert_eq!(p.corresponding_country.as_deref(), Some("CH"));
}
