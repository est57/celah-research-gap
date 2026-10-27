//! Sentences of the methods and results draft, in Indonesian and English.

use super::countries;
use super::facts::Facts;
use crate::report::text::Lang;

fn dec(lang: Lang, v: f64, digits: usize) -> String {
    let s = format!("{v:.digits$}");
    if lang == Lang::Id { s.replace('.', ",") } else { s }
}

/// "A, B, dan C" / "A, B, and C".
fn list(lang: Lang, items: &[String]) -> String {
    let and = if lang == Lang::Id { "dan" } else { "and" };
    match items {
        [] => String::new(),
        [a] => a.clone(),
        [a, b] => format!("{a} {and} {b}"),
        [rest @ .., last] => format!("{}, {and} {last}", rest.join(", ")),
    }
}

fn years(f: &Facts) -> String {
    match (f.info.year_min, f.info.year_max) {
        (Some(a), Some(b)) if a != b => format!("{a}-{b}"),
        (Some(a), _) => a.to_string(),
        _ => "-".into(),
    }
}

pub fn method(lang: Lang, f: &Facts, tools: &str) -> String {
    let origins: Vec<String> = f.origins.iter().map(|(o, n)| format!("{o} ({n})")).collect();
    let (docs, span) = (f.info.documents, years(f));
    match lang {
        Lang::Id => format!(
            "Data dihimpun dari {}. {}Koleksi yang dianalisis berisi {docs} dokumen terbitan {span}{}. Analisis bibliometrik dilakukan dengan aplikasi Celah, mencakup analisis kinerja (produksi tahunan, sumber, penulis, dan negara) serta pemetaan sains (ko-kemunculan kata kunci, ko-sitasi, kopling bibliografis, dan spektroskopi tahun rujukan) mengikuti alur bibliometrix {tools}.",
            list(lang, &origins),
            if f.merged > 0 { format!("Rekaman ganda antar sumber digabung ({} rekaman). ", f.merged) } else { String::new() },
            if f.filtered { " setelah penyaringan menurut tahun, jenis dokumen, atau sumber" } else { "" },
        ),
        Lang::En => format!(
            "Records were retrieved from {}. {}The analyzed collection comprises {docs} documents published in {span}{}. The bibliometric analysis was carried out with Celah, covering performance analysis (annual production, sources, authors, and countries) and science mapping (keyword co-occurrence, co-citation, bibliographic coupling, and reference publication year spectroscopy) following the bibliometrix workflow {tools}.",
            list(lang, &origins),
            if f.merged > 0 { format!("Duplicate records across sources were merged ({} records). ", f.merged) } else { String::new() },
            if f.filtered { " after filtering by year, document type, or source" } else { "" },
        ),
    }
}

pub fn performance(lang: Lang, f: &Facts) -> String {
    let i = &f.info;
    let growth = i.growth_rate.map(|g| dec(lang, g, 1));
    let named = |v: &[(String, i64)]| v.iter().map(|(s, n)| format!("{s} ({n})")).collect::<Vec<_>>();
    let sources = list(lang, &named(&f.sources));
    let authors = list(lang, &f.authors.iter().map(|(a, n)| format!("{a} ({n})")).collect::<Vec<_>>());
    let id = lang == Lang::Id;
    let countries = list(lang, &f.countries.iter().map(|(c, n)| format!("{} ({n})", countries::name(c, id))).collect::<Vec<_>>());
    let mut out = match lang {
        Lang::Id => format!(
            "Koleksi ini memuat {} dokumen dari {} sumber, ditulis oleh {} penulis dengan rata-rata {} sitasi per dokumen.",
            i.documents, i.sources, i.authors, dec(lang, i.avg_citations, 1)
        ),
        Lang::En => format!(
            "The collection contains {} documents from {} sources, written by {} authors, with an average of {} citations per document.",
            i.documents, i.sources, i.authors, dec(lang, i.avg_citations, 1)
        ),
    };
    if let Some(g) = growth {
        out.push_str(&if id { format!(" Produksi tumbuh rata-rata {g}% per tahun.") } else { format!(" Annual production grew by {g}% per year on average.") });
    }
    if let Some((c, year)) = f.quartiles.filter(|(c, _)| c[..4].iter().sum::<usize>() > 0) {
        let total = c.iter().sum::<usize>().max(1) as f64;
        let share = |n: usize| dec(lang, n as f64 * 100.0 / total, 1);
        let y = year.map(|y| format!(" {y}")).unwrap_or_default();
        out.push_str(&if id {
            format!(" Menurut SJR{y}, {}% dokumen terbit di jurnal Q1, {}% di Q2, {}% di Q3 atau Q4, dan {}% di sumber tanpa kuartil.", share(c[0]), share(c[1]), share(c[2] + c[3]), share(c[4]))
        } else {
            format!(" According to SJR{y}, {}% of the documents appeared in Q1 journals, {}% in Q2, {}% in Q3 or Q4, and {}% in sources without a quartile.", share(c[0]), share(c[1]), share(c[2] + c[3]), share(c[4]))
        });
    }
    if !sources.is_empty() {
        out.push_str(&if id { format!(" Sumber paling produktif adalah {sources}.") } else { format!(" The most productive sources are {sources}.") });
    }
    if !authors.is_empty() {
        out.push_str(&if id { format!(" Penulis paling produktif adalah {authors}.") } else { format!(" The most prolific authors are {authors}.") });
    }
    if !countries.is_empty() {
        out.push_str(&if id { format!(" Kontribusi terbesar datang dari {countries}") } else { format!(" The largest contributions come from {countries}") });
        out.push_str(&match (i.international_coauthorship, id) {
            (Some(s), true) => format!(", dengan {}% dokumen melibatkan kolaborasi internasional.", dec(lang, s * 100.0, 1)),
            (Some(s), false) => format!(", and {}% of documents involve international co-authorship.", dec(lang, s * 100.0, 1)),
            (None, _) => ".".into(),
        });
    }
    out
}

/// `top` holds a narrative citation and citation count per most cited document; `rpys` cites Marx et al.
pub fn influence(lang: Lang, f: &Facts, top: &[(String, i64)], rpys: &str) -> Option<String> {
    if top.is_empty() && f.roots.is_empty() {
        return None;
    }
    let id = lang == Lang::Id;
    let cited = list(lang, &top.iter().map(|(c, n)| if id { format!("{c} dengan {n} sitasi") } else { format!("{c} with {n} citations") }).collect::<Vec<_>>());
    let mut out = String::new();
    if !cited.is_empty() {
        out.push_str(&if id { format!("Dokumen paling berpengaruh dalam koleksi adalah {cited}.") } else { format!("The most influential documents in the collection are {cited}.") });
    }
    if !f.roots.is_empty() {
        let roots = list(lang, &f.roots.iter().map(|(y, l)| format!("{y} ({l})")).collect::<Vec<_>>());
        out.push_str(&if id {
            format!(" Spektroskopi tahun rujukan {rpys} menunjukkan akar sejarah bidang ini pada tahun {roots}.")
        } else {
            format!(" Reference publication year spectroscopy {rpys} places the historical roots of the field in {roots}.")
        });
    }
    Some(out.trim().to_string())
}
