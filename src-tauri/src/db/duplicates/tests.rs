use super::*;
use crate::db::insert::insert_many;
use crate::db::open_memory;
use crate::db::papers::NewPaper;

fn paper(ext: &str, title: &str, year: i32, origin_preprint: bool) -> NewPaper {
    NewPaper {
        ext_id: ext.into(),
        doi: Some(if origin_preprint { format!("10.48550/{ext}") } else { format!("10.1/{ext}") }),
        title: title.into(),
        year: Some(year),
        doc_type: Some(if origin_preprint { "preprint" } else { "article" }.into()),
        authors: "Rahman M".into(),
        author_count: 1,
        author_names: vec!["Rahman M".into()],
        ..Default::default()
    }
}

const T: &str = "Audio spectrogram transformers for tuberculosis cough screening";

#[test]
fn finds_preprint_and_near_titles_but_not_generic_or_distinct_works() {
    let mut conn = open_memory();
    let mut published = paper("a", T, 2024, false);
    published.refs = vec!["R1".into()];
    let items = vec![
        paper("p", T, 2023, true),
        published,
        paper("n", "Audio spectrogram transformers for tuberculosis cough screening:", 2024, true),
        paper("e1", "Editorial", 2020, false),
        paper("e2", "Editorial", 2021, false),
        paper("j", "Deep learning for chest radiograph triage in rural clinics", 2022, false),
        paper("c", "Deep learning for chest radiograph triage in rural clinic", 2022, false),
    ];
    insert_many(&mut conn, &items, "Test").unwrap();
    let list = list(&conn).unwrap();
    let titles: Vec<(&str, &str)> = list.pairs.iter().map(|p| (p.a.title.as_str(), p.b.title.as_str())).collect();
    assert!(list.pairs.iter().all(|p| p.a.title != "Editorial"), "generic titles are not duplicates: {titles:?}");
    assert!(list.pairs.iter().all(|p| !p.a.title.starts_with("Deep")), "two different DOIs of published works stay apart");
    let pre = list.pairs.iter().find(|p| p.reason == "same_title").expect("preprint pair");
    assert_eq!(pre.keep, if pre.a.is_preprint() { pre.b.id } else { pre.a.id }, "the published version is kept");
}

#[test]
fn merge_fills_fields_moves_refs_and_blocks_reimport() {
    let mut conn = open_memory();
    let mut pre = paper("arxiv:1", T, 2023, true);
    pre.abstract_text = Some("Abstract".into());
    pre.refs = vec!["R1".into(), "R2".into()];
    pre.countries = vec!["ID".into()];
    let mut published = paper("openalex:W1", T, 2024, false);
    published.doi = None;
    insert_many(&mut conn, &[pre, published], "Test").unwrap();
    let id = |ext: &str| conn.query_row("SELECT id FROM papers WHERE ext_id = ?1", [ext], |r| r.get::<_, i64>(0)).unwrap();
    let (drop, keep) = (id("arxiv:1"), id("openalex:W1"));

    merge(&mut conn, keep, drop).unwrap();
    let (abs, doi): (String, String) = conn.query_row("SELECT abstract, doi FROM papers WHERE id = ?1", [keep], |r| Ok((r.get(0)?, r.get(1)?))).unwrap();
    assert_eq!((abs.as_str(), doi.as_str()), ("Abstract", "10.48550/arxiv:1"));
    let count = |sql: &str| conn.query_row(sql, [], |r| r.get::<_, i64>(0)).unwrap();
    assert_eq!(count("SELECT COUNT(*) FROM papers"), 1);
    assert_eq!(count(&format!("SELECT COUNT(*) FROM paper_refs WHERE paper_id = {keep}")), 2);
    assert_eq!(count(&format!("SELECT COUNT(*) FROM paper_countries WHERE paper_id = {keep}")), 1);

    let (inserted, _) = insert_many(&mut conn, &[paper("arxiv:1", "Other title entirely here", 2023, true)], "Test").unwrap();
    assert_eq!(inserted, 0, "a merged record is not imported again");
    assert!(merge(&mut conn, keep, drop).is_err());
    assert!(merge(&mut conn, keep, keep).is_err());
}

#[test]
fn dismissed_pairs_are_hidden() {
    let mut conn = open_memory();
    insert_many(&mut conn, &[paper("p", T, 2023, true), paper("a", T, 2024, false)], "Test").unwrap();
    let pair = list(&conn).unwrap().pairs.remove(0);
    dismiss(&conn, pair.b.id, pair.a.id).unwrap();
    assert!(list(&conn).unwrap().pairs.is_empty());
    assert!(dismiss(&conn, 9999, pair.a.id).is_err());
}
