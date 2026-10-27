use super::*;
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(ext: &str, doi: Option<&str>) -> NewPaper {
    NewPaper { ext_id: ext.into(), doi: doi.map(str::to_string), title: format!("Title of {ext}"), year: Some(2020), ..Default::default() }
}

#[test]
fn stores_cites_aliases_and_skips_checked_documents() {
    let mut conn = open_memory();
    let mut has = paper("openalex:W9", None);
    has.cites = vec!["openalex:W1".into()];
    let items = [paper("openalex:W1", None), paper("scopus:2", Some("10.1/b")), paper("pubmed:3", None), has];
    insert_many(&mut conn, &items, "Test").unwrap();
    assert_eq!(remaining(&conn).unwrap(), 3, "a document without id/DOI is not pending; W9 lacks institutions");

    let p = pending(&conn, 100).unwrap();
    assert_eq!((p.by_id.len(), p.by_doi.len()), (2, 1));
    let records = vec![
        RefRecord { id: "W1".into(), doi: None, cites: vec![], affiliations: vec![] },
        RefRecord {
            id: "W2".into(),
            doi: Some("10.1/b".into()),
            cites: vec!["openalex:W1".into(), "openalex:W7".into()],
            affiliations: crate::geo::affil::from_named("Universitas Indonesia", Some("ID")).into_iter().collect(),
        },
    ];
    assert_eq!(save(&mut conn, &p, &records).unwrap(), 1);
    let count = |sql: &str| conn.query_row(sql, [], |r| r.get::<_, i64>(0)).unwrap();
    assert_eq!(count("SELECT COUNT(*) FROM paper_cites WHERE cited = 'openalex:W1'"), 2);
    assert_eq!(count("SELECT COUNT(*) FROM paper_aliases WHERE ext_id = 'openalex:W2'"), 1, "the Scopus record now answers to W2");
    assert_eq!(count("SELECT COUNT(*) FROM paper_affiliations WHERE name_key = 'universitas indonesia' AND country = 'ID'"), 1);
    assert_eq!(remaining(&conn).unwrap(), 0, "W1 has no cited works but is not asked again");

    let (inserted, _) = insert_many(&mut conn, &[paper("openalex:W2", Some("10.9/other"))], "Test").unwrap();
    assert_eq!(inserted, 0, "the aliased OpenAlex record is not imported as a second copy");
}
