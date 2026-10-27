use super::*;
use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

fn paper(id: &str, authors: &[&str], source: &str) -> NewPaper {
    NewPaper { ext_id: id.into(), title: id.into(), source_name: Some(source.into()), author_names: authors.iter().map(|a| a.to_string()).collect(), ..Default::default() }
}

#[test]
fn groups_compatible_variants_with_evidence() {
    let mut conn = open_memory();
    let items = [
        paper("a", &["Rahman M", "Chen L"], "Radiology"),
        paper("b", &["Muhammad Rahman", "Chen L"], "Radiology"),
        paper("c", &["Rahman MZ", "Okafor C"], "PLOS ONE"),
        paper("d", &["Rahman MA"], "IEEE Access"),
        paper("e", &["Silva R"], "IEEE Access"),
        paper("f", &["Liu, Yanyan", "Liu, Ying"], "Radiology"),
        paper("g", &["Ying Liu"], "PLOS ONE"),
        paper("h", &["Sameer Antani", "Stefan Jaeger"], "Radiology"),
        paper("i", &["Antani, Sameer", "Jaeger, Stefan"], "PLOS ONE"),
    ];
    insert_many(&mut conn, &items, "Test").unwrap();
    let s = suggest(&conn).unwrap();
    let groups: Vec<Vec<&str>> = s.iter().map(|g| { let mut n: Vec<&str> = g.variants.iter().map(|v| v.name.as_str()).collect(); n.sort(); n }).collect();
    let rahman = groups.iter().find(|g| g.contains(&"Muhammad Rahman")).unwrap();
    assert!(rahman.contains(&"Rahman M"), "{groups:?}");
    assert!(!(rahman.contains(&"Rahman MA") && rahman.contains(&"Rahman MZ")), "conflicting initials never together");
    let liu = groups.iter().find(|g| g.iter().any(|n| n.contains("Liu"))).expect("a Liu suggestion");
    assert_eq!(liu.len(), 2, "Yanyan and Ying share a paper and differ in first name: {liu:?}");
    assert!(liu.contains(&"Ying Liu") && liu.contains(&"Liu, Ying"));
    let r = s.iter().find(|g| g.variants.iter().any(|v| v.name == "Muhammad Rahman")).unwrap();
    assert!(r.shared_coauthors >= 1 && r.shared_sources >= 1, "Chen L and Radiology appear with both");
    let antani = s.iter().find(|g| g.variants.iter().any(|v| v.name == "Sameer Antani")).unwrap();
    assert!(antani.reordered);
    assert_eq!(antani.shared_coauthors, 1, "Jaeger is matched across name orders");
    assert!(!r.reordered);
}
