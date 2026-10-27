//! Migrations from V7 on (split from schema.rs to keep files short). Never edit a
//! migration that has shipped; append a new one instead.

/// Ids of records merged away as duplicates, so re-imports skip them, and pairs
/// the researcher marked as not duplicates.
pub const V7: &str = r#"
CREATE TABLE paper_aliases (
    ext_id   TEXT PRIMARY KEY,
    doi      TEXT,
    paper_id INTEGER NOT NULL REFERENCES papers(id) ON DELETE CASCADE
);
CREATE INDEX paper_aliases_doi ON paper_aliases(doi) WHERE doi IS NOT NULL;
CREATE TABLE dup_dismissed (
    a INTEGER NOT NULL REFERENCES papers(id) ON DELETE CASCADE,
    b INTEGER NOT NULL REFERENCES papers(id) ON DELETE CASCADE,
    PRIMARY KEY (a, b)
) WITHOUT ROWID;
"#;

/// The collection filter applied to every analysis (single row, JSON of CollectionFilter).
pub const V8: &str = r#"
CREATE TABLE collection_filter (
    id   INTEGER PRIMARY KEY CHECK (id = 1),
    json TEXT NOT NULL
);
"#;

/// Title of a reviewed gap at review time, so the review history stays readable
/// after the gap drops out of the current analysis.
pub const V9: &str = "ALTER TABLE gap_reviews ADD COLUMN title TEXT NOT NULL DEFAULT '';";

/// Works a paper cites, by source id (OpenAlex "referenced_works" as "openalex:W..."),
/// used to find direct citations inside the collection. Kept apart from paper_refs,
/// which holds reference text for co-citation.
pub const V10: &str = r#"
CREATE TABLE paper_cites (
    paper_id INTEGER NOT NULL REFERENCES papers(id) ON DELETE CASCADE,
    cited    TEXT NOT NULL,
    PRIMARY KEY (paper_id, cited)
) WITHOUT ROWID;
CREATE INDEX paper_cites_cited ON paper_cites(cited);
"#;

/// Documents whose cited works were already requested from OpenAlex, so a document
/// without any is not requested again on every run.
pub const V11: &str = r#"
CREATE TABLE refs_checked (
    paper_id   INTEGER PRIMARY KEY REFERENCES papers(id) ON DELETE CASCADE,
    checked_at TEXT NOT NULL DEFAULT (datetime('now'))
);
"#;

/// Institutions of each paper. Lookups done before this version did not fetch them,
/// so the lookup log is cleared once to let "Lengkapi rujukan" fill them in.
pub const V12: &str = r#"
CREATE TABLE paper_affiliations (
    paper_id INTEGER NOT NULL REFERENCES papers(id) ON DELETE CASCADE,
    name_key TEXT NOT NULL,
    name     TEXT NOT NULL,
    country  TEXT,
    PRIMARY KEY (paper_id, name_key)
) WITHOUT ROWID;
CREATE INDEX paper_affiliations_key ON paper_affiliations(name_key);
DELETE FROM refs_checked;
"#;

/// Author names the researcher merged: from_key now counts as to_key. Names as printed
/// are kept for display; imports apply the mapping too.
pub const V13: &str = r#"
CREATE TABLE author_aliases (
    from_key  TEXT PRIMARY KEY,
    from_name TEXT NOT NULL,
    to_key    TEXT NOT NULL,
    to_name   TEXT NOT NULL
) WITHOUT ROWID;
CREATE INDEX author_aliases_to ON author_aliases(to_key);
"#;

/// Keywords the researcher declared synonyms: from_key counts as to_key in analyses.
/// The stored keyword text is never rewritten; the analysis connection applies this.
pub const V14: &str = r#"
CREATE TABLE keyword_synonyms (
    from_key   TEXT PRIMARY KEY,
    from_label TEXT NOT NULL,
    to_key     TEXT NOT NULL,
    to_label   TEXT NOT NULL
) WITHOUT ROWID;
"#;

/// Gap ideas the researcher tested and saved; Gap Explorer recomputes them as candidates.
pub const V15: &str = r#"
CREATE TABLE gap_ideas (
    gap_key    TEXT PRIMARY KEY,
    concept_a  TEXT NOT NULL,
    concept_b  TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
"#;
