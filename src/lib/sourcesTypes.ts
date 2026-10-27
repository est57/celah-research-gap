export interface NamedCount {
  label: string;
  count: number;
}

export interface BradfordItem {
  rank: number;
  label: string;
  count: number;
  zone: 1 | 2 | 3;
}

export interface ZoneSummary {
  sources: number;
  documents: number;
}

export interface LotkaRow {
  documents: number;
  authors: number;
  observed: number;
  theoretical: number;
}

export interface YearCell {
  year: number;
  documents: number;
  tcPerYear: number;
}

export interface Timeline {
  label: string;
  documents: number;
  years: YearCell[];
}

export interface Impact {
  label: string;
  h: number;
  g: number;
  m: number | null;
  citations: number;
  documents: number;
  firstYear: number | null;
}

export interface SourcesAuthors {
  currentYear: number;
  totalSources: number;
  topSources: NamedCount[];
  bradford: { ranked: BradfordItem[]; zones: ZoneSummary[] };
  lotka: { totalAuthors: number; rows: LotkaRow[]; beta: number | null };
  production: Timeline[];
  impact: Impact[];
}
