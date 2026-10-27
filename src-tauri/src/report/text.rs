//! Sentence templates for the draft research-gap paragraphs (Indonesian and English).

use crate::analysis::gaps::model::Kind;
use crate::db::checks::CheckSummary;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    Id,
    En,
}

pub fn intro(lang: Lang, docs: usize, field: &str, gaps: usize) -> String {
    match lang {
        Lang::Id => format!(
            "Analisis bibliometrik terhadap {docs} dokumen, dengan {} sebagai dasar analisis konseptual, menunjukkan {gaps} celah penelitian yang layak dikaji lebih lanjut.",
            field.to_lowercase()
        ),
        Lang::En => format!(
            "A bibliometric analysis of {docs} documents, using {} for the conceptual analysis, points to {gaps} research gaps that merit further study.",
            if field.contains("Plus") { "Keywords Plus" } else { "author keywords" }
        ),
    }
}

/// Opening claim of a gap, with its main citations already formatted, e.g. "(Rahman et al., 2023)".
/// `variant` alternates the wording so consecutive paragraphs do not read the same.
pub fn claim(lang: Lang, kind: Kind, terms: &[String], cites: &str, variant: usize) -> String {
    let a = terms.first().map(String::as_str).unwrap_or("");
    let b = terms.get(1).map(String::as_str).unwrap_or("");
    let c = if cites.is_empty() { String::new() } else { format!(" {cites}") };
    let alt = variant % 2 == 1;
    match (lang, kind, alt) {
        (Lang::Id, Kind::Matrix, false) => format!("Meskipun {a} dan {b} sama-sama banyak diteliti, studi yang menggabungkan keduanya masih jarang{c}."),
        (Lang::Id, Kind::Matrix, true) => format!("Kajian tentang {a} dan {b} cukup banyak, tetapi keduanya jarang dibahas dalam satu studi{c}."),
        (Lang::Id, Kind::Emerging, false) => format!("Topik {a} sedang tumbuh dalam beberapa tahun terakhir, tetapi masih jarang dikaitkan dengan {b}{c}."),
        (Lang::Id, Kind::Emerging, true) => format!("Minat terhadap {a} meningkat, namun kaitannya dengan {b} belum banyak dieksplorasi{c}."),
        (Lang::Id, Kind::Niche, false) => format!("Penelitian tentang {a} telah berkembang, tetapi cenderung terpisah dari kajian {b}{c}."),
        (Lang::Id, Kind::Niche, true) => format!("Kajian {a} sudah cukup matang di dalam kelompoknya sendiri, tetapi jarang bersinggungan dengan {b}{c}."),
        (Lang::Id, Kind::Structural, _) => format!("Penelitian tentang {a} dan tentang {b} berkembang relatif terpisah dan jarang saling merujuk{c}."),
        (Lang::Id, Kind::Idea, _) => format!("Kajian tentang {a} dan tentang {b} sama-sama tersedia, tetapi studi yang menggabungkan keduanya masih sangat terbatas{c}."),
        (Lang::Id, Kind::Front, _) => format!("Kelompok studi tentang {a} banyak disitasi, tetapi jarang merujuk literatur yang sama dengan front utama tentang {b}{c}."),
        (Lang::Id, Kind::Explicit, _) => format!("Sejumlah penulis secara eksplisit menyatakan bahwa {a} masih memerlukan penelitian lanjutan{c}."),
        (Lang::En, Kind::Matrix, false) => format!("Although {a} and {b} are both widely studied, research combining the two remains scarce{c}."),
        (Lang::En, Kind::Matrix, true) => format!("{} and {b} each have a substantial literature, yet they are rarely examined together{c}.", capitalize(a)),
        (Lang::En, Kind::Emerging, false) => format!("{} has grown in recent years but is still rarely connected to {b}{c}.", capitalize(a)),
        (Lang::En, Kind::Emerging, true) => format!("Interest in {a} is rising, but its relation to {b} remains largely unexplored{c}."),
        (Lang::En, Kind::Niche, false) => format!("Research on {a} has matured, yet it remains largely separate from work on {b}{c}."),
        (Lang::En, Kind::Niche, true) => format!("Work on {a} is well developed within its own community but seldom intersects with {b}{c}."),
        (Lang::En, Kind::Structural, _) => format!("Research on {a} and on {b} has developed largely in parallel, with little cross-citation{c}."),
        (Lang::En, Kind::Idea, _) => format!("Studies on {a} and on {b} are both available, yet work combining the two remains very limited{c}."),
        (Lang::En, Kind::Front, _) => format!("Studies on {a} are highly cited, yet they share few references with the main research front on {b}{c}."),
        (Lang::En, Kind::Explicit, _) => format!("Several authors explicitly state that {a} still requires further research{c}."),
    }
}

