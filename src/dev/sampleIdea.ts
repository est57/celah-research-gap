// Sample idea test for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so it is left out of production builds. Made-up data.
import type { Idea } from "../lib/ideaTypes";

const d = (id: number, title: string, year: number, citedBy: number) => ({ id, title, authors: "Rahman M; Chen L", year, citedBy });

export function sampleIdea(a: string, b: string): Idea {
  return {
    a: a.split(",").map((s) => s.trim()).filter(Boolean), b: b.split(",").map((s) => s.trim()).filter(Boolean),
    documents: 553, withA: 48, withB: 37, withBoth: 1, expected: 3.2, verdict: "celah",
    recentA: 0.62, recentB: 0.21, recentAll: 0.38,
    years: [2018, 2019, 2020, 2021, 2022, 2023, 2024, 2025],
    perYear: [[0, 1, 2, 4, 7, 10, 12, 12], [6, 5, 5, 4, 5, 4, 4, 4], [0, 0, 0, 0, 0, 0, 1, 0]],
    bothDocs: [d(1, "Contoh: vision transformer untuk citra sputum smear", 2024, 3)],
    aOnly: [d(2, "Contoh: vision transformer untuk rontgen dada", 2023, 41), d(3, "Contoh: ViT ringan untuk skrining TB", 2024, 18)],
    bOnly: [d(4, "Contoh: mikroskopi sputum otomatis di layanan primer", 2021, 55), d(5, "Contoh: CNN untuk pewarnaan Ziehl-Neelsen", 2020, 30)],
  };
}
