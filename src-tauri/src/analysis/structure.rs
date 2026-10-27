//! Intellectual structure (reference co-citation), social structure (author collaboration)
//! and the keyword co-occurrence network of the conceptual structure.

use super::coupling::{self, Level};
use super::cooccur::best_field;
use super::network::{self, Network};
use super::terms_with_label;
use super::refkey::ref_key;
use crate::error::{AppError, AppResult};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StructureParams {
    /// "references" (co-citation), "authors" (collaboration), "keywords" (co-occurrence),
    /// or "coupling_documents" / "coupling_sources" / "coupling_authors" (bibliographic coupling).
    pub unit: String,
    pub nodes: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
    pub unit: &'static str,
    /// Documents that contributed at least one item.
    pub documents: usize,
    pub network: Network,
}

type Docs = (Vec<Vec<String>>, HashMap<String, String>);

fn grouped(rows: Vec<(i64, String, String)>) -> Docs {
    let mut per_doc: BTreeMap<i64, Vec<String>> = BTreeMap::new();
    let mut labels = HashMap::new();
    for (doc, key, label) in rows {
        labels.entry(key.clone()).or_insert(label);
        per_doc.entry(doc).or_default().push(key);
    }
    (per_doc.into_values().collect(), labels)
}

fn references(conn: &Connection) -> AppResult<Docs> {
    let mut stmt = conn.prepare("SELECT paper_id, raw FROM paper_refs")?;
    let rows = stmt
        .query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(grouped(rows.into_iter().filter_map(|(doc, raw)| ref_key(&raw).map(|(k, l)| (doc, k, l))).collect()))
}

fn authors(conn: &Connection) -> AppResult<Docs> {
    let mut stmt = conn.prepare("SELECT paper_id, name_key, name FROM paper_authors")?;
    let rows = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(grouped(rows))
}

/// Keywords of the field with more filled-in documents, normalized as in the thematic map.
fn keywords(conn: &Connection) -> AppResult<(&'static str, Docs)> {
    let field = best_field(conn)?;
    // The column name comes from the Field enum, never from user input.
    let mut stmt = conn.prepare(&format!("SELECT id, {} FROM papers", field.column()))?;
    let rows = stmt
        .query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, Option<String>>(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    let mut flat = Vec::new();
    for (doc, raw) in rows {
        flat.extend(terms_with_label(raw.as_deref()).map(|(k, l)| (doc, k, l)));
    }
    Ok((field.label(), grouped(flat)))
}

pub fn compute(conn: &Connection, p: &StructureParams) -> AppResult<Structure> {
    if !(10..=150).contains(&p.nodes) {
        return Err(AppError::Invalid("jumlah node harus 10 sampai 150".into()));
    }
    let coupling = match p.unit.as_str() {
        "coupling_documents" => Some(Level::Documents),
        "coupling_sources" => Some(Level::Sources),
        "coupling_authors" => Some(Level::Authors),
        _ => None,
    };
    if let Some(level) = coupling {
        let (documents, network) = coupling::compute(conn, level, p.nodes as usize)?;
        return Ok(Structure { unit: level.label(), documents, network });
    }
    let (unit, (docs, labels)) = match p.unit.as_str() {
        "references" => ("Referensi tersitasi", references(conn)?),
        "authors" => ("Penulis", authors(conn)?),
        "keywords" => keywords(conn)?,
        _ => return Err(AppError::Invalid("unit analisis tidak dikenal".into())),
    };
    Ok(Structure { unit, documents: docs.len(), network: network::build(&docs, &labels, p.nodes as usize) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{insert::insert_many, open_memory, papers::NewPaper};

    fn paper(id: usize, authors: &[&str], refs: &[&str]) -> NewPaper {
        NewPaper {
            ext_id: format!("p{id}"),
            title: format!("Paper {id}"),
            year: Some(2020),
            author_names: authors.iter().map(|a| a.to_string()).collect(),
            refs: refs.iter().map(|r| r.to_string()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn builds_cocitation_and_collaboration_networks() {
        let mut conn = open_memory();
        let cad = ["QIN ZZ, 2019, SCI REP, V9", "LAKHANI P, 2017, RADIOLOGY, V284", "JAEGER S, 2014, QUANT IMAG MED SURG"];
        let audio = ["BOTHA GHR, 2018, PHYSIOL MEAS, V39", "PAHAR M, 2021, COMPUT BIOL MED"];
        let mut items = Vec::new();
        for i in 0..4 {
            items.push(paper(i, &["Chen L", "Rahman M"], &cad));
            items.push(paper(10 + i, &["Okafor C", "Silva R"], &audio));
        }
        insert_many(&mut conn, &items, "Test").unwrap();

        let refs = compute(&conn, &StructureParams { unit: "references".into(), nodes: 20 }).unwrap();
        assert_eq!(refs.network.nodes.len(), 5);
        assert_eq!(refs.network.clusters.len(), 2, "CAD and audio references never appear together");
        assert!(refs.network.nodes.iter().any(|n| n.label == "Qin 2019" && n.count == 4));

        conn.execute("UPDATE papers SET keywords = CASE WHEN ext_id LIKE 'p1%' THEN 'cough; audio' ELSE 'chest x-ray; CAD' END", []).unwrap();
        let words = compute(&conn, &StructureParams { unit: "keywords".into(), nodes: 20 }).unwrap();
        assert_eq!((words.network.nodes.len(), words.network.clusters.len()), (4, 2));

        let people = compute(&conn, &StructureParams { unit: "authors".into(), nodes: 20 }).unwrap();
        assert_eq!(people.network.edges.len(), 2);
        assert_eq!(people.documents, 8);
    }

    #[test]
    fn rejects_bad_params() {
        let conn = open_memory();
        assert!(compute(&conn, &StructureParams { unit: "references".into(), nodes: 5 }).is_err());
        assert!(compute(&conn, &StructureParams { unit: "papers; --".into(), nodes: 20 }).is_err());
    }
}
