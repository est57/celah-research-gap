// Sample keyword synonyms for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so they are left out of production builds. Made-up data.
import type { KeywordSynonyms } from "../lib/synonymTypes";

const t = (label: string, documents: number) => ({ label, documents });

export const SAMPLE_SYNONYMS: KeywordSynonyms = {
  suggestions: [
    { kind: "singkatan", terms: [t("convolutional neural network", 41), t("CNN", 23)] },
    { kind: "ejaan", terms: [t("chest X-ray", 37), t("chest x-rays", 12), t("Chest-X-ray", 3)] },
    { kind: "singkatan", terms: [t("computer-aided detection", 14), t("computer-aided diagnosis", 9), t("CAD", 8)] },
    { kind: "ejaan", terms: [t("deep learning", 120), t("deep-learning", 4)] },
  ],
  synonyms: [{ fromKey: "tb", fromLabel: "TB", toKey: "tuberculosis", toLabel: "tuberculosis" }],
  terms: [t("deep learning", 120), t("tuberculosis", 98), t("convolutional neural network", 41), t("chest X-ray", 37), t("transfer learning", 30), t("CNN", 23)],
};
