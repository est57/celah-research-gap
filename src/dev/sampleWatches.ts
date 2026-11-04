// Sample topic monitors for the browser preview (npm run dev).
import type { Watch, WatchHit } from "../lib/apiWatch";

export const SAMPLE_WATCHES: Watch[] = [
  {
    id: 2, query: "cough sound tuberculosis deep learning", sources: ["openalex", "pubmed", "europe_pmc"], docType: "article",
    yearFrom: 2018, maxResults: 200, createdAt: "2026-09-02 08:10:00", lastRunAt: "2026-10-30 07:45:00", lastNew: 4, totalNew: 11,
  },
  {
    id: 1, query: "federated learning chest x-ray", sources: ["openalex", "crossref"], docType: null,
    yearFrom: 2020, maxResults: 100, createdAt: "2026-07-14 10:00:00", lastRunAt: "2026-08-01 09:00:00", lastNew: 0, totalNew: 3,
  },
];

export const SAMPLE_HITS: WatchHit[] = [
  { id: 101, title: "Contoh: model transformer audio untuk triase batuk TB di layanan primer", authors: "Rahman M; Chen L; +3", year: 2026, sourceName: "Contoh Jurnal", origin: "OpenAlex", foundAt: "2026-10-30 07:45:00" },
  { id: 102, title: "Contoh: dataset suara batuk multi-negara untuk skrining tuberkulosis", authors: "Mensah K; Okafor C", year: 2026, sourceName: "Contoh Data Journal", origin: "PubMed", foundAt: "2026-10-30 07:45:00" },
  { id: 103, title: "Contoh: evaluasi aplikasi ponsel pendeteksi batuk TB", authors: "Putri A; Lee J; +2", year: 2025, sourceName: "Contoh Digital Health", origin: "Europe PMC", foundAt: "2026-10-30 07:45:00" },
];
