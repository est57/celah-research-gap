use super::{compute, text::grams, WordParams};
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: &str, year: i32, title: &str, kw: &str, abs: Option<&str>) -> NewPaper {
    NewPaper {
        ext_id: id.into(),
        title: title.into(),
        year: Some(year),
        keywords: Some(kw.into()),
        abstract_text: abs.map(Into::into),
        ..Default::default()
    }
}

fn params(field: &str, ngram: u32, top: u32) -> WordParams {
    WordParams { field: field.into(), ngram, top }
}

#[test]
fn splits_phrases_at_stopwords_and_punctuation() {
    let g = grams("Deep learning for cancer detection: a deep-learning study. Deep learning!", 2);
    assert_eq!(g, vec!["deep learning", "cancer detection"], "a hyphenated word stays one word");
    assert_eq!(grams("The model's 2020 results, using CNNs", 1), vec!["model", "cnns"]);
    assert!(grams("of and the", 1).is_empty());
}

#[test]
fn counts_documents_per_keyword_and_year() {
    let mut conn = open_memory();
    let items = vec![
        paper("a", 2020, "Deep learning for tuberculosis", "Deep Learning; TB", Some("Deep learning detects TB.")),
        paper("b", 2022, "Deep learning in cough audio", "deep learning; Cough", None),
        paper("c", 2022, "Cough screening apps", "Cough; deep learning; cough", None),
    ];
    insert_many(&mut conn, &items, "Test").unwrap();

    let w = compute(&conn, &params("author_keywords", 3, 10)).unwrap();
    assert_eq!((w.documents, w.with_field, w.distinct, w.ngram), (3, 3, 3, 1));
    assert_eq!(w.words[0].label, "deep learning", "the most common spelling is shown");
    assert_eq!(w.words[0].documents, 3);
    assert_eq!((w.words[1].label.as_str(), w.words[1].documents), ("Cough", 2), "a keyword counts once per document");
    assert_eq!(w.years, vec![2020, 2021, 2022], "years run without gaps");
    assert_eq!(w.series[0].per_year, vec![1, 0, 2]);

    let t = compute(&conn, &params("title", 2, 10)).unwrap();
    assert_eq!(t.words[0].label, "deep learning");
    assert_eq!(t.words[0].documents, 2);

    let a = compute(&conn, &params("abstract", 1, 10)).unwrap();
    assert_eq!(a.with_field, 1, "only documents with an abstract");
}

#[test]
fn rejects_unknown_fields_and_ranges() {
    let conn = open_memory();
    assert!(compute(&conn, &params("authors", 1, 20)).is_err());
    assert!(compute(&conn, &params("title", 4, 20)).is_err());
    assert!(compute(&conn, &params("title", 1, 500)).is_err());
    assert!(compute(&conn, &params("author_keywords", 9, 20)).is_ok(), "keyword fields ignore the phrase length");
    let empty = compute(&conn, &params("title", 1, 20)).unwrap();
    assert!(empty.words.is_empty() && empty.years.is_empty());
}

#[test]
fn folds_plurals_but_not_words_ending_in_s() {
    use super::text::key;
    assert_eq!(key("neural networks"), key("neural network"));
    assert_eq!(key("x-rays"), "x-ray");
    assert_eq!(key("analysis"), "analysis");
    assert_eq!(key("virus class"), "virus class");
}
