// Sample duplicate candidates for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so they are left out of production builds. Made-up data.
import type { DuplicateList } from "../lib/duplicateTypes";

export const SAMPLE_DUPLICATES: DuplicateList = {
  documents: 553,
  truncated: false,
  pairs: [
    {
      reason: "same_title",
      similarity: 1,
      keep: 11,
      a: { id: 11, title: "Compact convolutional transformers for tuberculosis detection from chest X-ray images", year: 2024, origin: "OpenAlex", source: "Contoh Jurnal Radiologi", docType: "article", doi: "10.0000/contoh.11", authors: "Rahman M; Chen L; +2", citedBy: 18, has: ["Abstrak", "Kata kunci", "Negara"] },
      b: { id: 12, title: "Compact Convolutional Transformers for Tuberculosis Detection from Chest X-Ray Images", year: 2023, origin: "arXiv", source: null, docType: "preprint", doi: "10.48550/contoh.12", authors: "Rahman M; Chen L; +2", citedBy: 4, has: ["Abstrak", "Referensi"] },
    },
    {
      reason: "similar_title",
      similarity: 0.89,
      keep: 21,
      a: { id: 21, title: "Cough sound analysis with deep learning for pulmonary tuberculosis screening in primary care", year: 2025, origin: "Scopus", source: "Contoh Prosiding", docType: "proceedings", doi: null, authors: "Okafor C; Putri A", citedBy: 2, has: ["Abstrak", "Kata kunci", "Referensi", "Negara"] },
      b: { id: 22, title: "Cough sound analysis using deep learning for pulmonary tuberculosis screening in primary care", year: 2025, origin: "Europe PMC", source: "Contoh Prosiding", docType: "proceedings", doi: null, authors: "Okafor C; Putri A", citedBy: 2, has: ["Abstrak"] },
    },
  ],
};
