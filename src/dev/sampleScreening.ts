// Sample screening data for the browser preview (npm run dev).
import type { Prisma, ScreenRow } from "../lib/screeningTypes";

const ABSTRACT = "Contoh abstrak: studi ini mengevaluasi model deep learning untuk skrining tuberkulosis dari citra rontgen dada pada beberapa rumah sakit, dengan sensitivitas dan spesifisitas yang dibandingkan terhadap pembacaan radiolog.";

const TITLES = [
  "Contoh: deep learning untuk skrining TB pada rontgen dada",
  "Contoh: klasifikasi suara batuk dengan transformer",
  "Contoh: kebijakan pembiayaan program TB nasional",
  "Contoh: ensemble CNN untuk deteksi penyakit paru",
  "Contoh: tinjauan naratif sejarah vaksin BCG",
  "Contoh: Grad-CAM untuk menjelaskan prediksi rontgen",
  "Contoh: deteksi TB resisten obat dari data genomik",
  "Contoh: aplikasi ponsel untuk kepatuhan minum obat",
];

export const SAMPLE_SCREEN_ROWS: ScreenRow[] = TITLES.map((title, i) => ({
  id: i + 1,
  title,
  authors: "Rahman M; Chen L; +3",
  year: 2025 - (i % 5),
  sourceName: i % 2 ? "Scientific Reports" : "IEEE Access",
  abstractText: i === 4 ? null : ABSTRACT,
  keywords: "tuberculosis; deep learning; chest x-ray",
  decision: i === 0 || i === 3 ? "include" : i === 2 ? "exclude" : "",
  reason: i === 2 ? "topic" : "",
  note: i === 2 ? "Kebijakan, bukan deteksi" : "",
}));

export const SAMPLE_PRISMA: Prisma = {
  sources: [
    { label: "OpenAlex", n: 412 },
    { label: "Scopus", n: 160 },
    { label: "Crossref", n: 23 },
  ],
  identified: 595,
  duplicates: 41,
  removedOther: 1,
  screened: 553,
  excluded: 38,
  reasons: [
    { label: "topic", n: 24 },
    { label: "type", n: 9 },
    { label: "language", n: 5 },
  ],
  unscreened: 120,
  filterActive: true,
  filteredOut: 12,
  included: 503,
};
