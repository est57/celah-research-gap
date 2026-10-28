import { tk } from "../i18n";
export interface SjrStatus {
  /** Distinct journals in the imported list; 0 when none is imported. */
  journals: number;
  year: number | null;
}

export interface QuartileSource {
  label: string;
  documents: number;
  /** "Q1".."Q4", or "" when the journal is not in the SJR list. */
  quartile: string;
  sjr: number | null;
}

export interface Quartiles {
  status: SjrStatus;
  documents: number;
  /** Documents per quartile: Q1, Q2, Q3, Q4, then without a quartile. */
  counts: [number, number, number, number, number];
  sources: QuartileSource[];
}

export const QUARTILE_LABELS = ["Q1", "Q2", "Q3", "Q4", tk("Tanpa kuartil")] as const;
