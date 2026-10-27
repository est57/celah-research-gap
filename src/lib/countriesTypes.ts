import type { NetCluster, Bridge, NetEdge, NetNode } from "./structureTypes";

export interface CountryProd {
  code: string;
  articles: number;
  scp: number;
  mcp: number;
}

export interface CountryCount {
  code: string;
  count: number;
}

export interface CountryPair {
  a: string;
  b: string;
  count: number;
}

export interface Countries {
  documentsWithCountry: number;
  internationalShare: number | null;
  corresponding: CountryProd[];
  production: CountryCount[];
  pairs: CountryPair[];
  network: { nodes: NetNode[]; edges: NetEdge[]; clusters: NetCluster[]; bridges: Bridge[] };
}
