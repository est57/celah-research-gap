use super::{build, SummaryParams};
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: usize, year: i32, cited: i64, source: &str, country: &str) -> NewPaper {
    NewPaper {
        ext_id: format!("p{id}"),
        title: format!("Study {id}"),
        year: Some(year),
        cited_by: cited,
        author_names: vec![format!("Author{}, A.", id % 3), "Chen, L.".into()],
        source_name: Some(source.into()),
        countries: vec![country.into()],
        corresponding_country: Some(country.into()),
        refs: vec!["LECUN Y, 1998, P IEEE, V86".into()],
        ..Default::default()
    }
}

fn params(style: &str, language: &str) -> SummaryParams {
    SummaryParams { style: style.into(), language: language.into() }
}

#[test]
fn drafts_methods_and_results_with_citations() {
    let mut conn = open_memory();
    let items: Vec<NewPaper> = (0..6).map(|i| paper(i, 2020 + (i % 3) as i32, 10 * i as i64, if i < 4 { "PLOS ONE" } else { "IEEE Access" }, if i < 5 { "ID" } else { "US" })).collect();
    insert_many(&mut conn, &items, "Scopus").unwrap();

    let s = build(&conn, &params("apa", "id")).unwrap();
    assert_eq!(s.paragraphs.len(), 3);
    let method = &s.paragraphs[0].text;
    assert!(method.contains("Scopus (6)") && method.contains("6 dokumen terbitan 2020-2022"), "{method}");
    assert!(method.contains("(Aria & Cuccurullo, 2017)"));
    let perf = &s.paragraphs[1].text;
    assert!(perf.contains("PLOS ONE (4)") && perf.contains("Indonesia (5)") && perf.contains("Amerika Serikat (1)"), "{perf}");
    let influence = &s.paragraphs[2].text;
    assert!(influence.contains("Author2 & Chen (2022) dengan 50 sitasi"), "{influence}");
    assert!(influence.contains("1998 (Lecun 1998)") && influence.contains("(Marx et al., 2014)"), "{influence}");
    assert!(s.references.iter().any(|r| r.starts_with("Aria, M., & Cuccurullo, C. (2017).")));
    assert!(s.bibtex.contains("bibliometrix") && s.markdown.contains("## 1. Data dan metode"));

    let en = build(&conn, &params("ieee", "en")).unwrap();
    assert!(en.paragraphs[0].text.contains("bibliometrix workflow [1]"));
    assert!(en.paragraphs[2].text.contains("[2] with 50 citations"), "{}", en.paragraphs[2].text);
    assert!(en.paragraphs[1].text.contains("United States (1)"));
    assert!(en.references[0].starts_with("[1] M. Aria"));
}

#[test]
fn rejects_bad_params_and_empty_collections() {
    let conn = open_memory();
    assert!(build(&conn, &params("apa", "id")).is_err(), "empty collection");
    assert!(build(&conn, &params("mla", "id")).is_err());
    assert!(build(&conn, &params("apa", "fr")).is_err());
}
