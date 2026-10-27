export interface DupPaper {
  id: number;
  title: string;
  year: number | null;
  origin: string;
  source: string | null;
  docType: string | null;
  doi: string | null;
  authors: string;
  citedBy: number;
  /** Optional fields this record has, e.g. "Abstrak". */
  has: string[];
}

export interface DupPair {
  a: DupPaper;
  b: DupPaper;
  reason: "same_title" | "similar_title";
  similarity: number;
  /** Suggested record to keep. */
  keep: number;
}

export interface DuplicateList {
  documents: number;
  pairs: DupPair[];
  truncated: boolean;
}