/// English section heading from the gap's two main terms.
pub fn heading_en(terms: &[String]) -> String {
    let parts: Vec<&str> = terms.iter().take(2).map(String::as_str).collect();
    capitalize(&parts.join(" and "))
}

/// Mentions the closest existing studies, if any.
pub fn closest(lang: Lang, cites: &str) -> Option<String> {
    (!cites.is_empty()).then(|| match lang {
        Lang::Id => format!("Studi yang paling dekat {cites} belum menjawab persoalan ini secara langsung."),
        Lang::En => format!("The closest studies {cites} do not address this question directly."),
    })
}

pub fn verification(lang: Lang, check: &CheckSummary) -> String {
    let m = check.matching;
    match (lang, check.verdict) {
        (Lang::Id, "open") => "Penelusuran literatur terbaru di luar korpus ini tidak menemukan studi yang menggabungkan seluruh aspek tersebut.".into(),
        (Lang::Id, "new") => format!("Penelusuran literatur terbaru menemukan {m} studi yang mulai menyentuh aspek ini dan perlu dikaji lebih lanjut."),
        (Lang::Id, _) => format!("Namun, penelusuran literatur terbaru menemukan {m} studi yang mulai membahas hal ini, sehingga celah perlu dirumuskan lebih spesifik."),
        (Lang::En, "open") => "A search of recent literature outside this corpus found no study combining all of these aspects.".into(),
        (Lang::En, "new") => format!("A search of recent literature found {m} studies that begin to touch on this and should be examined."),
        (Lang::En, _) => format!("However, a search of recent literature found {m} studies beginning to address this, so the gap should be framed more narrowly."),
    }
}

/// Closing sentence that turns the gap into a research question.
pub fn question(lang: Lang, kind: Kind, terms: &[String]) -> String {
    let a = terms.first().map(String::as_str).unwrap_or("");
    let b = terms.get(1).map(String::as_str).unwrap_or("");
    match (lang, kind) {
        (Lang::Id, Kind::Matrix | Kind::Structural | Kind::Idea) => format!("Oleh karena itu, penelitian ini dapat diarahkan untuk menjawab bagaimana temuan dan pendekatan pada {a} dapat diterapkan untuk {b}."),
        (Lang::Id, Kind::Emerging | Kind::Niche | Kind::Front) => format!("Oleh karena itu, penelitian ini dapat diarahkan untuk menjawab bagaimana {a} dapat dikaitkan dengan {b}."),
        (Lang::Id, Kind::Explicit) => format!("Oleh karena itu, penelitian ini dapat diarahkan untuk menjawab pertanyaan terbuka tentang {a} yang disebutkan dalam studi-studi tersebut."),
        (Lang::En, Kind::Matrix | Kind::Structural | Kind::Idea) => format!("This study therefore asks how findings and approaches from {a} can be applied to {b}."),
        (Lang::En, Kind::Emerging | Kind::Niche | Kind::Front) => format!("This study therefore asks how {a} can be linked to {b}."),
        (Lang::En, Kind::Explicit) => format!("This study therefore addresses the open questions on {a} raised in these works."),
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().map(|f| f.to_uppercase().chain(chars).collect()).unwrap_or_default()
}
