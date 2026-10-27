//! Upper bounds for imported data, so a malformed or hostile file cannot
//! exhaust memory or bloat the database.

pub const MAX_FILE_BYTES: u64 = 50 * 1024 * 1024;
pub const MAX_RECORDS: usize = 20_000;
pub const MAX_REFS: usize = 3_000;
pub const MAX_REF_LEN: usize = 1_000;
pub const MAX_TITLE: usize = 1_000;
pub const MAX_ABSTRACT: usize = 20_000;
pub const MAX_FIELD: usize = 500;
