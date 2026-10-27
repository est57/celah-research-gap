export interface HistNode {
  id: number;
  /** "Surname Year". */
  label: string;
  title: string;
  year: number;
  source: string | null;
  doi: string | null;
  /** Citations from documents in the collection. */
  lcs: number;
  /** Citations overall, as reported by the source. */
  gcs: number;
  cluster: number;
}

export interface Historiograph {
  documents: number;
  links: number;
  byId: number;
  byDoi: number;
  byName: number;
  nodes: HistNode[];
  /** [citing, cited] as indices into `nodes`. */
  edges: [number, number][];
}
