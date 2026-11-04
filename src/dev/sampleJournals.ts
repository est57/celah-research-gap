import type { JournalPick, JournalTargets } from "../lib/journalTypes";

const BONUS: Record<string, number> = { Q1: 1.2, Q2: 1.1 };

const pick = (label: string, documents: number, matching: number, recent: number, citations: number, quartile: string, sjr: number | null, gaps: string[]): JournalPick => ({
  label, documents, matching, recent, citations, quartile, sjr, gaps,
  score: Math.round((matching + recent) * (BONUS[quartile] ?? 1) * 100) / 100,
});

const COUGH = "Deteksi TB dari suara batuk di ponsel";
const NOISE = "Ketahanan model audio terhadap derau";
const LOW = "Validasi di fasilitas kesehatan primer";

export const SAMPLE_JOURNALS: JournalTargets = {
  status: { journals: 29165, year: 2024 },
  fallback: false,
  gaps: [COUGH, NOISE, LOW],
  documents: 553,
  recentFrom: 2023,
  journals: [
    pick("Scientific Reports", 22, 9, 6, 141, "Q1", 0.9, [COUGH, NOISE, LOW]),
    pick("IEEE Access", 27, 8, 5, 96, "Q1", 0.96, [COUGH, NOISE]),
    pick("Diagnostics", 18, 7, 5, 64, "Q2", 0.67, [COUGH, LOW]),
    pick("Neurocomputing", 31, 6, 2, 210, "Q1", 1.471, [NOISE]),
    pick("Contoh Conference Proceedings", 15, 6, 3, 18, "", null, [COUGH]),
    pick("Sensors and Actuators A", 5, 3, 2, 22, "Q1", 0.88, [NOISE]),
    pick("Indian Journal of Tuberculosis", 9, 4, 1, 37, "Q3", 0.4, [COUGH, LOW]),
    pick("Jurnal Contoh Nasional", 6, 2, 2, 3, "", null, [LOW]),
  ],
};
