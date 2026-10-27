use super::{compute, Level};
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: usize, author: &str, source: &str, refs: &[&str]) -> NewPaper {
    NewPaper {
        ext_id: format!("p{id}"),
        title: format!("Paper {id}"),
        year: Some(2020 + id as i32),
        author_names: vec![author.to_string()],
        source_name: Some(source.into()),
        refs: refs.iter().map(|r| r.to_string()).collect(),
        ..Default::default()
    }
}

#[test]
fn links_documents_that_cite_the_same_works() {
    let mut conn = open_memory();
    let qin = "QIN ZZ, 2019, SCI REP, V9, DOI 10.1038/s41598-019-51503-3";
    let qin_scopus = "Qin Z.Z., Can AI read chest X-rays?, (2019) Sci Rep, doi:10.1038/S41598-019-51503-3";
    let lakhani = "LAKHANI P, 2017, RADIOLOGY, V284";
    let botha = "BOTHA GHR, 2018, PHYSIOL MEAS, V39";
    let items = vec![
        paper(0, "Chen L", "Sci Rep", &[qin, lakhani]),
        paper(1, "Rahman M", "PLOS ONE", &[qin_scopus, lakhani]),
        paper(2, "Okafor C", "PLOS ONE", &[botha]),
        paper(3, "Silva R", "Physiol Meas", &[botha, "only here, 2011, x"]),
    ];
    insert_many(&mut conn, &items, "Test").unwrap();

    let (coupled, net) = compute(&conn, Level::Documents, 20).unwrap();
    assert_eq!(coupled, 4);
    assert_eq!(net.nodes.len(), 4);
    let chen = net.nodes.iter().position(|n| n.label == "Chen 2020").unwrap();
    assert_eq!(net.nodes[chen].count, 2, "two shared works; the DOI joins both spellings of Qin 2019");
    let edge = net.edges.iter().find(|e| e.from == chen || e.to == chen).unwrap();
    assert_eq!(edge.count, 2);
    assert_eq!(net.edges.len(), 2, "the CAD pair and the audio pair are never linked");
    assert_eq!(net.clusters.len(), 2);

    let (_, sources) = compute(&conn, Level::Sources, 20).unwrap();
    assert_eq!(sources.nodes.len(), 3, "PLOS ONE counts once");
}

#[test]
fn uses_openalex_cited_works() {
    let mut conn = open_memory();
    insert_many(&mut conn, &[paper(0, "Chen L", "A", &[]), paper(1, "Lee J", "B", &[])], "Test").unwrap();
    conn.execute_batch(
        "INSERT INTO paper_cites (paper_id, cited) SELECT id, 'openalex:W1' FROM papers;
         INSERT INTO paper_cites (paper_id, cited) SELECT id, 'openalex:W2' FROM papers WHERE ext_id = 'p0';",
    )
    .unwrap();
    let (coupled, net) = compute(&conn, Level::Authors, 20).unwrap();
    assert_eq!((coupled, net.edges.len()), (2, 1));
}
