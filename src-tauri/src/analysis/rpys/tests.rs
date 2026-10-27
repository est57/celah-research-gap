use super::series::{deviation, peaks, quantile_year};
use super::{compute, RpysParams};
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: &str, year: i32, refs: &[&str]) -> NewPaper {
    NewPaper {
        ext_id: id.into(),
        title: format!("Paper {id}"),
        year: Some(year),
        refs: refs.iter().map(|r| r.to_string()).collect(),
        ..Default::default()
    }
}

#[test]
fn deviation_uses_the_five_year_median() {
    let counts = [1, 2, 9, 2, 1];
    assert_eq!(deviation(&counts), vec![-1, 0, 7, 0, -1]);
    assert_eq!(peaks(&deviation(&counts), 10), vec![2]);
    assert_eq!(deviation(&[4]), vec![0]);
    assert!(peaks(&[0, -1, 0], 10).is_empty(), "only positive deviations are peaks");
}

#[test]
fn quantile_year_skips_the_sparse_tail() {
    let spread = [(1850, 1), (1990, 50), (2000, 49)];
    assert_eq!(quantile_year(&spread, 0.01), Some(1850));
    assert_eq!(quantile_year(&spread, 0.02), Some(1990));
    assert_eq!(quantile_year(&[], 0.01), None);
}

#[test]
fn counts_each_reference_once_per_citing_document() {
    let mut conn = open_memory();
    let lecun = "LECUN Y, 1998, P IEEE, V86, P2278";
    let items = vec![
        paper("a", 2020, &[lecun, "lecun y, 1998, P IEEE, V86, P2278, DOI 10.1109/5.726791", "HE K, 2016, PROC CVPR, P770"]),
        paper("b", 2021, &[lecun, "Krizhevsky A., Imagenet classification, (2012) NIPS", "no year at all"]),
        paper("c", 2022, &[lecun, "Future work, 2030, draft"]),
    ];
    insert_many(&mut conn, &items, "Test").unwrap();

    let r = compute(&conn, &RpysParams { from: 1995, to: 0 }).unwrap();
    assert_eq!((r.documents, r.citing), (3, 3));
    assert_eq!(r.dated, 5, "the duplicate LeCun reference in paper a counts once");
    assert_eq!(r.undated, 2, "no year, and a year after the newest document");
    assert_eq!((r.from, r.to), (1995, 2022));
    let at = |y: u16| r.citations[(y - r.from) as usize];
    assert_eq!((at(1998), at(2012), at(2016)), (3, 1, 1));
    let top = &r.peaks[0];
    assert_eq!((top.year, top.citations, top.deviation), (1998, 3, 3));
    assert_eq!((top.refs[0].label.as_str(), top.refs[0].documents), ("Lecun 1998", 3));
}

#[test]
fn validates_the_year_range() {
    let conn = open_memory();
    assert!(compute(&conn, &RpysParams { from: 2020, to: 2010 }).is_err());
    assert!(compute(&conn, &RpysParams { from: 1500, to: 0 }).is_err());
    let empty = compute(&conn, &RpysParams { from: 0, to: 0 }).unwrap();
    assert!(empty.peaks.is_empty() && empty.dated == 0);
}

#[test]
fn reads_years_outside_dois_and_keys_by_doi() {
    use super::parse::reference;
    assert!(reference(", DOI 10.1016/j.ijtb.2020.01.003", 2025).is_none(), "a year inside a DOI is not a year");
    let a = reference("Lakhani P, 2017, Radiology, DOI 10.1148/radiol.2017162326", 2025).unwrap();
    let b = reference("Lakhani P., Deep learning at chest radiography, (2017) Radiology, doi:10.1148/RADIOL.2017162326", 2025).unwrap();
    assert_eq!((a.year, a.key.as_str(), a.label.as_str()), (2017, "doi:10.1148/radiol.2017162326", "Lakhani 2017"));
    assert_eq!(a.key, b.key, "two formats of one work share the DOI key");
}
