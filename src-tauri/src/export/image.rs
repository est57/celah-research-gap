//! Checks chart images produced by the UI before they are written to a file the user
//! chose: bounded size, the right format, and no active content in SVG.

use crate::error::{AppError, AppResult};

pub const MAX_BYTES: usize = 20 * 1024 * 1024;
const PNG_SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
/// Markup that could run code or load content when the SVG is opened elsewhere.
const FORBIDDEN: [&str; 6] = ["<script", "foreignobject", "javascript:", "<iframe", "<embed", "<object"];

/// Returns the file extension for a valid image.
pub fn check(kind: &str, data: &[u8]) -> AppResult<&'static str> {
    if data.is_empty() || data.len() > MAX_BYTES {
        return Err(AppError::Invalid("ukuran gambar tidak sah".into()));
    }
    match kind {
        "png" if data.starts_with(&PNG_SIGNATURE) => Ok("png"),
        "svg" => {
            let text = std::str::from_utf8(data).map_err(|_| AppError::Invalid("SVG harus berupa teks UTF-8".into()))?;
            let lower = text.to_lowercase();
            let starts = lower.trim_start().starts_with("<svg");
            if !starts || FORBIDDEN.iter().any(|f| lower.contains(f)) || has_event_attribute(&lower) {
                return Err(AppError::Invalid("isi SVG tidak diizinkan".into()));
            }
            Ok("svg")
        }
        _ => Err(AppError::Invalid("format gambar tidak dikenal".into())),
    }
}

/// An inline event handler such as onclick= or onload= after any whitespace.
fn has_event_attribute(lower: &str) -> bool {
    let bytes = lower.as_bytes();
    lower.match_indices("on").any(|(i, _)| {
        let after_space = i > 0 && bytes[i - 1].is_ascii_whitespace();
        let rest = &lower[i + 2..];
        let name_len = rest.bytes().take_while(u8::is_ascii_alphabetic).count();
        after_space && name_len > 0 && rest[name_len..].trim_start().starts_with('=')
    })
}

/// "Jaringan ko-sitasi" -> "jaringan-ko-sitasi"; empty falls back to "grafik".
pub fn file_stem(raw: &str) -> String {
    let lower = raw.to_lowercase();
    let words: Vec<&str> = lower.split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| !w.is_empty()).collect();
    let stem: String = words.join("-").chars().take(60).collect();
    if stem.is_empty() { "grafik".into() } else { stem }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_plain_images_and_refuses_active_content() {
        let png = [&PNG_SIGNATURE[..], b"rest"].concat();
        assert_eq!(check("png", &png).unwrap(), "png");
        assert!(check("png", b"<svg/>").is_err());
        assert_eq!(check("svg", br#"<svg xmlns="http://www.w3.org/2000/svg"><text style="fill: red">only</text></svg>"#).unwrap(), "svg");
        assert!(check("svg", b"<svg><script>alert(1)</script></svg>").is_err());
        assert!(check("svg", b"<svg><foreignObject/></svg>").is_err());
        assert!(check("svg", br#"<svg><a href="javascript:x">a</a></svg>"#).is_err());
        assert!(check("svg", b"<svg><rect onload = 'x'/></svg>").is_err());
        assert!(check("svg", b"<svg><rect\n\tonclick='x'/></svg>").is_err(), "newline or tab before the handler");
        assert!(check("svg", br#"<svg><text>Kolaborasi on top</text></svg>"#).is_ok(), "the word 'on' in text is fine");
        assert!(check("svg", b"<html></html>").is_err());
        assert!(check("gif", &png).is_err());
        assert_eq!(file_stem("Jaringan ko-sitasi (2024)"), "jaringan-ko-sitasi-2024");
        assert_eq!(file_stem("../../etc"), "etc");
        assert_eq!(file_stem("!!!"), "grafik");
    }
}
