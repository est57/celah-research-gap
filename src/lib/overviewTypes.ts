import { tk } from "../i18n";
export interface TypeCount {
  docType: string;
  count: number;
}

export interface MainInfo {
  yearMin: number | null;
  yearMax: number | null;
  documents: number;
  sources: number;
  growthRate: number | null;
  avgAge: number | null;
  avgCitations: number;
  references: number;
  keywordsPlus: number;
  authorKeywords: number;
  authors: number;
  singleAuthorDocs: number;
  authorsOfSingle: number;
  coauthorsPerDoc: number | null;
  internationalCoauthorship: number | null;
  docTypes: TypeCount[];
}

export interface YearPoint {
  year: number;
  documents: number;
  /** Mean citations per article (MeanTCperArt). */
  meanTc: number;
  /** Years the documents could be cited, including the publication year. */
  citableYears: number;
  meanTcPerYear: number;
  partial: boolean;
}

export interface FieldNode {
  label: string;
  count: number;
}

export interface FieldLink {
  from: number;
  to: number;
  value: number;
}

export type ThreeFieldId = "authors" | "author_keywords" | "keywords_plus" | "sources" | "countries" | "references";

export const THREE_FIELD_LABEL: Record<ThreeFieldId, string> = {
  authors: tk("Penulis"),
  author_keywords: tk("Kata kunci penulis"),
  keywords_plus: tk("Keywords Plus"),
  sources: tk("Sumber"),
  countries: tk("Negara"),
  references: tk("Referensi tersitasi"),
};

export interface ThreeFieldParams {
  left: ThreeFieldId;
  middle: ThreeFieldId;
  right: ThreeFieldId;
  items: number;
}

export interface ThreeField {
  /** Column headings, left to right. */
  labels: [string, string, string];
  /** "countries" marks a column of ISO codes shown as country names. */
  kinds: [string, string, string];
  left: FieldNode[];
  middle: FieldNode[];
  right: FieldNode[];
  leftLinks: FieldLink[];
  rightLinks: FieldLink[];
}

export interface Overview {
  currentYear: number;
  info: MainInfo;
  annual: YearPoint[];
  threeField: ThreeField;
}
