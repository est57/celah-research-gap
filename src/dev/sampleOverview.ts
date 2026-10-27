// Sample data for the browser preview only (npm run dev without Tauri).
// Loaded through a dynamic import guarded by import.meta.env.DEV, so it is
// left out of production builds. The numbers are made up.
import type { Overview } from "../lib/overviewTypes";

const years: [number, number, number][] = [
  [2015, 22, 4.1], [2016, 31, 4.6], [2017, 48, 5.2], [2018, 67, 5.9], [2019, 95, 6.8], [2020, 141, 7.9],
  [2021, 188, 7.1], [2022, 236, 6.2], [2023, 284, 5.1], [2024, 331, 3.4], [2025, 362, 1.6], [2026, 103, 0.4],
];

export const SAMPLE_OVERVIEW: Overview = {
  currentYear: 2026,
  info: {
    yearMin: 2015, yearMax: 2026, documents: 1908, sources: 612, growthRate: 21.4, avgAge: 3.6,
    avgCitations: 18.2, references: 61437, keywordsPlus: 4912, authorKeywords: 3877, authors: 9214,
    singleAuthorDocs: 41, authorsOfSingle: 38, coauthorsPerDoc: 7.3, internationalCoauthorship: 0.386,
    docTypes: [
      { docType: "article", count: 1512 },
      { docType: "proceedings", count: 254 },
      { docType: "review", count: 142 },
    ],
  },
  annual: years.map(([year, documents, meanTcPerYear]) => ({ year, documents, meanTcPerYear, citableYears: 2027 - year, meanTc: meanTcPerYear * (2027 - year), partial: year === 2026 })),
  threeField: {
    labels: ["Penulis", "Kata kunci penulis", "Sumber"],
    kinds: ["", "", ""],
    left: ["Rahman M", "Chen L", "Mensah K", "Kumar S", "Okafor C"].map((label, i) => ({ label, count: 14 - i * 2 })),
    middle: ["deep learning", "chest x-ray", "computer-aided detection", "cough sound", "screening"].map((label, i) => ({ label, count: 40 - i * 6 })),
    right: ["Int J Tuberc Lung Dis", "PLOS ONE", "Scientific Reports", "IEEE Access", "Lancet Digit Health"].map((label, i) => ({ label, count: 30 - i * 4 })),
    leftLinks: [
      { from: 0, to: 0, value: 8 }, { from: 0, to: 1, value: 5 }, { from: 1, to: 0, value: 7 }, { from: 1, to: 2, value: 3 },
      { from: 2, to: 2, value: 6 }, { from: 2, to: 4, value: 4 }, { from: 3, to: 0, value: 4 }, { from: 3, to: 3, value: 2 },
      { from: 4, to: 3, value: 5 }, { from: 4, to: 4, value: 2 },
    ],
    rightLinks: [
      { from: 0, to: 2, value: 9 }, { from: 0, to: 3, value: 10 }, { from: 1, to: 1, value: 5 }, { from: 2, to: 0, value: 6 },
      { from: 2, to: 4, value: 3 }, { from: 3, to: 3, value: 5 }, { from: 3, to: 1, value: 2 }, { from: 4, to: 0, value: 6 },
    ],
  },
};
