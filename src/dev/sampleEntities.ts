// Sample source and author tables for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so they are left out of production builds. Made-up data.
import type { Affiliations, AuthorRow, CountryTable, SourceRow, SourcesDetail } from "../lib/entityTypes";

const src = (label: string, documents: number, citations: number, h: number, firstYear: number, zone: 1 | 2 | 3): SourceRow =>
  ({ label, documents, citations, h, g: Math.min(documents, h + 3), m: h / (2027 - firstYear), firstYear, zone });

const years = [2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024, 2025, 2026];
const grow = (end: number, start: number) => years.map((_, i) => Math.round(start + ((end - start) * i * i) / ((years.length - 1) ** 2)));

export const SAMPLE_SOURCES: SourcesDetail = {
  currentYear: 2026,
  years,
  sources: [
    src("Contoh Jurnal Radiologi", 48, 2410, 22, 2016, 1), src("Contoh Jurnal Informatika Medis", 41, 1320, 18, 2017, 1),
    src("Contoh Prosiding Visi Komputer", 37, 610, 12, 2018, 1), src("Contoh Jurnal Tuberkulosis", 29, 880, 14, 2016, 2),
    src("Contoh Jurnal Kesehatan Masyarakat", 22, 430, 10, 2019, 2), src("Contoh Jurnal Kecerdasan Buatan", 18, 300, 8, 2020, 2),
    src("Contoh Jurnal Pencitraan Digital", 9, 120, 5, 2021, 3), src("Contoh Prosiding Pemrosesan Sinyal", 6, 40, 3, 2022, 3),
  ],
  dynamics: [
    { label: "Contoh Jurnal Radiologi", cumulative: grow(48, 2) }, { label: "Contoh Jurnal Informatika Medis", cumulative: grow(41, 0) },
    { label: "Contoh Prosiding Visi Komputer", cumulative: grow(37, 0) }, { label: "Contoh Jurnal Tuberkulosis", cumulative: grow(29, 3) },
    { label: "Contoh Jurnal Kesehatan Masyarakat", cumulative: grow(22, 0) },
  ],
};

const names = ["Rahman M", "Chen L", "Okafor C", "Silva R", "Putri A", "Lee J", "Kumar S", "Nguyen T", "Mensah K", "Santos P", "Hartono B", "Wijaya D"];
export const SAMPLE_AUTHORS: AuthorRow[] = names.flatMap((n, i) => [0, 1, 2].map((k) => {
  const documents = Math.max(1, 14 - i - k * 4);
  return { label: k ? `${n.split(" ")[0]}${k} ${n.split(" ")[1]}` : n, documents, fractional: documents / (2.5 + k), citations: documents * (40 - i * 2), h: Math.max(1, Math.round(documents * 0.6)), g: documents, m: 0.4 + (i % 4) / 10, firstYear: 2016 + (i % 6), lastYear: 2025 };
}));

const inst: [string, string, number][] = [["Universitas Contoh", "ID", 31], ["Contoh Institute of Technology", "IN", 24], ["Contoh Medical University", "KR", 19],
  ["Contoh University Hospital", "US", 15], ["Contoh National University", "SG", 12], ["Contoh Research Center", "ZA", 9], ["Politeknik Contoh", "ID", 6]];
export const SAMPLE_AFFILIATIONS: Affiliations = {
  documents: 553,
  withAffiliation: 498,
  rows: inst.map(([label, country, documents], i) => ({ label, country, documents, citations: documents * (30 - i * 3), firstYear: 2016 + i })),
  years,
  dynamics: inst.slice(0, 5).map(([label, , documents]) => ({ label, cumulative: grow(documents, 0) })),
};

const nations: [string, number, number][] = [["IN", 142, 28], ["CN", 118, 41], ["US", 96, 77], ["ID", 64, 9], ["KR", 51, 33], ["GB", 44, 52],
  ["ZA", 38, 21], ["BR", 27, 14], ["SA", 25, 18], ["NG", 19, 7]];
export const SAMPLE_COUNTRY_TABLE: CountryTable = {
  documents: 553,
  withCountry: 471,
  rows: nations.map(([code, documents, avg], i) => {
    const corresponding = Math.round(documents * 0.7);
    const mcp = Math.round(corresponding * (0.15 + (i % 4) * 0.08));
    return { code, documents, citations: documents * avg, avgCitations: avg, corresponding, scp: corresponding - mcp, mcp, firstYear: 2015 + (i % 5) };
  }),
  years,
  dynamics: nations.slice(0, 6).map(([label, documents]) => ({ label, cumulative: grow(documents, 0) })),
};
