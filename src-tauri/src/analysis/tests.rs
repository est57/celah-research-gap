use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: &str, year: i32, cited: i64, authors: &[&str], kw: &str, source: &str) -> NewPaper {
    let names: Vec<String> = authors.iter().map(|a| a.to_string()).collect();
    NewPaper {
        ext_id: id.into(),
        title: format!("Paper {id}"),
        year: Some(year),
        cited_by: cited,
        author_count: names.len() as i64,
        author_names: names,
        keywords: Some(kw.into()),
        source_name: Some(source.into()),
        doc_type: Some("article".into()),
        refs: vec!["Ref A".into(), "ref a".into()],
        ..Default::default()
    }
}

#[test]
fn computes_overview_from_database() {
    let mut conn = open_memory();
    let items = vec![
        paper("a", 2020, 10, &["Rahman, M."], "Deep learning; TB", "PLOS ONE"),
        paper("b", 2021, 4, &["rahman m", "Chen L"], "deep learning; Cough", "plos one"),
        paper("c", 2022, 0, &["Chen L", "Okafor C", "Silva R"], "TB", "IEEE Access"),
    ];
    insert_many(&mut conn, &items, "Test").unwrap();
    let o = super::overview(&conn).unwrap();

    let i = &o.info;
    assert_eq!((i.year_min, i.year_max, i.documents), (Some(2020), Some(2022), 3));
    assert_eq!(i.sources, 2, "source names compare case-insensitively");
    assert_eq!(i.authors, 4, "'Rahman, M.' and 'rahman m' are one author");
    assert_eq!(i.author_keywords, 3);
    assert_eq!(i.references, 1, "references compare case-insensitively");
    assert_eq!((i.single_author_docs, i.authors_of_single), (1, 1));
    assert_eq!(i.coauthors_per_doc, Some(2.0));

    assert_eq!(o.annual.len(), 3);
    assert_eq!(o.annual[0].documents, 1);

    let t = &o.three_field;
    assert_eq!(t.labels[1], "Kata kunci penulis");
    assert_eq!(t.left[0].label, "Chen L", "Chen L and Rahman M tie at 2 papers; alphabetical order wins");
    assert_eq!(t.middle.len(), 3);
    assert_eq!(t.right[0].count, 2);
    let total_right: i64 = t.right_links.iter().map(|l| l.value).sum();
    assert_eq!(total_right, 5, "every keyword occurrence flows to its source");
}
