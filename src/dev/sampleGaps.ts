// Sample gaps for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so they are left out of production builds. Made-up data.
import type { Gap, GapEvidence, GapKind } from "../lib/gapTypes";

const ev = (paperId: number, title: string, year: number, role: GapEvidence["role"], quote: string | null = null): GapEvidence => ({
  paperId, title, year, role, quote, source: "Contoh Jurnal", authors: "Rahman M; Chen L; +3", citedBy: 20 - paperId,
});

const gap = (key: string, kind: GapKind, title: string, score: number, statement: string, evidence: GapEvidence[], screen: Gap["signals"][number]["screen"]): Gap => ({
  key, kind, title, score, statement, evidence,
  confidence: score >= 0.7 ? "Kuat" : score >= 0.5 ? "Sedang" : "Lemah",
  parts: [{ label: "Kebaruan", value: Math.min(1, score + 0.08) }, { label: "Bukti", value: score - 0.15 }, { label: "Momentum", value: score - 0.05 }],
  signals: [{ analysis: kind === "explicit" ? "Kalimat celah di abstrak" : kind === "front" ? "Peta kopling" : "Peta tematik", finding: "Contoh temuan analisis", screen,
    tab: kind === "front" ? "coupling" : undefined }],
  questions: ["Bagaimana pendekatan dari tema pertama dapat diterapkan pada tema kedua?", "Apa yang berubah jika keduanya diteliti dalam satu desain studi?"],
  terms: key.split(":").slice(1).map((t) => t.replace(/-/g, " ")).concat("tuberculosis"),
  check: null,
  status: "",
  note: "",
});

const checked: Gap["check"] = {
  checkedAt: "2026-10-21 10:12:00",
  query: "transformer cough sound tuberculosis",
  found: 14,
  matching: 1,
  verdict: "new",
  papers: [
    { title: "Audio transformers for tuberculosis cough triage in primary care", year: 2025, doi: "10.0000/contoh.1", source: "Contoh Jurnal", citedBy: 3, matches: true },
    { title: "Cough sound datasets for respiratory disease", year: 2026, doi: null, source: "Contoh Prosiding", citedBy: 0, matches: false },
  ],
};

export const SAMPLE_GAPS = {
  documents: 553,
  field: "Kata kunci penulis",
  gaps: [
    gap("matrix:transformer:cough-sound", "matrix", "transformer dan cough sound jarang diteliti bersama", 0.86,
      "\"transformer\" muncul di 74 dokumen dan \"cough sound\" di 41 dokumen, tetapi keduanya hanya muncul bersama di 2 dokumen, padahal diperkirakan sekitar 6,6.",
      [ev(1, "Audio spectrogram transformer for TB cough screening", 2025, "hampir menjawab"), ev(2, "Cough audio classification for pulmonary TB triage", 2024, "konteks"), ev(3, "Deep learning for TB screening on chest radiographs", 2023, "konteks")], "conceptual"),
    gap("niche:drug-resistant-tuberculosis", "niche", "Tema drug-resistant tuberculosis berkembang sendiri, terpisah dari tema utama", 0.77,
      "Tema drug-resistant tuberculosis (35 dokumen) adalah tema niche: rapat di dalam kelompoknya tetapi jarang terhubung dengan tema lain.",
      [ev(4, "Deep learning on genomic sequences for drug-resistant TB", 2025, "mendukung"), ev(5, "Subtype triage of drug-resistant TB on CT", 2024, "mendukung")], "conceptual"),
    gap("emerging:federated-learning", "emerging", "Tema federated learning baru tumbuh dan belum terhubung dengan tema utama", 0.73,
      "Tema federated learning (23 dokumen) berada di kuadran baru muncul: 83% dokumennya terbit 2023-2025, di atas rata-rata koleksi (52%).",
      [ev(6, "Federated learning for privacy-preserving TB diagnosis", 2026, "mendukung"), ev(7, "Multi-center chest X-ray models without data sharing", 2025, "mendukung"), ev(8, "Privacy in medical imaging AI", 2024, "konteks")], "conceptual"),
    gap("explicit:smartphone", "explicit", "Penulis menyebut smartphone masih perlu diteliti lebih lanjut", 0.64,
      "4 dokumen tentang \"smartphone\" menyatakan sendiri di abstraknya bahwa ada hal yang belum terjawab.",
      [ev(9, "Cough apps for TB triage", 2024, "mendukung", "Robustness to low-cost smartphone microphones remains unclear."), ev(10, "Field evaluation of mobile TB screening", 2025, "mendukung", "Future research should validate these models on field recordings.")], "collection"),
    gap("structural:cad:xpert", "structural", "Kelompok topik CAD dan Xpert belum saling terhubung", 0.58,
      "Kelompok kata kunci (CAD, screening) dan (Xpert, diagnostic accuracy) hanya terhubung lewat 3 kemunculan bersama, sekitar 18% dari yang diharapkan.",
      [ev(11, "CAD followed by Xpert in community screening", 2022, "hampir menjawab"), ev(12, "Computer-aided detection: a multi-country evaluation", 2022, "konteks")], "conceptual"),
    gap("front:cough-sound:chest-x-ray", "front", "Front riset cough sound berdampak tinggi tetapi terpisah dari front utama", 0.55,
      "9 dokumen tentang (cough sound, smartphone, audio classification) saling merujuk karya yang sama dan disitasi 1.9 kali rata-rata tahun terbitnya, tetapi jarang berbagi rujukan dengan front utama (chest x-ray, diagnostic accuracy, cxr image).",
      [ev(13, "Cough audio deep learning for TB triage", 2023, "mendukung"), ev(14, "Deep learning chest radiograph reading at scale", 2021, "konteks")], "intellectual"),
  ].map((g, i) => (i === 0 ? { ...g, status: "accepted", check: checked } : i === 2 ? { ...g, status: "accepted" } : g)) as Gap[],
};
