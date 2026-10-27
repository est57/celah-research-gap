export interface FieldInfo {
  tag: string;
  label: string;
}

export interface AnalysisInfo {
  key: string;
  label: string;
  needs: string[];
}

export interface OriginQuality {
  origin: string;
  count: number;
  /** Missing documents per field, in the order of `fields`. */
  missing: number[];
  /** Usable documents per analysis, in the order of `analyses`. */
  usable: number[];
}

export interface MetadataQuality {
  total: number;
  fields: FieldInfo[];
  analyses: AnalysisInfo[];
  origins: OriginQuality[];
}

export type Readiness = "ready" | "limited" | "poor";

export function readiness(usable: number, total: number): Readiness {
  const share = total > 0 ? usable / total : 0;
  return share >= 0.8 ? "ready" : share >= 0.4 ? "limited" : "poor";
}

export const READINESS_LABEL: Record<Readiness, string> = { ready: "Siap", limited: "Terbatas", poor: "Kurang" };
