export type CouplingQuadrant = "motor" | "niche" | "basic" | "peripheral";

export const COUPLING_QUADRANT: Record<CouplingQuadrant, string> = {
  motor: "Front inti berdampak",
  niche: "Niche berdampak",
  basic: "Arus utama, dampak rendah",
  peripheral: "Pinggiran",
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
