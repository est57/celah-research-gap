// Sample factorial map for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so it is left out of production builds. Made-up data.
import type { Factorial, FactorWord } from "../lib/factorialTypes";

const group = (cluster: number, cx: number, cy: number, labels: string[]): FactorWord[] =>
  labels.map((label, i) => ({ label, cluster, freq: 120 - i * 9 - cluster * 15, x: cx + Math.cos(i * 1.7) * (0.15 + i * 0.05), y: cy + Math.sin(i * 1.7) * (0.12 + i * 0.04) }));

const words = [
  ...group(0, -0.8, 0.4, ["tuberculosis detection", "chest radiography", "convolutional neural networks", "transfer learning", "chest X-ray images", "image classification", "ResNet", "data augmentation"]),
  ...group(1, 0.75, -0.05, ["deep learning", "Mycobacterium tuberculosis", "tuberculosis diagnosis", "pulmonary tuberculosis", "sputum smear", "drug resistance", "machine learning"]),
  ...group(2, -0.8, -1.9, ["pneumonia", "COVID-19 pandemic", "computed tomography", "lung disease diagnosis", "lung cancer"]),
];

export const SAMPLE_FACTORIAL: Factorial = {
  field: "Kata kunci penulis",
  documents: 541,
  explained: [0.38, 0.29],
  words,
  clusters: [
    { id: 0, size: 8, top: ["tuberculosis detection", "chest radiography", "convolutional neural networks", "transfer learning"], x: -0.78, y: 0.38 },
    { id: 1, size: 7, top: ["deep learning", "Mycobacterium tuberculosis", "tuberculosis diagnosis", "pulmonary tuberculosis"], x: 0.74, y: -0.05 },
    { id: 2, size: 5, top: ["pneumonia", "COVID-19 pandemic", "computed tomography", "lung disease diagnosis"], x: -0.79, y: -1.95 },
  ],
  topDocuments: [
    { id: 1, title: "Contoh: pendekatan deep learning untuk diagnosis berbagai penyakit paru", year: 2023, x: -0.99, y: -0.72, cluster: 2 },
    { id: 2, title: "Contoh: tinjauan sistematis deep learning untuk penyakit paru", year: 2024, x: -0.51, y: -1.09, cluster: 2 },
    { id: 3, title: "Contoh: transfer learning pada rontgen dada untuk skrining TB", year: 2022, x: -1.1, y: 0.6, cluster: 0 },
    { id: 4, title: "Contoh: prediksi resistansi obat dari data genomik", year: 2025, x: 1.2, y: -0.3, cluster: 1 },
  ],
};
