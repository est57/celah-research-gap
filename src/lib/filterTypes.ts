export interface CollectionFilter {
  yearFrom: number | null;
  yearTo: number | null;
  /** Stored document types; "" stands for documents without a type. Empty = all. */
  docTypes: string[];
  origins: string[];
  /** Publication sources; "" for documents without one. Empty = all. */
  sources: string[];
  /** SJR quartiles "Q1".."Q4", or "none" for no quartile. Empty = all. */
  quartiles: string[];
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
  /** Empty until an SJR list is imported. */
  quartiles: Choice[];
  filter: CollectionFilter;
  matching: number;
}

export const NO_FILTER: CollectionFilter = { yearFrom: null, yearTo: null, docTypes: [], origins: [], sources: [], quartiles: [] };

export function isActive(f: CollectionFilter): boolean {
  return f.yearFrom !== null || f.yearTo !== null || f.docTypes.length > 0 || f.origins.length > 0 || f.sources.length > 0 || f.quartiles.length > 0;
}
