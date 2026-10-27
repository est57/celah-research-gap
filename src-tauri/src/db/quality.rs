//! Metadata completeness per import source, and how many documents each analysis can use.

use super::stats::FIELDS;
use crate::error::AppResult;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
pub struct FieldInfo {
    pub tag: &'static str,
    pub label: &'static str,
}

#[derive(Serialize)]
pub struct OriginQuality {
    pub origin: String,
    pub count: i64,
    /// Missing documents per field, in the order of `fields`.
    pub missing: Vec<i64>,
    /// Usable documents per analysis, in the order of `analyses`.
    pub usable: Vec<i64>,
}

#[derive(Serialize)]
pub struct AnalysisInfo {
    pub key: &'static str,
    pub label: &'static str,
    pub needs: &'static [&'static str],
}

#[derive(Serialize)]
pub struct MetadataQuality {
    pub total: i64,
    pub fields: Vec<FieldInfo>,
    pub analyses: Vec<AnalysisInfo>,
    pub origins: Vec<OriginQuality>,
}

/// (key, label, field tags it relies on, condition for a usable document). Fixed literals only.
const ANALYSES: [(&str, &str, &[&str], &str); 7] = [
    ("conceptual", "Peta tematik & tren topik", &["DE", "ID"], "COALESCE(keywords, '') <> '' OR COALESCE(keywords_plus, '') <> ''"),
    ("intellectual", "Ko-sitasi & struktur intelektual", &["CR"], "EXISTS (SELECT 1 FROM paper_refs r WHERE r.paper_id = papers.id)"),
    ("historiograph", "Historiograf (sitasi langsung)", &["CR"], "EXISTS (SELECT 1 FROM paper_cites c WHERE c.paper_id = papers.id) OR EXISTS (SELECT 1 FROM paper_refs r WHERE r.paper_id = papers.id)"),
    ("social", "Jaringan kolaborasi penulis", &["AU"], "author_count > 1"),
    ("countries", "Kolaborasi negara (SCP/MCP)", &["CU"], "EXISTS (SELECT 1 FROM paper_countries c WHERE c.paper_id = papers.id)"),
    ("explicit", "Kalimat celah di abstrak", &["AB"], "COALESCE(abstract, '') <> ''"),
    ("trends", "Produksi tahunan & momentum", &["PY"], "year IS NOT NULL"),
];

fn sum(cond: &str) -> String {
    format!("COALESCE(SUM(CASE WHEN {cond} THEN 1 ELSE 0 END), 0)")
}

pub fn quality(conn: &Connection) -> AppResult<MetadataQuality> {
    let cols: Vec<String> = FIELDS.iter().map(|f| sum(f.2)).chain(ANALYSES.iter().map(|a| sum(a.3))).collect();
    let sql = format!("SELECT origin, COUNT(*), {} FROM papers GROUP BY origin ORDER BY 2 DESC, 1", cols.join(", "));
    let mut stmt = conn.prepare(&sql)?;
    let origins = stmt
        .query_map([], |r| {
            let at = |range: std::ops::Range<usize>| range.map(|i| r.get::<_, i64>(i)).collect::<Result<Vec<_>, _>>();
            let f = FIELDS.len();
            Ok(OriginQuality { origin: r.get(0)?, count: r.get(1)?, missing: at(2..2 + f)?, usable: at(2 + f..2 + f + ANALYSES.len())? })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(MetadataQuality {
        total: origins.iter().map(|o| o.count).sum(),
        fields: FIELDS.iter().map(|&(tag, label, _)| FieldInfo { tag, label }).collect(),
        analyses: ANALYSES.iter().map(|&(key, label, needs, _)| AnalysisInfo { key, label, needs }).collect(),
        origins,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::open_memory;
    use rusqlite::params;

    fn add(conn: &Connection, id: i64, origin: &str, abstract_text: Option<&str>, refs: bool) {
        conn.execute(
            "INSERT INTO papers (id, ext_id, title, year, author_count, abstract, keywords, ref_count, origin)
             VALUES (?1, ?2, 'T', 2024, 2, ?3, 'a; b', 40, ?4)",
            params![id, format!("x:{id}"), abstract_text, origin],
        )
        .unwrap();
        if refs {
            conn.execute("INSERT INTO paper_refs (paper_id, position, raw) VALUES (?1, 0, 'R')", params![id]).unwrap();
        }
    }

    #[test]
    fn counts_missing_and_usable_per_origin() {
        let conn = open_memory();
        add(&conn, 1, "OpenAlex", Some("x"), false);
        add(&conn, 2, "OpenAlex", None, false);
        add(&conn, 3, "Scopus", Some("x"), true);
        let q = quality(&conn).unwrap();
        assert_eq!(q.total, 3);
        let oa = &q.origins[0];
        assert_eq!((oa.origin.as_str(), oa.count), ("OpenAlex", 2));
        let field = |tag: &str| q.fields.iter().position(|f| f.tag == tag).unwrap();
        assert_eq!(oa.missing[field("AB")], 1);
        assert_eq!(oa.missing[field("CR")], 2, "a reference count without stored text is still missing");
        assert_eq!(q.origins[1].missing[field("CR")], 0);
        let usable = |key: &str| q.analyses.iter().position(|a| a.key == key).unwrap();
        assert_eq!(oa.usable[usable("intellectual")], 0);
        assert_eq!(oa.usable[usable("explicit")], 1);
        assert_eq!(oa.usable[usable("conceptual")], 2);
    }
}
