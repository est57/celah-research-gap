// Sample documents for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so they are left out of production builds. Made-up data.
import type { PaperPage, PaperRow } from "../lib/types";

const titles = [
  "Contoh: deep learning untuk skrining TB pada rontgen dada", "Contoh: klasifikasi suara batuk dengan transformer",
  "Contoh: deteksi TB resisten obat dari data genomik", "Contoh: ensemble CNN untuk deteksi penyakit paru",
  "Contoh: Grad-CAM untuk menjelaskan prediksi rontgen", "Contoh: mikroskopi sputum otomatis di layanan primer",
  "Contoh: vision transformer untuk klasifikasi lesi paru", "Contoh: transfer learning pada data rontgen kecil",
];
const origins = ["OpenAlex", "Scopus", "PubMed"];

const ROWS: PaperRow[] = titles.map((title, i) => ({
  id: i + 1, title, authors: "Rahman M; Chen L; +2", year: 2018 + (i % 8), sourceName: "Contoh Jurnal",
  docType: "article", citedBy: 120 - i * 13, origin: origins[i % 3],
}));

export function samplePapers(filter: string): PaperPage {
  const f = filter.trim().toLowerCase();
  const items = f ? ROWS.filter((r) => r.title.toLowerCase().includes(f)) : ROWS;
  return { items, total: items.length };
}
