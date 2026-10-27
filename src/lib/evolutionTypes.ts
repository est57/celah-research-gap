import type { FieldChoice, Quadrant, Theme } from "./conceptualTypes";

export interface EvolutionParams {
  field: FieldChoice;
  words: number;
  minClusterFreq: number;
  labels: number;
  /** Cut years; empty for an automatic split. */
  cuts: number[];
}

export const DEFAULT_EVOLUTION: EvolutionParams = { field: "auto", words: 250, minClusterFreq: 5, labels: 2, cuts: [] };

export interface Period {
  from: number;
  to: number;
  documents: number;
  themes: Theme[];
}

export interface ThemeLink {
  /** Index of the earlier period; the link goes to period + 1. */
  period: number;
  from: number;
  to: number;
  inclusion: number;
  outValue: number;
  inValue: number;
  shared: string[];
}

export interface Evolution {
  field: string;
  cuts: number[];
  periods: Period[];
  links: ThemeLink[];
}

export interface ThemeRef {
  period: number;
  id: number;
}

export const QUADRANT_COLOR: Record<Quadrant, string> = {
  motor: "#2456A6",
  basic: "#2E8B6E",
  niche: "#7A5BB0",
  emerging: "#D9622B",
};

export const periodLabel = (p: Period) => (p.from === p.to ? `${p.from}` : `${p.from}-${p.to}`);

/** Themes of period `p` that no theme of the previous period flows into, at this threshold. */
export function newThemes(e: Evolution, p: number, minInclusion: number): Theme[] {
  if (p === 0) return [];
  return e.periods[p].themes.filter((t) => !e.links.some((l) => l.period === p - 1 && l.to === t.id && l.inclusion >= minInclusion));
}

/** Themes of period `p` that flow into no theme of the next period, at this threshold. */
export function endedThemes(e: Evolution, p: number, minInclusion: number): Theme[] {
  if (p >= e.periods.length - 1) return [];
  return e.periods[p].themes.filter((t) => !e.links.some((l) => l.period === p && l.from === t.id && l.inclusion >= minInclusion));
}
