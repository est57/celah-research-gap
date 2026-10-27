//! Markdown rendering of the report.

use super::cite::Style;
use super::ReportParagraph;

pub fn render(title: &str, intro: &str, paragraphs: &[ReportParagraph], refs_title: &str, refs: &[String], style: Style) -> String {
    let mut out = format!("# {title}\n\n{intro}\n\n");
    for (i, p) in paragraphs.iter().enumerate() {
        out.push_str(&format!("## {}. {}\n\n{}\n\n", i + 1, p.heading, p.text));
    }
    out.push_str(&format!("## {refs_title}\n\n"));
    for r in refs {
        // IEEE entries already carry their number.
        out.push_str(&if style == Style::Apa { format!("- {r}\n") } else { format!("{r}\n\n") });
    }
    out
}
