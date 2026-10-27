use super::*;
use crate::db::{filter, insert::insert_many, open, papers::NewPaper};

fn paper(id: usize, year: i32, origin_doc: &str) -> NewPaper {
    NewPaper {
        ext_id: format!("p{id}"),
        title: format!("Paper number {id}"),
        year: Some(year),
        doc_type: Some(origin_doc.into()),
        author_names: vec!["Rahman M".into()],
        refs: vec!["R".into()],
        ..Default::default()
    }
}

#[test]
fn filters_analyses_and_follows_new_data() {
    let path = std::env::temp_dir().join(format!("celah-filter-{}.sqlite", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let mut main = open(&path).unwrap();
    let items: Vec<NewPaper> = (0..10).map(|i| paper(i, 2015 + i as i32, if i % 2 == 0 { "article" } else { "review" })).collect();
    insert_many(&mut main, &items, "Test").unwrap();
    let mut db = AnalysisDb::open(&path).unwrap();
    let count = |c: &Connection, t: &str| c.query_row(&format!("SELECT COUNT(*) FROM {t}"), [], |r| r.get::<_, i64>(0)).unwrap();

    assert_eq!(count(db.prepare().unwrap(), "papers"), 10, "no filter: the whole collection");

    let f = CollectionFilter { year_from: Some(2018), doc_types: vec!["article".into()], ..Default::default() };
    filter::save(&main, &f).unwrap();
    assert_eq!(filter::matching(&main, &f).unwrap(), 3);
    let conn = db.prepare().unwrap();
    assert_eq!(count(conn, "papers"), 3);
    assert_eq!(count(conn, "paper_refs"), 3, "child tables follow the filtered papers");
    assert_eq!(count(&main, "papers"), 10, "the main connection still sees everything");

    insert_many(&mut main, &[paper(20, 2024, "article")], "Test").unwrap();
    assert_eq!(count(db.prepare().unwrap(), "papers"), 4, "new imports show up in the filtered view");

    filter::save(&main, &CollectionFilter::default()).unwrap();
    assert_eq!(count(db.prepare().unwrap(), "papers"), 11);
    drop(db);
    drop(main);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn rejects_bad_filters() {
    let bad_years = CollectionFilter { year_from: Some(2024), year_to: Some(2020), ..Default::default() };
    assert!(bad_years.validate().is_err());
    let control = CollectionFilter { origins: vec!["Scopus\u{0}".into()], ..Default::default() };
    assert!(control.validate().is_err());
    let many = CollectionFilter { sources: vec!["x".into(); 201], ..Default::default() };
    assert!(many.validate().is_err());
    let (sql, values) = CollectionFilter { sources: vec!["J'; DROP TABLE papers; --".into()], ..Default::default() }.condition();
    assert_eq!((sql.as_str(), values.len()), ("COALESCE(source_name, '') IN (?)", 1), "values are bound, never inlined");
}
