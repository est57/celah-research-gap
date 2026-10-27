use super::*;
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: usize, year: i32, kw: &str) -> NewPaper {
    NewPaper { ext_id: format!("p{id}"), title: format!("Paper {id}"), year: Some(year), keywords: Some(kw.into()), ..Default::default() }
}

fn params(cuts: Vec<i32>) -> EvolutionParams {
    EvolutionParams { field: "auto".into(), words: 50, min_cluster_freq: 0, labels: 2, cuts }
}

#[test]
fn links_a_continuing_theme_and_flags_a_new_one() {
    let mut conn = open_memory();
    let mut items = Vec::new();
    for i in 0..6 {
        items.push(paper(i, 2018 + i as i32 % 3, "deep learning; chest x-ray; cnn"));
        items.push(paper(10 + i, 2018 + i as i32 % 3, "sputum smear; microscopy; staining"));
        items.push(paper(20 + i, 2022 + i as i32 % 2, "deep learning; chest x-ray; vision transformer"));
        items.push(paper(30 + i, 2022 + i as i32 % 2, "cough sound; audio; smartphone"));
    }
    insert_many(&mut conn, &items, "Test").unwrap();
    let e = compute(&conn, &params(vec![2020])).unwrap();
    assert_eq!(e.cuts, vec![2020]);
    assert_eq!(e.periods.len(), 2);
    assert_eq!((e.periods[0].from, e.periods[0].to, e.periods[1].from), (2018, 2020, 2021));
    assert!(e.periods.iter().all(|p| p.themes.len() == 2 && p.documents == 12));

    let theme = |p: usize, word: &str| e.periods[p].themes.iter().find(|t| t.words.iter().any(|w| w.label == word)).unwrap().id;
    assert_eq!(e.links.len(), 1, "only the imaging theme continues");
    let l = &e.links[0];
    assert_eq!((l.period, l.from, l.to), (0, theme(0, "cnn"), theme(1, "vision transformer")));
    assert!((l.inclusion - 2.0 / 3.0).abs() < 1e-9, "12 of 18 occurrences are shared");
    assert_eq!((l.out_value, l.in_value), (12, 12));
    let audio = theme(1, "audio");
    assert!(e.links.iter().all(|l| l.to != audio), "the audio theme has no predecessor");
}

#[test]
fn splits_automatically_and_rejects_bad_cuts() {
    let mut conn = open_memory();
    let items: Vec<NewPaper> = (0..30).map(|i| paper(i, 2015 + (i as i32 % 9), "a b; c d; e f")).collect();
    insert_many(&mut conn, &items, "Test").unwrap();
    let e = compute(&conn, &params(Vec::new())).unwrap();
    assert_eq!(e.periods.len(), 3);
    assert_eq!(e.cuts.len(), 2);
    assert!(compute(&conn, &params(vec![2022, 2020])).is_err());
    let mut p = params(Vec::new());
    p.field = "keywords; --".into();
    assert!(compute(&conn, &p).is_err());
}
