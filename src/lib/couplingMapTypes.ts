import { tk } from "../i18n";
export type CouplingQuadrant = "motor" | "niche" | "basic" | "peripheral";

export const COUPLING_QUADRANT: Record<CouplingQuadrant, string> = {
  motor: tk("Front inti berdampak"),
  niche: tk("Niche berdampak"),
  basic: tk("Arus utama, dampak rendah"),
  peripheral: tk("Pinggiran"),
};

export interface ClusterDoc {
  id: number;
  title: string;
  year: number | null;
  citedBy: number;
}

export interface CouplingCluster {
  id: number;
  size: number;
  keywords: string[];
  centrality: number;
  /** Mean year-normalized citations; 1.0 is the collection average. */
  impact: number;
  /** Rank positions in 0..1. */
  x: number;
  y: number;
  quadrant: CouplingQuadrant;
  documents: ClusterDoc[];
}

export interface CouplingMap {
  coupled: number;
  used: number;
  clusters: CouplingCluster[];
}
