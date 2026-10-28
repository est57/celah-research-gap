import { tk } from "../i18n";
export type FieldChoice = "auto" | "author" | "plus";
export type Quadrant = "motor" | "niche" | "emerging" | "basic";

export interface ConceptualParams {
  field: FieldChoice;
  words: number;
  minClusterFreq: number;
  labels: number;
  trendMinFreq: number;
}

export const DEFAULT_PARAMS: ConceptualParams = {
  field: "auto",
  words: 250,
  minClusterFreq: 5,
  labels: 3,
  trendMinFreq: 5,
};

export interface ThemeWord {
  label: string;
  freq: number;
}

export interface Theme {
  id: number;
  name: string;
  words: ThemeWord[];
  frequency: number;
  documents: number;
  centrality: number;
  density: number;
  x: number;
  y: number;
  quadrant: Quadrant;
  recentShare: number;
}

export interface Trend {
  label: string;
  freq: number;
  q1: number;
  median: number;
  q3: number;
}

export interface Conceptual {
  field: string;
  wordsUsed: number;
  documents: number;
  map: { themes: Theme[]; collectionRecentShare: number; recentFrom: number; recentTo: number };
  trends: Trend[];
}

export const QUADRANT_LABEL: Record<Quadrant, string> = {
  motor: tk("Tema motor"),
  niche: tk("Tema niche"),
  emerging: tk("Tema baru muncul atau menurun"),
  basic: tk("Tema dasar"),
};

/** A theme in the emerging/declining quadrant that is growing faster than the collection. */
export function isEmergingSignal(t: Theme, collectionShare: number): boolean {
  return t.quadrant === "emerging" && t.recentShare > collectionShare;
}
