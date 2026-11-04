// Target journal recommendation from the accepted gaps (see analysis/journals in the backend).
import type { SjrStatus } from "./quartileTypes";

export interface JournalPick {
  label: string;
  /** Documents of this journal in the collection. */
  documents: number;
  /** Documents that match at least one gap. */
  matching: number;
  /** Matching documents from the last three years of the collection. */
  recent: number;
  /** Total citations of the matching documents. */
  citations: number;
  /** "Q1".."Q4", or "" when the journal is not in the SJR list. */
  quartile: string;
  sjr: number | null;
  /** Titles of the gaps this journal matches. */
  gaps: string[];
  score: number;
}

export interface JournalTargets {
  status: SjrStatus;
  /** True when no gap is accepted and all candidates (except rejected ones) were used. */
  fallback: boolean;
  /** Titles of the gaps used for matching. */
  gaps: string[];
  documents: number;
  /** First year counted as recent. */
  recentFrom: number | null;
  journals: JournalPick[];
}
