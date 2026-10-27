import type { CouplingMap, CouplingQuadrant } from "../lib/couplingMapTypes";

const C: [string[], number, number, number, number, CouplingQuadrant][] = [
  [["chest x-ray", "diagnostic accuracy", "cxr image"], 33, 2.6, 0.9, 0.85, "motor"],
  [["covid-19 detection", "classification task", "chest radiograph"], 31, 2.1, 0.8, 0.7, "motor"],
  [["cnn architecture", "transfer learning", "hospitalized patient"], 16, 1.5, 0.65, 0.4, "basic"],
  [["cough sound", "smartphone", "audio classification"], 9, 1.9, 0.25, 0.6, "niche"],
  [["explainable ai", "grad-cam", "peer review"], 6, 0.3, 0.35, 0.2, "peripheral"],
  [["panoramic radiograph", "dental lesion", "clinical decision"], 3, 3.4, 0.1, 0.95, "niche"],
  [["sputum smear", "microscopy", "low-resource setting"], 4, 0.4, 0.55, 0.1, "basic"],
];

export const SAMPLE_COUPLING_MAP: CouplingMap = {
  coupled: 121,
  used: 121,
  clusters: C.map(([keywords, size, impact, x, y, quadrant], id) => ({
    id, size, keywords, impact, x, y, quadrant, centrality: x * 30,
    documents: [0, 1, 2].map((k) => ({ id: id * 10 + k, title: `Contoh dokumen klaster ${id + 1} nomor ${k + 1}`, year: 2020 + k, citedBy: 120 - k * 30 - id * 5 })),
  })),
};
