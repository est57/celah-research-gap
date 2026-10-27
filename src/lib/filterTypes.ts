export interface CollectionFilter {
  yearFrom: number | null;
  yearTo: number | null;
  /** Stored document types; "" stands for documents without a type. Empty = all. */
  docTypes: string[];
  origins: string[];
  /** Publication sources; "" for documents without one. Empty = all. */
  sources: string[];
}

export interface Choice {
  value: string;
  count: number;
}

export interface FilterOptions {
  total: number;
  yearMin: number | null;
  yearMax: number | null;
  docTypes: Choice[];
  origins: Choice[];
  sources: Choice[];
  filter: CollectionFilter;
  matching: number;
}

export const NO_FILTER: CollectionFilter = { yearFrom: null, yearTo: null, docTypes: [], origins: [], sources: [] };

export function isActive(f: CollectionFilter): boolean {
  return f.yearFrom !== null || f.yearTo !== null || f.docTypes.length > 0 || f.origins.length > 0 || f.sources.length > 0;
}
