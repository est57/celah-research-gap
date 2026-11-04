use super::{links, pending, status, store, target, DocLinks, OaStatus};
use crate::db::insert::insert_many;
use crate::db::{open_memory, papers::NewPaper};
use crate::unpaywall::OaLink;

fn paper(id: &str, doi: Option<&str>, year: i32) -> NewPaper {
    NewPaper { ext_id: id.into(), title: format!("Paper {id}"), doi: doi.map(Into::into), year: Some(year), ..Default::default() }
}

#[test]
fn lookups_are_queued_stored_and_opened_in_order_of_preference() {
    let mut conn = open_memory();
    insert_many(&mut conn, &[paper("a", Some("10.1/a"), 2021), paper("b", Some("10.1/b"), 2023), paper("c", None, 2024)], "Scopus").unwrap();
    let todo = pending(&conn, 10).unwrap();
    assert_eq!(todo.iter().map(|t| t.1.as_str()).collect::<Vec<_>>(), ["10.1/b", "10.1/a"], "only DOIs, newest first");
    let (b, a) = (todo[0].0, todo[1].0);
    store(&conn, b, &OaLink { is_oa: true, pdf_url: Some("https://x.org/b.pdf".into()), url: Some("https://x.org/b".into()), ..Default::default() }).unwrap();
    store(&conn, a, &OaLink::default()).unwrap();
    assert!(pending(&conn, 10).unwrap().is_empty());
    assert_eq!(status(&conn).unwrap(), OaStatus { with_doi: 2, checked: 2, open: 1 });

    assert_eq!(target(&conn, b).unwrap().as_deref(), Some("https://x.org/b.pdf"));
    assert_eq!(target(&conn, a).unwrap().as_deref(), Some("https://doi.org/10.1/a"), "closed works open their DOI page");
    let c: i64 = conn.query_row("SELECT id FROM papers WHERE ext_id = 'c'", [], |r| r.get(0)).unwrap();
    assert_eq!(target(&conn, c).unwrap(), None);
    assert_eq!(target(&conn, 9_999).unwrap(), None);

    let got = links(&conn, &[b, c, 9_999]).unwrap();
    assert_eq!(got, [DocLinks { id: b, open: true, pdf: true, doi: true }, DocLinks { id: c, open: false, pdf: false, doi: false }]);
}
