use super::*;
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: usize, authors: &[&str], kw: &str, source: &str, country: &str) -> NewPaper {
    NewPaper {
        ext_id: format!("p{id}"),
        title: format!("Paper {id}"),
        year: Some(2020),
        source_name: Some(source.into()),
        keywords: Some(kw.into()),
        author_names: authors.iter().map(|a| a.to_string()).collect(),
        countries: vec![country.into()],
        refs: vec!["QIN ZZ, 2019, SCI REP, V9".into()],
        ..Default::default()
    }
}

#[test]
fn links_any_three_fields() {
    let mut conn = open_memory();
    let items = [
        paper(1, &["Rahman M", "Chen L"], "deep learning; tb", "Radiology", "ID"),
        paper(2, &["Rahman M"], "deep learning", "Radiology", "ID"),
        paper(3, &["Okafor C"], "cough", "PLOS ONE", "NG"),
    ];
    insert_many(&mut conn, &items, "Test").unwrap();
    let d = compute(&conn, 8).unwrap();
    assert_eq!(d.labels, ["Penulis", "Kata kunci penulis", "Sumber"]);
    assert_eq!((d.left[0].label.as_str(), d.left[0].count), ("Rahman M", 2));
    let p = |l: &str, m: &str, r: &str| ThreeFieldParams { left: l.into(), middle: m.into(), right: r.into(), items: 10 };
    let c = compute_params(&conn, &p("countries", "author_keywords", "references")).unwrap();
    assert_eq!(c.kinds[0], "countries");
    assert_eq!((c.left[0].label.as_str(), c.right[0].label.as_str()), ("ID", "Qin 2019"));
    let id_to_dl = c.left_links.iter().find(|l| l.from == 0 && c.middle[l.to].label == "deep learning").unwrap();
    assert_eq!(id_to_dl.value, 2);
    assert!(compute_params(&conn, &p("authors", "authors", "sources")).is_err());
    assert!(compute_params(&conn, &p("authors", "titles", "sources")).is_err());
    assert!(compute_params(&conn, &ThreeFieldParams { items: 50, ..p("authors", "sources", "countries") }).is_err());
}
