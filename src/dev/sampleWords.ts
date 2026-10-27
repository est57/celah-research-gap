import type { Words } from "../lib/wordTypes";

const LABELS = ["Deep Learning", "Artificial intelligence", "Machine Learning", "Tuberculosis", "Radiomics", "Chest X-ray",
  "Diagnosis", "Convolutional Neural Networks", "Medical imaging", "Antimicrobial resistance", "Classification",
  "Transfer Learning", "Radiology", "Tuberculosis Detection", "Explainable AI", "Cough sound", "Sputum smear",
  "Federated learning", "Vision transformer", "Low-resource settings"];
const COUNTS = [153, 105, 82, 54, 19, 17, 17, 14, 12, 11, 11, 11, 10, 10, 9, 8, 7, 6, 5, 4];
const YEARS = [2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024, 2025];
const SHAPE = [0.01, 0.02, 0.03, 0.05, 0.07, 0.1, 0.12, 0.14, 0.15, 0.15, 0.16];

export const SAMPLE_WORDS: Words = {
  field: "Kata kunci penulis",
  ngram: 1,
  documents: 495,
  withField: 401,
  distinct: 1303,
  words: LABELS.map((label, i) => ({ label, documents: COUNTS[i] })),
  years: YEARS,
  series: LABELS.slice(0, 8).map((label, i) => ({ label, perYear: SHAPE.map((s) => Math.round(COUNTS[i] * s)) })),
};
