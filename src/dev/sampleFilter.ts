// Sample filter options for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so they are left out of production builds. Made-up data.
import type { FilterOptions } from "../lib/filterTypes";

const journals = ["Contoh Jurnal Radiologi", "Contoh Jurnal Informatika Medis", "Contoh Prosiding Visi Komputer", "Contoh Jurnal Tuberkulosis",
  "Contoh Jurnal Kesehatan Masyarakat", "Contoh Jurnal Kecerdasan Buatan", "Contoh Jurnal Pencitraan Digital", "Contoh Prosiding Pemrosesan Sinyal"];

export const SAMPLE_FILTER: FilterOptions = {
  total: 553,
  yearMin: 2012,
  yearMax: 2025,
  docTypes: [{ value: "article", count: 402 }, { value: "proceedings", count: 71 }, { value: "review", count: 48 }, { value: "preprint", count: 25 }, { value: "", count: 7 }],
  origins: [{ value: "OpenAlex", count: 312 }, { value: "Scopus", count: 168 }, { value: "PubMed", count: 73 }],
  sources: [...journals.map((value, i) => ({ value, count: 40 - i * 4 })), { value: "", count: 21 }],
  filter: { yearFrom: 2018, yearTo: null, docTypes: ["article", "review"], origins: [], sources: [] },
  matching: 371,
};
