export interface TopRef {
  label: string;
  year: number;
  /** The reference as one citing document wrote it. */
  example: string;
  doi: string | null;
  documents: number;
  /** Stored document it points to, searched in the whole collection. */
  inCollection: number | null;
  title: string | null;
}

export interface TopRefs {
  documents: number;
  citing: number;
  works: number;
  refs: TopRef[];
  /** Listed works not found in the collection. */
  missing: number;
}
