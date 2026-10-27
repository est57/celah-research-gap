export interface RpysParams {
  /** First and last cited year; 0 picks them from the data. */
  from: number;
  to: number;
}

export const DEFAULT_RPYS: RpysParams = { from: 0, to: 0 };

export interface PeakRef {
  label: string;
  example: string;
  documents: number;
}

export interface RpysPeak {
  year: number;
  citations: number;
  deviation: number;
  refs: PeakRef[];
}

export interface Rpys {
  documents: number;
  citing: number;
  dated: number;
  undated: number;
  from: number;
  to: number;
  years: number[];
  citations: number[];
  /** Citations minus the 5-year median, per year in `years`. */
  deviation: number[];
  /** Largest deviations first. */
  peaks: RpysPeak[];
}
