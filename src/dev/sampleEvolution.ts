// Sample thematic evolution for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so it is left out of production builds. Made-up data.
import type { Quadrant, Theme } from "../lib/conceptualTypes";
import type { Evolution } from "../lib/evolutionTypes";

const theme = (id: number, name: string, frequency: number, quadrant: Quadrant): Theme => ({
  id, name, frequency, quadrant, documents: Math.round(frequency / 3), centrality: 1, density: 1, x: 0.5, y: 0.5, recentShare: 0.4,
  words: name.split(" · ").map((label, i) => ({ label, freq: Math.round(frequency / (3 + i)) })),
});

const link = (period: number, from: number, to: number, inclusion: number, value: number, shared: string[]) =>
  ({ period, from, to, inclusion, outValue: value, inValue: Math.round(value * 1.1), shared });

export const SAMPLE_EVOLUTION: Evolution = {
  field: "Kata kunci penulis",
  cuts: [2020, 2023],
  periods: [
    { from: 2014, to: 2020, documents: 184, themes: [
      theme(0, "tuberculosis · chest radiography", 420, "basic"), theme(1, "computer-aided detection · CAD4TB", 160, "motor"),
      theme(2, "sputum smear · microscopy", 90, "niche"), theme(3, "drug resistance · genomics", 60, "niche") ] },
    { from: 2021, to: 2023, documents: 201, themes: [
      theme(0, "deep learning · tuberculosis detection", 520, "basic"), theme(1, "transfer learning · CNN", 190, "motor"),
      theme(2, "COVID-19 · pneumonia", 120, "emerging"), theme(3, "Mycobacterium tuberculosis · automated microscopy", 80, "niche"),
      theme(4, "Grad-CAM · explainable AI", 60, "motor") ] },
    { from: 2024, to: 2025, documents: 168, themes: [
      theme(0, "tuberculosis detection · convolutional neural networks", 460, "basic"), theme(1, "Grad-CAM · explainable AI", 140, "motor"),
      theme(2, "vision transformer · lung disease diagnosis", 70, "emerging"), theme(3, "drug resistance prediction · machine learning", 60, "niche"),
      theme(4, "cough sound · smartphone", 40, "emerging") ] },
  ],
  links: [
    link(0, 0, 0, 0.72, 260, ["tuberculosis", "chest radiography", "screening"]), link(0, 1, 1, 0.48, 90, ["computer-aided detection", "CNN"]),
    link(0, 0, 2, 0.31, 60, ["pneumonia", "chest radiograph"]), link(0, 2, 3, 0.66, 50, ["sputum smear", "microscopy"]),
    link(0, 1, 0, 0.22, 40, ["deep learning"]), link(1, 0, 0, 0.81, 300, ["tuberculosis detection", "deep learning"]),
    link(1, 1, 0, 0.44, 80, ["transfer learning"]), link(1, 4, 1, 0.62, 50, ["Grad-CAM", "explainable AI"]),
    link(1, 2, 0, 0.18, 30, ["pneumonia"]), link(1, 3, 3, 0.12, 12, ["Mycobacterium tuberculosis"]),
  ],
};
