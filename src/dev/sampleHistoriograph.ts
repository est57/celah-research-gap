// Sample historiograph for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so it is left out of production builds. Made-up data.
import type { HistNode, Historiograph } from "../lib/histTypes";

const n = (id: number, who: string, year: number, lcs: number, cluster: number, title: string): HistNode =>
  ({ id, label: `${who} ${year}`, title, year, source: "Contoh Jurnal", doi: null, lcs, gcs: lcs * 7, cluster });

const nodes = [
  n(1, "Jaeger", 2014, 40, 0, "Contoh: dataset rontgen dada publik untuk skrining TB"),
  n(2, "Hwang", 2016, 52, 0, "Contoh: CNN dalam untuk skrining tuberkulosis"),
  n(3, "Lakhani", 2017, 48, 0, "Contoh: deep learning pada radiografi dada untuk TB"),
  n(4, "Rajpurkar", 2017, 62, 1, "Contoh: deteksi pneumonia setara radiolog"),
  n(5, "Hwang", 2018, 29, 2, "Contoh: validasi algoritma deteksi otomatis"),
  n(6, "Pasa", 2019, 66, 0, "Contoh: arsitektur jaringan efisien untuk skrining TB"),
  n(7, "Qin", 2019, 25, 2, "Contoh: membaca radiografi dada dengan kecerdasan buatan"),
  n(8, "Rahman", 2020, 121, 0, "Contoh: deteksi TB andal dengan segmentasi dan visualisasi"),
  n(9, "Chouhan", 2020, 33, 1, "Contoh: transfer learning untuk deteksi pneumonia"),
  n(10, "Rajaraman", 2020, 33, 1, "Contoh: ensembel model khusus modalitas untuk TB"),
  n(11, "Ayaz", 2021, 32, 0, "Contoh: ensemble learning untuk deteksi TB otomatis"),
  n(12, "Nafisah", 2022, 26, 2, "Contoh: perbandingan arsitektur CNN untuk TB"),
];

export const SAMPLE_HISTORIOGRAPH: Historiograph = {
  documents: 591,
  links: 1852,
  byId: 1790,
  byDoi: 48,
  byName: 14,
  nodes,
  edges: [[1, 0], [2, 0], [2, 1], [5, 1], [5, 2], [7, 1], [7, 5], [7, 0], [8, 3], [9, 3], [9, 2], [10, 7], [10, 5], [11, 7], [11, 8], [6, 4], [4, 2], [6, 2], [11, 9]],
};
