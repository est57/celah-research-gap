//! Ordered schema migrations. Index i is applied when PRAGMA user_version == i.
//! Never edit a migration that has shipped; append a new one instead.

const V1: &str = r#"
CREATE TABLE IF NOT EXISTS papers (
    id              INTEGER PRIMARY KEY,
    openalex_id     TEXT NOT NULL UNIQUE,
    doi             TEXT,
    title           TEXT NOT NULL,
    year            INTEGER,
    source_name     TEXT,
    doc_type        TEXT,
    cited_by        INTEGER NOT NULL DEFAULT 0,
    authors         TEXT NOT NULL DEFAULT '',
    author_count    INTEGER NOT NULL DEFAULT 0,
    has_affiliation INTEGER NOT NULL DEFAULT 0,
    abstract        TEXT,
    keywords        TEXT,
    ref_count       INTEGER NOT NULL DEFAULT 0,
    origin          TEXT NOT NULL,
    imported_at     TEXT NOT NULL DEFAULT (datetime('now'))
);
"#;

/// Generic source id (e.g. "openalex:W1", "wos:WOS:000...") plus cited references.
const V2: &str = r#"
CREATE TABLE papers_v2 (
    id              INTEGER PRIMARY KEY,
    ext_id          TEXT NOT NULL UNIQUE,
    doi             TEXT,
    title           TEXT NOT NULL,
    title_key       TEXT NOT NULL DEFAULT '',
    year            INTEGER,
    source_name     TEXT,
    doc_type        TEXT,
    cited_by        INTEGER NOT NULL DEFAULT 0,
    authors         TEXT NOT NULL DEFAULT '',
    author_count    INTEGER NOT NULL DEFAULT 0,
    has_affiliation INTEGER NOT NULL DEFAULT 0,
    abstract        TEXT,
    keywords        TEXT,
    keywords_plus   TEXT,
    ref_count       INTEGER NOT NULL DEFAULT 0,
    origin          TEXT NOT NULL,
    imported_at     TEXT NOT NULL DEFAULT (datetime('now'))
);
INSERT INTO papers_v2 (id, ext_id, doi, title, year, source_name, doc_type, cited_by, authors,
    author_count, has_affiliation, abstract, keywords, ref_count, origin, imported_at)
SELECT id, 'openalex:' || openalex_id, doi, title, year, source_name, doc_type, cited_by, authors,
    author_count, has_affiliation, abstract, keywords, ref_count, origin, imported_at FROM papers;
DROP TABLE papers;
ALTER TABLE papers_v2 RENAME TO papers;
CREATE UNIQUE INDEX papers_doi ON papers(doi) WHERE doi IS NOT NULL;
CREATE INDEX papers_year ON papers(year);
CREATE INDEX papers_cited ON papers(cited_by DESC);
CREATE INDEX papers_title_key ON papers(title_key, year);
CREATE TABLE paper_refs (
    paper_id INTEGER NOT NULL REFERENCES papers(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    raw      TEXT NOT NULL,
    PRIMARY KEY (paper_id, position)
) WITHOUT ROWID;
"#;

/// Full author lists. `name_key` is a normalized form used for counting.
const V3: &str = r#"
CREATE TABLE paper_authors (
    paper_id INTEGER NOT NULL REFERENCES papers(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    name     TEXT NOT NULL,
    name_key TEXT NOT NULL,
    PRIMARY KEY (paper_id, position)
) WITHOUT ROWID;
CREATE INDEX paper_authors_key ON paper_authors(name_key);
"#;

/// Countries of a paper's author affiliations (ISO alpha-2). `corresponding`
/// marks the corresponding author's country, used for SCP/MCP.
const V4: &str = r#"
CREATE TABLE paper_countries (
    paper_id      INTEGER NOT NULL REFERENCES papers(id) ON DELETE CASCADE,
    code          TEXT NOT NULL,
    corresponding INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (paper_id, code)
) WITHOUT ROWID;
CREATE INDEX paper_countries_code ON paper_countries(code);
"#;

/// Researcher decisions on gap candidates, keyed by the gap's stable key.
const V5: &str = r#"
CREATE TABLE gap_reviews (
    gap_key    TEXT PRIMARY KEY,
    status     TEXT NOT NULL CHECK (status IN ('accepted', 'check', 'rejected')),
    note       TEXT NOT NULL DEFAULT '',
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
"#;

/// Latest verification of a gap against recent literature not in the collection.
const V6: &str = r#"
CREATE TABLE gap_checks (
    gap_key    TEXT PRIMARY KEY,
    checked_at TEXT NOT NULL DEFAULT (datetime('now')),
    query      TEXT NOT NULL,
    found      INTEGER NOT NULL,
    matching   INTEGER NOT NULL
);
CREATE TABLE gap_check_papers (
    gap_key  TEXT NOT NULL REFERENCES gap_checks(gap_key) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    title    TEXT NOT NULL,
    year     INTEGER,
    doi      TEXT,
    source   TEXT,
    cited_by INTEGER NOT NULL DEFAULT 0,
    matches  INTEGER NOT NULL,
    PRIMARY KEY (gap_key, position)
) WITHOUT ROWID;
"#;

use super::schema_late::{V10, V11, V12, V13, V14, V15, V7, V8, V9};

pub const MIGRATIONS: [&str; 15] = [V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15];
