export type Unit = "references" | "authors" | "keywords" | CouplingUnit;

/** Bibliographic coupling of documents, sources or authors. */
export type CouplingUnit = "coupling_documents" | "coupling_sources" | "coupling_authors";

export interface StructureParams {
  unit: Unit;
  nodes: number;
}

export interface NetNode {
  label: string;
  count: number;
  cluster: number;
}

export interface NetEdge {
  from: number;
  to: number;
  count: number;
  weight: number;
}

export interface NetCluster {
  id: number;
  size: number;
  top: string[];
}

export interface Bridge {
  a: number;
  b: number;
  links: number;
  ratio: number;
}

export interface Structure {
  unit: string;
  documents: number;
  network: { nodes: NetNode[]; edges: NetEdge[]; clusters: NetCluster[]; bridges: Bridge[] };
}

const CLUSTER_COLORS = ["#2456A6", "#2E8B6E", "#7A5BB0", "#D9622B", "#B8457A", "#8C7A3E", "#3B8FB5", "#6B7480"];

export function clusterColor(id: number): string {
  return CLUSTER_COLORS[id % CLUSTER_COLORS.length];
}
