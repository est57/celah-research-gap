import type { FieldChoice } from "./conceptualTypes";

export interface FactorialParams {
  field: FieldChoice;
  words: number;
  /** 0 picks the number of clusters automatically. */
  clusters: number;
}

export const DEFAULT_FACTORIAL: FactorialParams = { field: "auto", words: 50, clusters: 0 };

export interface FactorWord {
  label: string;
  freq: number;
  x: number;
  y: number;
  cluster: number;
}

export interface FactorCluster {
  id: number;
  size: number;
  top: string[];
  x: number;
  y: number;
}

export interface FactorDoc {
  id: number;
  title: string;
  year: number | null;
  x: number;
  y: number;
  cluster: number;
}

export interface Factorial {
  field: string;
  documents: number;
  /** Benzecri-corrected inertia of axes 1 and 2, in 0..1. */
  explained: [number, number];
  words: FactorWord[];
  clusters: FactorCluster[];
  topDocuments: FactorDoc[];
}
