//! V19: saved searches the researcher re-runs to catch new papers (topic monitor), and
//! the documents each run added.

pub const V19: &str = r#"
CREATE TABLE watches (
    id          INTEGER PRIMARY KEY,
    query       TEXT NOT NULL,
    sources     TEXT NOT NULL,
    doc_type    TEXT,
    year_from   INTEGER NOT NULL,
    max_results INTEGER NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    last_run_at TEXT,
    last_new    INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE watch_hits (
    watch_id INTEGER NOT NULL REFERENCES watches(id) ON DELETE CASCADE,
    paper_id INTEGER NOT NULL REFERENCES papers(id) ON DELETE CASCADE,
    found_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (watch_id, paper_id)
) WITHOUT ROWID;
"#;
