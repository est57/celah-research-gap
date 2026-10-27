//! Minimal BibTeX reader: entries, keys and `name = {value}` / "value" / bare fields.
//! Scanning works on bytes; every delimiter is ASCII, so slices stay valid UTF-8.

use std::collections::HashMap;

pub struct Entry {
    /// Entry type such as "article" or "inproceedings".
    pub kind: String,
    pub fields: HashMap<String, String>,
}

struct Cursor<'a> {
    s: &'a str,
    b: &'a [u8],
    i: usize,
}

impl<'a> Cursor<'a> {
    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }
    fn skip_ws(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_whitespace()) {
            self.i += 1;
        }
    }
    fn take_while(&mut self, f: impl Fn(u8) -> bool) -> &'a str {
        let start = self.i;
        while self.peek().is_some_and(&f) {
            self.i += 1;
        }
        &self.s[start..self.i]
    }
    /// Reads `{...}` with nested braces, or `"..."`, or a bare token.
    fn value(&mut self, close: u8) -> &'a str {
        match self.peek() {
            Some(b'{') => {
                let start = self.i + 1;
                let mut depth = 0usize;
                while let Some(c) = self.peek() {
                    self.i += 1;
                    match c {
                        b'{' => depth += 1,
                        b'}' => {
                            depth -= 1;
                            if depth == 0 {
                                return &self.s[start..self.i - 1];
                            }
                        }
                        _ => {}
                    }
                }
                &self.s[start..self.i]
            }
            Some(b'"') => {
                self.i += 1;
                let v = self.take_while(|c| c != b'"');
                self.i += 1;
                v
            }
            _ => self.take_while(|c| c != b',' && c != close).trim(),
        }
    }
}

fn tidy(raw: &str) -> String {
    raw.replace("\\&", "&").replace("\\%", "%").replace("\\_", "_").replace(['{', '}'], "")
}

pub fn parse(text: &str, max_entries: usize) -> Vec<Entry> {
    let mut c = Cursor { s: text, b: text.as_bytes(), i: 0 };
    let mut out = Vec::new();
    while out.len() < max_entries {
        let Some(at) = c.b[c.i..].iter().position(|&x| x == b'@') else { break };
        c.i += at + 1;
        let kind = c.take_while(|x| x.is_ascii_alphabetic()).to_lowercase();
        c.skip_ws();
        let close = match c.peek() {
            Some(b'{') => b'}',
            Some(b'(') => b')',
            _ => continue,
        };
        c.i += 1;
        if matches!(kind.as_str(), "comment" | "preamble" | "string") {
            continue;
        }
        // Citation key: not needed, skipped.
        c.take_while(|x| x != b',' && x != close);
        let mut fields = HashMap::new();
        loop {
            c.skip_ws();
            while c.peek() == Some(b',') {
                c.i += 1;
                c.skip_ws();
            }
            if c.peek().is_none_or(|x| x == close) {
                c.i += 1;
                break;
            }
            let name = c.take_while(|x| x.is_ascii_alphanumeric() || x == b'_' || x == b'-').to_lowercase();
            c.skip_ws();
            if name.is_empty() || c.peek() != Some(b'=') {
                break;
            }
            c.i += 1;
            c.skip_ws();
            let value = c.value(close);
            fields.insert(name, tidy(value));
        }
        out.push(Entry { kind, fields });
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn reads_nested_braces_and_quotes() {
        let src = "@comment{x}\n@ARTICLE{Key1,\n author = {Okafor, C. and Silva, R.},\n\
            title = {{Cough} audio \\& TB},\n year = 2024,\n note = \"Cited by: 5\",\n\
            correspondence_address = {a@b.org}\n}\n@misc(Key2, title={Two})";
        let entries = super::parse(src, 10);
        assert_eq!(entries.len(), 2);
        let e = &entries[0];
        assert_eq!(e.kind, "article");
        assert_eq!(e.fields["title"], "Cough audio & TB");
        assert_eq!(e.fields["year"], "2024");
        assert_eq!(e.fields["note"], "Cited by: 5");
        assert_eq!(entries[1].fields["title"], "Two");
    }
}
