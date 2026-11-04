//! V18: open-access copies found through Unpaywall, one row per looked-up document.

pub const V18: &str = r#"
CREATE TABLE paper_oa (
    paper_id   INTEGER PRIMARY KEY REFERENCES papers(id) ON DELETE CASCADE,
    is_oa      INTEGER NOT NULL,
    url        TEXT,
    pdf_url    TEXT,
    host       TEXT,
    license    TEXT,
    checked_at TEXT NOT NULL DEFAULT (datetime('now'))
);
"#;
