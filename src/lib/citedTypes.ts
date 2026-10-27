export type CitedSort = "global" | "local" | "normalized";

export interface CitedDoc {
  id: number;
  title: string;
  authors: string;
  year: number | null;
  source: string | null;
  doi: string | null;
  global: number;
  local: number;
  perYear: number;
  /** Global citations divided by the mean of documents from the same year (1 = average). */
  normalized: number;
}

export interface Cited {
  documents: number;
  /** Citation links found inside the collection; 0 means local counts are unavailable. */
  localLinks: number;
  docs: CitedDoc[];
}
