use super::*;
use crate::db::{analysis_conn::AnalysisDb, insert::insert_many, open, papers::NewPaper};

fn paper(id: &str, kw: &str) -> NewPaper {
    NewPaper { ext_id: id.into(), title: id.into(), year: Some(2020), keywords: Some(kw.into()), ..Default::default() }
}

#[test]
fn rewrites_keywords_without_repeats() {
    let map: Map = [("cnns".to_string(), "CNN".to_string()), ("convolutional neural network".to_string(), "CNN".to_string())].into();
    assert_eq!(rewrite("CNNs; deep learning; Convolutional Neural Network", &map), "CNN; deep learning");
    assert_eq!(suggest::spelling_form("Deep-Learning"), suggest::spelling_form("deep learnings"));
}

#[test]
fn analyses_see_synonyms_and_suggestions_find_them() {
    let path = std::env::temp_dir().join(format!("celah-syn-{}.sqlite", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let mut main = open(&path).unwrap();
    insert_many(&mut main, &[paper("a", "CNN; tuberculosis"), paper("b", "convolutional neural networks; tuberculosis"), paper("c", "Convolutional neural network; chest x-ray"), paper("d", "chest X-rays")], "Test").unwrap();
    let s = suggest::suggest(&main).unwrap();
    assert!(s.iter().any(|g| g.kind == "singkatan" && g.terms.iter().any(|t| t.label == "CNN")), "acronym found");
    assert!(s.iter().any(|g| g.kind == "ejaan" && g.terms.len() == 2 && g.terms.iter().any(|t| t.label == "chest X-rays")));

    merge(&mut main, &["CNN".into(), "convolutional neural networks".into(), "Convolutional neural network".into()], "CNN").unwrap();
    assert_eq!(list(&main).unwrap().len(), 2);
    let mut db = AnalysisDb::open(&path).unwrap();
    let conn = db.prepare().unwrap();
    let kws: Vec<String> = conn.prepare("SELECT keywords FROM papers ORDER BY id").unwrap().query_map([], |r| r.get(0)).unwrap().map(Result::unwrap).collect();
    assert_eq!(kws[1], "CNN; tuberculosis");
    assert_eq!(kws[2], "CNN; chest x-ray");
    let stored: String = main.query_row("SELECT keywords FROM papers WHERE ext_id = 'b'", [], |r| r.get(0)).unwrap();
    assert_eq!(stored, "convolutional neural networks; tuberculosis", "the stored text is untouched");

    remove(&main, "cnn").unwrap_err();
    remove(&main, "convolutional neural networks").unwrap();
    assert!(merge(&mut main, &["a; b".into(), "c".into()], "c").is_err());
    drop((db, main));
    let _ = std::fs::remove_file(&path);
}
