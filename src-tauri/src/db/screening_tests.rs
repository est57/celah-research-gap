use super::{counts, page, set, ScreenCounts};
use crate::db::insert::insert_many;
use crate::db::{open_memory, papers::NewPaper};

fn paper(id: &str, title: &str, year: i32) -> NewPaper {
    NewPaper { ext_id: id.into(), title: title.into(), year: Some(year), abstract_text: Some(format!("about {title}")), ..Default::default() }
}

#[test]
fn decisions_are_set_listed_and_cleared() {
    let mut conn = open_memory();
    insert_many(&mut conn, &[paper("a", "Cough audio", 2021), paper("b", "Chest x-ray", 2022), paper("c", "Sputum", 2020)], "Scopus").unwrap();
    let (all, total) = page(&conn, "all", None, 10, 0).unwrap();
    assert_eq!((total, all[0].title.as_str()), (3, "Chest x-ray"), "newest first");
    let ids: Vec<i64> = all.iter().map(|r| r.id).collect();
    assert_eq!(set(&mut conn, &ids[..2], Some("exclude"), "topic", "not TB").unwrap(), 2);
    set(&mut conn, &ids[2..], Some("include"), "", "").unwrap();
    assert_eq!(counts(&conn).unwrap(), ScreenCounts { unscreened: 0, included: 1, excluded: 2 });
    let (ex, n) = page(&conn, "exclude", Some("audio"), 10, 0).unwrap();
    assert_eq!((n, ex[0].reason.as_str(), ex[0].note.as_str()), (1, "topic", "not TB"));
    // Changing a decision replaces it; clearing returns the document to "not screened".
    set(&mut conn, &ids[..1], Some("include"), "", "").unwrap();
    set(&mut conn, &ids[1..2], None, "", "").unwrap();
    assert_eq!(counts(&conn).unwrap(), ScreenCounts { unscreened: 1, included: 2, excluded: 0 });
    assert_eq!(page(&conn, "none", None, 10, 0).unwrap().1, 1);
    assert!(page(&conn, "bogus", None, 10, 0).is_err());
    // Unknown ids change nothing.
    assert_eq!(set(&mut conn, &[9_999], Some("exclude"), "other", "").unwrap(), 0);
}

#[test]
fn wildcards_in_the_query_match_literally() {
    let mut conn = open_memory();
    insert_many(&mut conn, &[paper("a", "100% accuracy", 2021), paper("b", "Plain", 2022)], "Scopus").unwrap();
    assert_eq!(page(&conn, "all", Some("%"), 10, 0).unwrap().1, 1);
}
