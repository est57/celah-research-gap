use super::papers::NewPaper;
use crate::error::AppResult;
use rusqlite::{params, Connection, OptionalExtension};

/// Lowercase letters and digits only, used to spot the same paper across sources.
pub fn title_key(title: &str) -> String {
    title.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).take(300).collect()
}

/// "Rahman, M." and "rahman m" share a key. Different name orders across
/// sources ("Ahmed Iqbal" vs "Iqbal, Ahmed") are not merged.
pub fn name_key(name: &str) -> String {
    let lower: String = name.chars().map(|c| if c.is_alphanumeric() { c } else { ' ' }).flat_map(char::to_lowercase).collect();
    lower.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Inserts papers with their authors and cited references in one transaction.
/// Returns (inserted, skipped duplicates); see `insert_and_fill`.
pub fn insert_many(conn: &mut Connection, items: &[NewPaper], origin: &str) -> AppResult<(usize, usize)> {
    insert_and_fill(conn, items, origin).map(|(inserted, duplicates, _)| (inserted, duplicates))
}

/// As `insert_many`. A record whose source id, DOI, or title + year already exists is not
/// added again, but the stored copy gets what it lacks (reference list, abstract, keywords);
/// a record merged away as a duplicate earlier is skipped. Returns (inserted, duplicates, completed).
pub fn insert_and_fill(conn: &mut Connection, items: &[NewPaper], origin: &str) -> AppResult<(usize, usize, usize)> {
    let tx = conn.transaction()?;
    let (mut inserted, mut completed) = (0, 0);
    {
        let mut same_title = tx.prepare_cached(
            "SELECT 1 FROM papers WHERE title_key = ?1 AND title_key <> '' AND year IS ?2 LIMIT 1",
        )?;
        let mut merged_away = tx.prepare_cached(
            "SELECT 1 FROM paper_aliases WHERE ext_id = ?1 OR (doi IS NOT NULL AND doi = ?2) LIMIT 1",
        )?;
        let mut insert = tx.prepare_cached(
            "INSERT OR IGNORE INTO papers (ext_id, doi, title, title_key, year, source_name, doc_type,
             cited_by, authors, author_count, has_affiliation, abstract, keywords, keywords_plus,
             ref_count, origin, issn)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
        )?;
        let mut add_ref = tx.prepare_cached("INSERT INTO paper_refs (paper_id, position, raw) VALUES (?1, ?2, ?3)")?;
        let mut add_cite = tx.prepare_cached("INSERT OR IGNORE INTO paper_cites (paper_id, cited) VALUES (?1, ?2)")?;
        let mut add_affil = tx.prepare_cached(
            "INSERT OR IGNORE INTO paper_affiliations (paper_id, name_key, name, country) VALUES (?1, ?2, ?3, ?4)",
        )?;
        let mut alias = tx.prepare_cached("SELECT to_key FROM author_aliases WHERE from_key = ?1")?;
        let mut add_author = tx.prepare_cached(
            "INSERT INTO paper_authors (paper_id, position, name, name_key) VALUES (?1, ?2, ?3, ?4)",
        )?;
        let mut add_country = tx.prepare_cached(
            "INSERT OR IGNORE INTO paper_countries (paper_id, code, corresponding) VALUES (?1, ?2, ?3)",
        )?;
        for p in items {
            let key = title_key(&p.title);
            if merged_away.exists(params![p.ext_id, p.doi])? {
                continue;
            }
            if same_title.exists(params![key, p.year])? {
                if let Some(id) = super::fill::existing(&tx, p)? {
                    completed += usize::from(super::fill::fill(&tx, id, p)?);
                }
                continue;
            }
            let changed = insert.execute(params![
                p.ext_id, p.doi, p.title, key, p.year, p.source_name, p.doc_type, p.cited_by,
                p.authors, p.author_count, p.has_affiliation, p.abstract_text, p.keywords,
                p.keywords_plus, p.ref_count, origin, p.issn
            ])?;
            if changed == 0 {
                // Same source id or DOI as a stored record.
                if let Some(id) = super::fill::existing(&tx, p)? {
                    completed += usize::from(super::fill::fill(&tx, id, p)?);
                }
                continue;
            }
            let id = tx.last_insert_rowid();
            for (pos, raw) in p.refs.iter().enumerate() {
                add_ref.execute(params![id, pos as i64, raw])?;
            }
            for cited in &p.cites {
                add_cite.execute(params![id, cited])?;
            }
            for a in &p.affiliations {
                add_affil.execute(params![id, a.key, a.name, a.country])?;
            }
            for (pos, name) in p.author_names.iter().enumerate() {
                // Names the researcher merged earlier count as the merged author.
                let raw = name_key(name);
                let key: String = alias.query_row(params![raw], |r| r.get(0)).optional()?.unwrap_or(raw);
                add_author.execute(params![id, pos as i64, name, key])?;
            }
            // The corresponding country is stored even when it is missing from the affiliation list.
            let corr = p.corresponding_country.as_deref();
            let extra = corr.filter(|c| !p.countries.iter().any(|x| x == c));
            for code in p.countries.iter().map(String::as_str).chain(extra) {
                add_country.execute(params![id, code, corr == Some(code)])?;
            }
            inserted += 1;
        }
    }
    tx.commit()?;
    Ok((inserted, items.len() - inserted, completed))
}
