export type DocType = "article" | "review" | "preprint" | "book-chapter" | "proceedings";
export type SourceId = "openalex" | "semantic_scholar" | "crossref" | "pubmed" | "europe_pmc" | "arxiv";
export type ServiceId = "openalex" | "semantic_scholar" | "ncbi";

export interface SearchInput {
  query: string;
  yearFrom: number;
  yearTo: number;
  docType: DocType | null;
  maxResults: number;
  sources: SourceId[];
}

export interface ImportReport {
  fetched: number;
  inserted: number;
  duplicates: number;
}

export interface SourceReport extends ImportReport {
  source: string;
  error: string | null;
}

export interface FileImportReport extends ImportReport {
  origin: string;
}

export interface PaperRow {
  id: number;
  title: string;
  authors: string;
  year: number | null;
  sourceName: string | null;
  docType: string | null;
  citedBy: number;
  origin: string;
}

export interface PaperPage {
  items: PaperRow[];
  total: number;
}

export interface ListInput {
  filter: string | null;
  limit: number;
  offset: number;
}

export interface OriginCount {
  origin: string;
  count: number;
}

export interface FieldStat {
  tag: string;
  label: string;
  missing: number;
}

export interface KeyStatus {
  configured: boolean;
  hint: string | null;
}

export interface CollectionStats {
  total: number;
  /** A collection filter is active; `analyzed` documents pass it. */
  filtered: boolean;
  analyzed: number;
  origins: OriginCount[];
  fields: FieldStat[];
}
