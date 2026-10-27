use super::*;
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: &str, authors: &[&str]) -> NewPaper {
    NewPaper { ext_id: id.into(), title: id.into(), author_names: authors.iter().map(|a| a.to_string()).collect(), ..Default::default() }
}

fn keys(conn: &Connection) -> Vec<String> {
    let mut stmt = conn.prepare("SELECT DISTINCT name_key FROM paper_authors ORDER BY 1").unwrap();
    stmt.query_map([], |r| r.get(0)).unwrap().map(Result::unwrap).collect()
}

#[test]
fn merges_variants_applies_them_to_imports_and_splits_again() {
    let mut conn = open_memory();
    insert_many(&mut conn, &[paper("a", &["Rahman M", "Chen L"]), paper("b", &["Muhammad Rahman"])], "Test").unwrap();
    assert_eq!(keys(&conn), vec!["chen l", "muhammad rahman", "rahman m"]);

    let changed = merge(&mut conn, &["muhammad rahman".into(), "rahman m".into()], "rahman m").unwrap();
    assert_eq!(changed, 1);
    assert_eq!(keys(&conn), vec!["chen l", "rahman m"]);
    let printed: String = conn.query_row("SELECT name FROM paper_authors WHERE name LIKE 'Muhammad%'", [], |r| r.get(0)).unwrap();
    assert_eq!(printed, "Muhammad Rahman", "the byline is untouched");

    insert_many(&mut conn, &[paper("c", &["Muhammad Rahman"])], "Test").unwrap();
    assert_eq!(keys(&conn), vec!["chen l", "rahman m"], "a new import follows the merge");
    assert_eq!(list(&conn).unwrap().len(), 1);

    assert_eq!(split(&mut conn, "muhammad rahman").unwrap(), 2);
    assert_eq!(keys(&conn), vec!["chen l", "muhammad rahman", "rahman m"]);
    assert!(list(&conn).unwrap().is_empty());

    assert!(merge(&mut conn, &["rahman m".into()], "rahman m").is_err(), "needs two names");
    assert!(merge(&mut conn, &["rahman m".into(), "chen l".into()], "okafor c").is_err(), "target must be one of them");
    assert!(merge(&mut conn, &["rahman m".into(), "nobody".into()], "rahman m").is_err());
    assert!(split(&mut conn, "chen l").is_err());
}

#[test]
fn chained_merges_point_at_the_last_target() {
    let mut conn = open_memory();
    insert_many(&mut conn, &[paper("a", &["Rahman M"]), paper("b", &["Rahman MA"]), paper("c", &["Muhammad Rahman"])], "Test").unwrap();
    merge(&mut conn, &["rahman m".into(), "rahman ma".into()], "rahman ma").unwrap();
    merge(&mut conn, &["rahman ma".into(), "muhammad rahman".into()], "muhammad rahman").unwrap();
    assert_eq!(keys(&conn), vec!["muhammad rahman"]);
    assert!(list(&conn).unwrap().iter().all(|m| m.to_key == "muhammad rahman"));
}
