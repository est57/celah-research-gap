//! V17: title and abstract screening, and a log of every import and saved search so
//! the PRISMA flow diagram can count records identified and duplicates removed.

pub const V17: &str = r#"
CREATE TABLE screening (
    paper_id   INTEGER PRIMARY KEY REFERENCES papers(id) ON DELETE CASCADE,
    decision   TEXT NOT NULL CHECK (decision IN ('include', 'exclude')),
    reason     TEXT NOT NULL DEFAULT '',
    note       TEXT NOT NULL DEFAULT '',
    decided_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX screening_decision ON screening(decision);
CREATE TABLE import_log (
    id         INTEGER PRIMARY KEY,
    origin     TEXT NOT NULL,
    fetched    INTEGER NOT NULL,
    inserted   INTEGER NOT NULL,
    duplicates INTEGER NOT NULL,
    at         TEXT NOT NULL DEFAULT (datetime('now'))
);
-- Collections from before this version: what is stored now counts as identified, and the
-- records already merged away count once, under an empty origin, as duplicates removed.
INSERT INTO import_log (origin, fetched, inserted, duplicates)
    SELECT origin, COUNT(*), COUNT(*), 0 FROM papers GROUP BY origin;
INSERT INTO import_log (origin, fetched, inserted, duplicates)
    SELECT '', COUNT(*), 0, 0 FROM paper_aliases HAVING COUNT(*) > 0;
"#;
