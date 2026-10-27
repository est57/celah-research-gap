// Sample data for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so it is left out of production builds. Made-up numbers.
import type { Conceptual, Quadrant, Theme } from "../lib/conceptualTypes";

const theme = (id: number, words: [string, number][], x: number, y: number, quadrant: Quadrant, recentShare: number, documents: number): Theme => ({
  id,
  name: words.slice(0, 3).map((w) => w[0]).join(" · "),
  words: words.map(([label, freq]) => ({ label, freq })),
  frequency: words.reduce((a, w) => a + w[1], 0),
  documents,
  centrality: x * 12,
  density: y * 40,
  x,
  y,
  quadrant,
  recentShare,
});

export const SAMPLE_CONCEPTUAL: Conceptual = {
  field: "Kata kunci penulis",
  wordsUsed: 250,
  documents: 1908,
  map: {
    collectionRecentShare: 0.52,
    recentFrom: 2023,
    recentTo: 2025,
    themes: [
      theme(0, [["deep learning", 412], ["chest radiograph", 301], ["cnn", 188], ["transfer learning", 96]], 0.86, 0.3, "basic", 0.48, 412),
      theme(1, [["computer-aided detection", 188], ["screening", 164], ["cad4tb", 71], ["triage", 52]], 0.75, 0.72, "motor", 0.44, 288),
      theme(2, [["xpert mtb/rif", 92], ["diagnostic accuracy", 88], ["sensitivity", 61]], 0.63, 0.18, "basic", 0.31, 174),
      theme(3, [["sputum microscopy", 77], ["smear", 49], ["ziehl-neelsen", 28]], 0.18, 0.82, "niche", 0.29, 131),
      theme(4, [["hiv", 66], ["coinfection", 31], ["people living with hiv", 24]], 0.31, 0.62, "niche", 0.36, 94),
      theme(5, [["cough sound", 41], ["audio", 33], ["smartphone", 19], ["mfcc", 14]], 0.2, 0.3, "emerging", 0.71, 96),
      theme(6, [["cost-effectiveness", 38], ["implementation", 29], ["health economics", 17]], 0.38, 0.2, "emerging", 0.64, 71),
      theme(7, [["federated learning", 17], ["privacy", 11]], 0.07, 0.4, "emerging", 0.83, 23),
      theme(8, [["explainability", 44], ["grad-cam", 23], ["saliency", 15]], 0.47, 0.44, "emerging", 0.49, 58),
    ],
  },
  trends: [
    ["xpert mtb/rif", 92, 2016, 2018, 2020], ["transfer learning", 96, 2018, 2019, 2021], ["cad4tb", 71, 2019, 2020, 2022],
    ["explainable ai", 44, 2021, 2022, 2024], ["cough sound", 41, 2021, 2023, 2025], ["vision transformer", 29, 2023, 2024, 2025],
    ["federated learning", 17, 2023, 2024, 2025], ["large language model", 9, 2024, 2025, 2025],
  ].map(([label, freq, q1, median, q3]) => ({ label: label as string, freq: freq as number, q1: q1 as number, median: median as number, q3: q3 as number })),
};
