// Sample metadata quality for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so it is left out of production builds. Made-up data.
import type { MetadataQuality } from "../lib/qualityTypes";

const tags: [string, string][] = [
  ["AU", "Penulis"], ["TI", "Judul"], ["PY", "Tahun terbit"], ["SO", "Sumber"], ["TC", "Jumlah sitasi"], ["AB", "Abstrak"],
  ["DI", "DOI"], ["C1", "Afiliasi"], ["DE", "Kata kunci penulis"], ["ID", "Keywords Plus"], ["CR", "Referensi"], ["UN", "Institusi"], ["CU", "Negara afiliasi"],
];

export const SAMPLE_QUALITY: MetadataQuality = {
  total: 553,
  fields: tags.map(([tag, label]) => ({ tag, label })),
  analyses: [
    { key: "conceptual", label: "Peta tematik & tren topik", needs: ["DE", "ID"] },
    { key: "intellectual", label: "Ko-sitasi & struktur intelektual", needs: ["CR"] },
    { key: "historiograph", label: "Historiograf (sitasi langsung)", needs: ["CR"] },
    { key: "social", label: "Jaringan kolaborasi penulis", needs: ["AU"] },
    { key: "countries", label: "Kolaborasi negara (SCP/MCP)", needs: ["CU"] },
    { key: "explicit", label: "Kalimat celah di abstrak", needs: ["AB"] },
    { key: "trends", label: "Produksi tahunan & momentum", needs: ["PY"] },
  ],
  origins: [
    { origin: "OpenAlex", count: 312, missing: [2, 0, 0, 14, 0, 41, 9, 30, 120, 312, 312, 34, 36], usable: [298, 0, 120, 280, 276, 271, 312] },
    { origin: "Scopus", count: 168, missing: [0, 0, 0, 0, 0, 3, 5, 2, 11, 18, 4, 3, 2], usable: [166, 164, 164, 151, 166, 165, 168] },
    { origin: "PubMed", count: 73, missing: [0, 0, 0, 0, 0, 2, 6, 4, 25, 73, 51, 11, 9], usable: [70, 22, 22, 66, 64, 71, 73] },
  ],
};
