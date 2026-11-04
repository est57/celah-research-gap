use super::compute;
use crate::db::filter::{save, CollectionFilter};
use crate::db::insert::insert_many;
use crate::db::{open_memory, papers::NewPaper, screening};

fn paper(id: &str, title: &str, year: i32) -> NewPaper {
    NewPaper { ext_id: id.into(), title: title.into(), year: Some(year), ..Default::default() }
}

#[test]
fn flow_adds_up_from_identification_to_inclusion() {
    let mut conn = open_memory();
    let scopus = [paper("s:1", "Deep learning TB", 2020), paper("s:2", "Cough sound", 2021), paper("s:3", "Old study", 2010)];
    insert_many(&mut conn, &scopus, "Scopus").unwrap();
    // The same work again from another source is skipped as a duplicate.
    insert_many(&mut conn, &[paper("o:9", "Deep learning TB", 2020), paper("o:4", "Federated", 2022)], "OpenAlex").unwrap();
    let ids: Vec<i64> = conn.prepare("SELECT id FROM papers ORDER BY id").unwrap().query_map([], |r| r.get(0)).unwrap().map(Result::unwrap).collect();
    screening::set(&mut conn, &ids[1..2], Some("exclude"), "topic", "").unwrap();
    screening::set(&mut conn, &ids[..1], Some("include"), "", "").unwrap();
    save(&conn, &CollectionFilter { year_from: Some(2015), ..Default::default() }).unwrap();

    let p = compute(&conn).unwrap();
    assert_eq!((p.identified, p.duplicates, p.removed_other, p.screened), (5, 1, 0, 4));
    assert_eq!(p.sources.iter().map(|c| (c.label.as_str(), c.n)).collect::<Vec<_>>(), [("Scopus", 3), ("OpenAlex", 2)]);
    assert_eq!((p.excluded, p.reasons[0].label.as_str(), p.unscreened), (1, "topic", 1), "the unscreened 2010 study is outside the filter");
    assert!(p.filter_active);
    assert_eq!((p.filtered_out, p.included), (1, 2), "the 2010 study is outside the filter");

    // Removing a document shows up as removed for other reasons.
    conn.execute("DELETE FROM papers WHERE id = ?1", [ids[3]]).unwrap();
    let p = compute(&conn).unwrap();
    assert_eq!((p.removed_other, p.screened), (1, 3));
}

#[test]
fn exclusions_leave_the_analysis_count() {
    let mut conn = open_memory();
    insert_many(&mut conn, &[paper("a", "One", 2020), paper("b", "Two", 2021)], "Scopus").unwrap();
    let id: i64 = conn.query_row("SELECT id FROM papers WHERE ext_id = 'a'", [], |r| r.get(0)).unwrap();
    screening::set(&mut conn, &[id], Some("exclude"), "type", "review").unwrap();
    let stats = crate::db::stats::collection(&conn).unwrap();
    assert_eq!((stats.total, stats.analyzed, stats.filtered), (2, 1, true));
    screening::set(&mut conn, &[id], None, "", "").unwrap();
    assert_eq!(crate::db::stats::collection(&conn).unwrap().analyzed, 2);
}
