use super::*;
use crate::db::insert::insert_many;
use crate::db::open_memory;

fn paper(id: &str, doi: Option<&str>, title: &str, year: i32) -> NewPaper {
    NewPaper {
        ext_id: id.into(),
        doi: doi.map(Into::into),
        title: title.into(),
        year: Some(year),
        authors: "Rahman M".into(),
        author_count: 1,
        keywords: Some("tuberculosis; screening".into()),
        refs: vec!["CHEN L, 2019, J DIGIT IMAGING".into()],
        ref_count: 1,
        ..Default::default()
    }
}

#[test]
fn skips_duplicate_ids_dois_and_titles() {
    let mut conn = open_memory();
    let items = vec![
        paper("openalex:W1", Some("10.1/a"), "First paper", 2024),
        paper("openalex:W1", None, "Same id", 2024),
        paper("wos:WOS:1", Some("10.1/a"), "Same doi", 2024),
        paper("scopus:1", None, "FIRST paper!", 2024),
        paper("scopus:2", None, "First paper", 2023),
    ];
    let (inserted, dup) = insert_many(&mut conn, &items, "Test").unwrap();
    assert_eq!((inserted, dup), (2, 3));
    let refs: i64 = conn.query_row("SELECT COUNT(*) FROM paper_refs", [], |r| r.get(0)).unwrap();
    assert_eq!(refs, 2);
}

#[test]
fn filter_treats_wildcards_literally() {
    let mut conn = open_memory();
    let items = vec![paper("a", None, "100% recall", 2024), paper("b", None, "Other title", 2024)];
    insert_many(&mut conn, &items, "Test").unwrap();
    let (rows, total) = list(&conn, Some("%"), 50, 0).unwrap();
    assert_eq!(total, 1);
    assert_eq!(rows[0].title, "100% recall");
    let (_, all) = list(&conn, None, 50, 0).unwrap();
    assert_eq!(all, 2);
}

#[test]
fn migrates_v1_rows_to_prefixed_ids() {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.execute_batch(super::super::schema::MIGRATIONS[0]).unwrap();
    conn.pragma_update(None, "user_version", 1).unwrap();
    conn.execute("INSERT INTO papers (openalex_id, title, origin) VALUES ('W9', 'Old', 'OpenAlex')", [])
        .unwrap();
    let tx = conn.transaction().unwrap();
    tx.execute_batch(super::super::schema::MIGRATIONS[1]).unwrap();
    tx.commit().unwrap();
    let id: String = conn.query_row("SELECT ext_id FROM papers", [], |r| r.get(0)).unwrap();
    assert_eq!(id, "openalex:W9");
}
