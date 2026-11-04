use super::{count, create, delete, get, hits, list, record_run};
use crate::db::insert::insert_many;
use crate::db::{open_memory, papers::NewPaper};
use crate::search::Source;

#[test]
fn watches_keep_their_query_and_the_documents_each_run_added() {
    let mut conn = open_memory();
    let id = create(&conn, "cough sound tuberculosis", &[Source::Openalex, Source::Pubmed], Some("article"), 2020, 200).unwrap();
    let w = get(&conn, id).unwrap().unwrap();
    assert_eq!((w.query.as_str(), w.sources.as_slice(), w.doc_type.as_deref(), w.year_from), ("cough sound tuberculosis", &[Source::Openalex, Source::Pubmed][..], Some("article"), 2020));
    assert!(w.last_run_at.is_none());

    let papers = [NewPaper { ext_id: "a".into(), title: "Cough audio".into(), year: Some(2025), ..Default::default() }];
    insert_many(&mut conn, &papers, "OpenAlex").unwrap();
    let paper: i64 = conn.query_row("SELECT id FROM papers", [], |r| r.get(0)).unwrap();
    record_run(&mut conn, id, &[paper]).unwrap();
    record_run(&mut conn, id, &[]).unwrap();
    let w = &list(&conn).unwrap()[0];
    assert_eq!((w.last_new, w.total_new), (0, 1), "the second run found nothing new");
    assert!(w.last_run_at.is_some());
    assert_eq!(hits(&conn, id, 10).unwrap()[0].title, "Cough audio");

    // Removing the document from the collection drops its hit; deleting the watch drops the rest.
    conn.execute("DELETE FROM papers", []).unwrap();
    assert!(hits(&conn, id, 10).unwrap().is_empty());
    assert!(delete(&conn, id).unwrap());
    assert_eq!(count(&conn).unwrap(), 0);
    assert!(!delete(&conn, id).unwrap());
}
