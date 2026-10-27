// Sample data for the browser preview only (npm run dev without Tauri).
// Loaded lazily from a DEV-only branch, so it is left out of production builds. Made-up numbers.
import type { SourcesAuthors } from "../lib/sourcesTypes";

const sources: [string, number][] = [
  ["Int J Tuberc Lung Dis", 64], ["PLOS ONE", 58], ["Scientific Reports", 47], ["Diagnostics", 41],
  ["IEEE Access", 38], ["BMC Infect Dis", 31], ["Frontiers in Medicine", 27], ["Lancet Digital Health", 22],
  ["J Clin Med", 19], ["eClinicalMedicine", 17],
];

const ranked = Array.from({ length: 40 }, (_, i) => ({
  rank: i + 1,
  label: sources[i]?.[0] ?? `Sumber ${i + 1}`,
  count: Math.round(64 / (i + 1) ** 0.72),
  zone: (i < 14 ? 1 : 2) as 1 | 2,
}));

const timeline = (label: string, cells: [number, number, number][]) => ({
  label,
  documents: cells.reduce((a, c) => a + c[1], 0),
  years: cells.map(([year, documents, tcPerYear]) => ({ year, documents, tcPerYear })),
});

export const SAMPLE_SOURCES_AUTHORS: SourcesAuthors = {
  currentYear: 2026,
  totalSources: 612,
  topSources: sources.map(([label, count]) => ({ label, count })),
  bradford: {
    ranked,
    zones: [
      { sources: 14, documents: 636 },
      { sources: 86, documents: 636 },
      { sources: 512, documents: 636 },
    ],
  },
  lotka: {
    totalAuthors: 9214,
    beta: 2.41,
    rows: [
      [1, 7482], [2, 1050], [3, 332], [4, 138], [5, 82], [6, 50], [7, 30], [8, 50],
    ].map(([documents, authors]) => ({ documents, authors, observed: authors / 9214, theoretical: 0.6079 / documents ** 2 })),
  },
  production: [
    timeline("Rahman M", [[2018, 1, 4], [2019, 2, 9], [2020, 2, 12], [2021, 3, 10], [2023, 4, 7], [2024, 2, 5]]),
    timeline("Chen L", [[2017, 1, 3], [2019, 3, 14], [2021, 2, 8], [2022, 3, 6], [2024, 3, 4]]),
    timeline("Mensah K", [[2019, 1, 6], [2020, 2, 15], [2022, 4, 18], [2023, 2, 9]]),
    timeline("Kumar S", [[2018, 2, 5], [2020, 2, 7], [2023, 3, 10], [2025, 3, 2]]),
    timeline("Silva R", [[2019, 1, 5], [2021, 3, 11], [2023, 2, 6], [2024, 2, 4]]),
    timeline("Okafor C", [[2021, 2, 3], [2022, 2, 4], [2024, 4, 6], [2025, 3, 2]]),
    timeline("Mbeki T", [[2018, 1, 3], [2021, 3, 9], [2022, 2, 6], [2024, 1, 2]]),
    timeline("Pratama A", [[2022, 1, 2], [2023, 2, 3], [2024, 3, 4], [2025, 3, 3]]),
  ],
  impact: [
    ["Rahman M", 7, 11, 0.78, 412, 14, 2018], ["Mensah K", 6, 9, 0.75, 506, 9, 2019], ["Chen L", 6, 10, 0.6, 388, 12, 2017],
    ["Silva R", 5, 7, 0.63, 302, 8, 2019], ["Kumar S", 5, 8, 0.56, 241, 10, 2018], ["Mbeki T", 4, 6, 0.44, 188, 7, 2018],
    ["Okafor C", 4, 6, 0.67, 96, 11, 2021], ["Pratama A", 3, 4, 0.6, 41, 9, 2022],
  ].map(([label, h, g, m, citations, documents, firstYear]) => ({
    label: label as string, h: h as number, g: g as number, m: m as number,
    citations: citations as number, documents: documents as number, firstYear: firstYear as number,
  })),
};
