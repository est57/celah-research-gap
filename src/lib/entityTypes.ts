export interface SourceRow {
  label: string;
  documents: number;
  citations: number;
  h: number;
  g: number;
  m: number | null;
  firstYear: number | null;
  /** Bradford zone, 1 = core. */
  zone: 1 | 2 | 3;
}

export interface SourcesDetail {
  currentYear: number;
  sources: SourceRow[];
  years: number[];
  /** Cumulative documents of the top sources, one value per year in `years`. */
  dynamics: { label: string; cumulative: number[] }[];
}

export interface AuthorRow {
  label: string;
  documents: number;
  /** Documents split equally among their authors. */
  fractional: number;
  citations: number;
  h: number;
  g: number;
  m: number | null;
  firstYear: number | null;
  lastYear: number | null;
}

export type EntitySort = "h" | "documents" | "citations" | "fractional";

export interface AffiliationRow {
  label: string;
  /** ISO alpha-2, when known. */
  country: string | null;
  documents: number;
  citations: number;
  firstYear: number | null;
}

export interface Affiliations {
  documents: number;
  /** Documents with at least one known institution. */
  withAffiliation: number;
  rows: AffiliationRow[];
  years: number[];
  dynamics: { label: string; cumulative: number[] }[];
}

export interface NameVariant {
  key: string;
  name: string;
  documents: number;
}

export interface NameSuggestion {
  variants: NameVariant[];
  sharedCoauthors: number;
  sharedSources: number;
  /** Same words in another order, e.g. "Antani, Sameer" and "Sameer Antani". */
  reordered: boolean;
}

export interface NameMerge {
  fromKey: string;
  fromName: string;
  toKey: string;
  toName: string;
}

export interface AuthorNames {
  suggestions: NameSuggestion[];
  merges: NameMerge[];
}

export interface CountryRow {
  /** ISO alpha-2 code. */
  code: string;
  documents: number;
  citations: number;
  avgCitations: number;
  /** Documents whose corresponding author is from the country, split into SCP and MCP. */
  corresponding: number;
  scp: number;
  mcp: number;
  firstYear: number | null;
}

export interface CountryTable {
  documents: number;
  withCountry: number;
  rows: CountryRow[];
  years: number[];
  /** Labels are country codes. */
  dynamics: { label: string; cumulative: number[] }[];
}
