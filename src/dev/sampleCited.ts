// Sample most-cited documents for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so they are left out of production builds. Made-up data.
import type { Cited, CitedDoc, CitedSort } from "../lib/citedTypes";

const doc = (id: number, title: string, authors: string, year: number, global: number, local: number, normalized: number): CitedDoc =>
  ({ id, title, authors, year, source: "Contoh Jurnal", doi: `10.0000/contoh.${id}`, global, local, perYear: global / (2027 - year), normalized });

const DOCS = [
  doc(1, "Contoh: deteksi pneumonia setara radiolog dengan deep learning", "Rajpurkar P; Irvin J; +8", 2017, 2279, 62, 4.1),
  doc(2, "Contoh: transfer learning untuk klasifikasi COVID-19 dari rontgen", "Abbas A; Abdelsamea M", 2020, 1126, 23, 2.6),
  doc(3, "Contoh: deteksi TB andal dengan segmentasi dan visualisasi", "Rahman T; Khandakar A; +6", 2020, 655, 121, 1.5),
  doc(4, "Contoh: arsitektur jaringan efisien untuk skrining TB", "Pasa F; Golkov V; +3", 2019, 370, 66, 1.9),
  doc(5, "Contoh: membaca radiografi dada dengan kecerdasan buatan", "Qin ZZ; Sander MS; +4", 2019, 288, 25, 1.4),
  doc(6, "Contoh: CNN dalam untuk skrining tuberkulosis", "Hwang S; Kim HE; +2", 2016, 215, 52, 2.2),
  doc(7, "Contoh: vision transformer untuk diagnosis penyakit paru", "Putri A; Lee J", 2024, 48, 3, 3.4),
];

export function sampleCited(sort: CitedSort): Cited {
  const key = (d: CitedDoc) => (sort === "local" ? d.local : sort === "normalized" ? d.normalized : d.global);
  return { documents: 553, localLinks: 1852, docs: [...DOCS].sort((a, b) => key(b) - key(a)) };
}
