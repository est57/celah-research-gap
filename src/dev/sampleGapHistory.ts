// Sample review history for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so it is left out of production builds. Made-up data.
import type { ReviewEntry } from "../lib/gapTypes";

export const SAMPLE_GAP_HISTORY: ReviewEntry[] = [
  { key: "matrix:transformer:cough-sound", kind: "matrix", title: "transformer dan cough sound jarang diteliti bersama", status: "accepted",
    note: "Cek dataset Coswara dan CODA TB DREAM.", updatedAt: "2026-10-21 09:42:10", inAnalysis: true, score: 0.86, verdict: "new", checkedAt: "2026-10-21 10:12:00" },
  { key: "niche:drug-resistant-tuberculosis", kind: "niche", title: "Tema drug-resistant tuberculosis berkembang sendiri, terpisah dari tema utama", status: "check",
    note: "", updatedAt: "2026-10-20 16:05:44", inAnalysis: true, score: 0.77, verdict: null, checkedAt: null },
  { key: "explicit:sputum-smear", kind: "explicit", title: "sputum smear x microscopy", status: "rejected",
    note: "Sudah dibahas tuntas oleh review 2024.", updatedAt: "2026-10-18 11:20:03", inAnalysis: false, score: null, verdict: "answered", checkedAt: "2026-10-18 11:30:00" },
];
