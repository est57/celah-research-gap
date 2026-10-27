use super::*;

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("celah-projects-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn creates_lists_renames_and_deletes_projects() {
    let dir = temp_dir("crud");
    let mut r = load(&dir);
    assert_eq!((r.current.as_str(), r.names.len()), (MAIN, 1));
    let a = create(&dir, &mut r, "Skripsi TB").unwrap();
    let b = create(&dir, &mut r, "Skripsi TB").unwrap();
    assert_eq!((a.as_str(), b.as_str()), ("skripsi-tb", "skripsi-tb-2"));
    assert!(path_of(&dir, &a).unwrap().exists());
    assert_eq!(list(&dir, &r).len(), 3);
    rename(&dir, &mut r, &a, "Artikel jurnal").unwrap();
    assert_eq!(load(&dir).names[&a], "Artikel jurnal", "the registry is saved");
    delete(&dir, &mut r, &b).unwrap();
    assert!(!path_of(&dir, &b).unwrap().exists());
    assert!(delete(&dir, &mut r, MAIN).is_err());
    r.current = a.clone();
    assert!(delete(&dir, &mut r, &a).is_err(), "the open project stays");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn refuses_paths_outside_the_folder_and_bad_registries() {
    let dir = temp_dir("safe");
    assert!(path_of(&dir, "../celah").is_err());
    assert!(path_of(&dir, "A").is_err());
    let mut r = load(&dir);
    assert!(create(&dir, &mut r, "").is_err());
    assert!(create(&dir, &mut r, "a\u{0}b").is_err());
    assert_eq!(create(&dir, &mut r, "!!!").unwrap(), "proyek");
    std::fs::write(dir.join(REGISTRY), r#"{"current":"../x","names":{"../x":"evil"}}"#).unwrap();
    let back = load(&dir);
    assert_eq!((back.current.as_str(), back.names.len()), (MAIN, 1), "a tampered registry falls back to the main project");
    let _ = std::fs::remove_dir_all(&dir);
}
