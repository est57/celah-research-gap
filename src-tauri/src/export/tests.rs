use super::*;
use crate::db::{insert::insert_many, open, open_memory, papers::NewPaper};

fn paper(id: &str, title: &str, year: i32) -> NewPaper {
    NewPaper {
        ext_id: id.into(),
        title: title.into(),
        year: Some(year),
        doi: Some(format!("10.1/{id}")),
        author_names: vec!["Rahman M".into(), "Chen L".into()],
        countries: vec!["ID".into()],
        ..Default::default()
    }
}

#[test]
fn exports_csv_bibtex_and_ris_safely() {
    let mut conn = open_memory();
    insert_many(&mut conn, &[paper("b", "=HYPERLINK(\"http://x\")", 2022), paper("a", "Deep learning, TB \"screening\"", 2019)], "Test").unwrap();
    let (bytes, ext, n) = build(&conn, "csv").unwrap();
    let text = String::from_utf8(bytes).unwrap();
    assert_eq!((ext, n), ("csv", 2));
    assert!(text.starts_with('\u{FEFF}'));
    let mut reader = csv::Reader::from_reader(text.trim_start_matches('\u{FEFF}').as_bytes());
    let rows: Vec<csv::StringRecord> = reader.records().map(Result::unwrap).collect();
    assert_eq!(&rows[0][1], "Deep learning, TB \"screening\"", "oldest first, commas and quotes survive");
    assert_eq!(&rows[0][2], "Rahman M; Chen L");
    assert!(rows[1][1].starts_with("'="), "a formula-looking title is neutralized");

    let (bib, ext, n) = build(&conn, "bibtex").unwrap();
    assert_eq!((ext, n), ("bib", 2));
    assert_eq!(String::from_utf8(bib).unwrap().matches("@article").count(), 2);
    assert!(String::from_utf8(build(&conn, "ris").unwrap().0).unwrap().contains("DO  - 10.1/a"));
    assert!(build(&conn, "xlsx").is_err());
}

#[test]
fn backs_up_to_a_file_that_opens() {
    let dir = std::env::temp_dir().join(format!("celah-backup-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut conn = open(&dir.join("live.sqlite")).unwrap();
    insert_many(&mut conn, &[paper("a", "Title", 2020)], "Test").unwrap();
    let target = dir.join("cadangan.sqlite");
    std::fs::write(&target, b"old").unwrap();
    backup(&conn, &target).unwrap();
    let copy = open(&target).unwrap();
    let n: i64 = copy.query_row("SELECT COUNT(*) FROM papers", [], |r| r.get(0)).unwrap();
    assert_eq!(n, 1, "an existing file the user chose is replaced");
    assert!(!dir.join("cadangan.celah-tmp").exists());
    drop((conn, copy));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn restores_a_backup_and_rejects_other_files() {
    let dir = std::env::temp_dir().join(format!("celah-restore-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut live = open(&dir.join("live.sqlite")).unwrap();
    insert_many(&mut live, &[paper("a", "Kept in backup", 2020)], "Test").unwrap();
    let saved = dir.join("cadangan.sqlite");
    backup(&live, &saved).unwrap();
    insert_many(&mut live, &[paper("b", "Added later", 2021)], "Test").unwrap();

    let info = restore::inspect(&saved).unwrap();
    assert_eq!((info.documents, info.reviews, info.schema), (1, 0, crate::db::SCHEMA_VERSION));
    restore::restore(&mut live, &saved).unwrap();
    let titles: Vec<String> = live.prepare("SELECT title FROM papers").unwrap().query_map([], |r| r.get(0)).unwrap().map(Result::unwrap).collect();
    assert_eq!(titles, vec!["Kept in backup"]);
    let mode: String = live.query_row("PRAGMA journal_mode", [], |r| r.get(0)).unwrap();
    assert_eq!(mode, "wal");

    let text = dir.join("bukan.sqlite");
    std::fs::write(&text, b"not a database").unwrap();
    assert!(restore::inspect(&text).is_err());
    let other = rusqlite::Connection::open(dir.join("lain.sqlite")).unwrap();
    other.execute_batch("CREATE TABLE notes (x TEXT); PRAGMA user_version = 3;").unwrap();
    drop(other);
    assert!(restore::inspect(&dir.join("lain.sqlite")).is_err(), "a database without Celah tables is refused");
    drop(live);
    let _ = std::fs::remove_dir_all(&dir);
}
