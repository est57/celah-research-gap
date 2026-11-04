use super::{compute, rank_for, GapTerms};
use crate::db::{insert::insert_many, open_memory, papers::NewPaper, reviews, sjr};
use crate::import::sjr::Journal;

fn paper(id: &str, year: i32, cited: i64, kw: &str, source: &str, issn: Option<&str>) -> NewPaper {
    NewPaper {
        ext_id: id.into(),
        title: format!("Paper {id}"),
        year: Some(year),
        cited_by: cited,
        keywords: Some(kw.into()),
        source_name: Some(source.into()),
        issn: issn.map(Into::into),
        ..Default::default()
    }
}

fn gap(title: &str, terms: &[&str]) -> GapTerms {
    GapTerms { title: title.into(), terms: terms.iter().map(|t| t.to_string()).collect() }
}

#[test]
fn ranks_recent_matching_journals_first_with_quartiles() {
    let mut conn = open_memory();
    let j = |issn: &str, title: &str, q: &str| Journal { issns: issn.into(), title: title.into(), sjr: Some(1.3), quartile: Some(q.into()), categories: None };
    sjr::replace(&mut conn, &[j("11112222", "Audio Journal", "Q1")], Some(2024)).unwrap();
    let items = vec![
        paper("a1", 2016, 5, "cough sound; triage", "Old Journal", None),
        paper("a2", 2017, 5, "cough sound", "Old Journal", None),
        paper("a3", 2018, 5, "cough sound", "Old Journal", None),
        paper("b1", 2023, 2, "smartphone audio", "Audio Journal", Some("11112222")),
        paper("b2", 2024, 1, "Cough Sound", "audio journal", Some("11112222")),
        paper("b3", 2024, 0, "radiology", "Audio Journal", Some("11112222")),
        paper("c1", 2024, 9, "chest x-ray", "Imaging Letters", None),
        paper("d1", 2024, 9, "cough sound", "", None),
        paper("e1", 2024, 9, "radiology", "Filler", None),
        paper("e2", 2024, 9, "radiology", "Filler", None),
    ];
    insert_many(&mut conn, &items, "Test").unwrap();
    let gaps = [gap("Cough on phones", &["cough sound"]), gap("Phone audio", &["smartphone"])];
    let r = rank_for(&conn, &gaps, false).unwrap();
    assert_eq!((r.documents, r.recent_from, r.fallback), (10, Some(2022), false));
    let labels: Vec<&str> = r.journals.iter().map(|j| j.label.as_str()).collect();
    assert_eq!(labels, vec!["Audio Journal", "Old Journal"], "no match and empty names are left out");
    let top = &r.journals[0];
    assert_eq!((top.documents, top.matching, top.recent, top.citations), (3, 2, 2, 3));
    assert_eq!((top.quartile.as_str(), top.sjr), ("Q1", Some(1.3)));
    assert_eq!(top.score, 4.8, "(2 matching + 2 recent) x 1.2 for Q1");
    assert_eq!(top.gaps, vec!["Cough on phones", "Phone audio"]);
    let old = &r.journals[1];
    assert_eq!((old.matching, old.recent, old.quartile.as_str(), old.sjr, old.score), (3, 0, "", None, 3.0));
    assert_eq!(old.gaps, vec!["Cough on phones"]);
}

fn gap_collection() -> Vec<NewPaper> {
    let mut items = Vec::new();
    for i in 0..30 {
        items.push(paper(&format!("x{i}"), 2016 + i % 6, 1, "deep learning; chest x-ray; cnn", "Radiology Reports", None));
        items.push(paper(&format!("y{i}"), 2017 + i % 5, 1, "cough sound; audio; triage", "Audio Journal", None));
    }
    for i in 0..4 {
        let mut p = paper(&format!("z{i}"), 2024, 0, "cough sound; smartphone", "Audio Journal", None);
        p.abstract_text = Some("Audio models look promising. Robustness to smartphone noise remains unclear.".into());
        items.push(p);
    }
    items
}

#[test]
fn falls_back_to_all_candidates_until_a_gap_is_accepted() {
    let mut conn = open_memory();
    insert_many(&mut conn, &gap_collection(), "Test").unwrap();
    let r = compute(&conn).unwrap();
    assert!(r.fallback);
    assert!(r.gaps.len() > 1);
    assert!(!r.journals.is_empty());

    let list = crate::analysis::gaps::compute(&conn).unwrap();
    let chosen = &list.gaps[0];
    reviews::set(&conn, &chosen.key, "accepted", "", "").unwrap();
    let r = compute(&conn).unwrap();
    assert!(!r.fallback);
    assert_eq!(r.gaps, vec![chosen.title.clone()]);
    assert!(r.journals.iter().all(|j| j.gaps == vec![chosen.title.clone()]));
}

#[test]
fn empty_collection_has_no_journals() {
    let conn = open_memory();
    let r = compute(&conn).unwrap();
    assert!(r.fallback && r.journals.is_empty() && r.recent_from.is_none());
}
