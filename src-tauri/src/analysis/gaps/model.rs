use serde::Serialize;

#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// Two popular keywords that rarely appear together.
    Matrix,
    /// A young, fast-growing theme that is weakly linked to the rest.
    Emerging,
    /// A developed theme that is isolated from the main themes.
    Niche,
    /// Two keyword clusters connected far less than their size predicts.
    Structural,
    /// Authors state the gap themselves (future work, limitations).
    Explicit,
    /// Two concepts the researcher tested and saved.
    Idea,
    /// A highly cited coupling cluster that shares few references with the main front.
    Front,
}

#[derive(Serialize, Clone)]
pub struct Signal {
    pub analysis: &'static str,
    pub finding: String,
    /// Screen id in the app where the signal can be inspected.
    pub screen: &'static str,
    /// Tab of that screen, when not its first one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab: Option<&'static str>,
}

#[derive(Serialize, Clone)]
pub struct ScorePart {
    pub label: &'static str,
    /// 0..1
    pub value: f64,
}

/// A supporting document, before its details are loaded.
#[derive(Clone)]
pub struct EvidenceRef {
    pub paper_id: i64,
    /// "mendukung", "hampir menjawab" or "konteks".
    pub role: &'static str,
    pub quote: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Evidence {
    pub paper_id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub source: Option<String>,
    pub authors: String,
    pub cited_by: i64,
    pub role: &'static str,
    pub quote: Option<String>,
}

pub struct Draft {
    /// Stable id built from the kind and its keywords, so reviews survive recomputation.
    pub key: String,
    pub kind: Kind,
    pub title: String,
    pub statement: String,
    /// Core terms of the gap, used to search for newer papers during verification.
    pub terms: Vec<String>,
    pub parts: Vec<ScorePart>,
    pub signals: Vec<Signal>,
    pub evidence: Vec<EvidenceRef>,
    pub questions: Vec<String>,
}

#[derive(Serialize)]
pub struct Gap {
    pub key: String,
    pub kind: Kind,
    pub title: String,
    pub statement: String,
    pub terms: Vec<String>,
    pub score: f64,
    pub confidence: &'static str,
    pub parts: Vec<ScorePart>,
    pub signals: Vec<Signal>,
    pub evidence: Vec<Evidence>,
    pub questions: Vec<String>,
    /// "accepted", "check", "rejected" or "" when not reviewed.
    pub status: String,
    pub note: String,
    /// Latest verification against recent literature, if one was run.
    pub check: Option<crate::db::checks::CheckSummary>,
}

/// Weighted mean of the parts, each weight given alongside.
pub fn score(parts: &[ScorePart], weights: &[f64]) -> f64 {
    let total: f64 = weights.iter().sum();
    let sum: f64 = parts.iter().zip(weights).map(|(p, w)| p.value.clamp(0.0, 1.0) * w).sum();
    if total > 0.0 { sum / total } else { 0.0 }
}

pub fn confidence(score: f64, evidence: usize) -> &'static str {
    match (score, evidence) {
        (s, e) if s >= 0.7 && e >= 3 => "Kuat",
        (s, e) if s >= 0.5 && e >= 2 => "Sedang",
        _ => "Lemah",
    }
}

/// Key fragment: lowercase, alphanumerics and single dashes only.
pub fn slug(text: &str) -> String {
    let raw: String = text.chars().map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '-' }).collect();
    raw.split('-').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("-").chars().take(80).collect()
}

pub fn part(label: &'static str, value: f64) -> ScorePart {
    ScorePart { label, value: (value.clamp(0.0, 1.0) * 100.0).round() / 100.0 }
}
