import type { Quartiles } from "../lib/quartileTypes";

const src = (label: string, documents: number, quartile: string, sjr: number | null) => ({ label, documents, quartile, sjr });

export const SAMPLE_QUARTILES: Quartiles = {
  status: { journals: 29165, year: 2024 },
  documents: 553,
  counts: [214, 151, 62, 19, 107],
  sources: [
    src("Neurocomputing", 31, "Q1", 1.471), src("IEEE Access", 27, "Q1", 0.96), src("Scientific Reports", 22, "Q1", 0.9),
    src("Diagnostics", 18, "Q2", 0.67), src("Contoh Conference Proceedings", 15, "", null), src("Measurement", 12, "Q1", 1.05),
    src("Indian Journal of Tuberculosis", 9, "Q3", 0.4), src("Jurnal Contoh Nasional", 6, "", null), src("Sensors and Actuators A", 5, "Q1", 0.88),
    src("Lecture Notes in Computer Science", 4, "Q4", 0.32),
  ],
};
