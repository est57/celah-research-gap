export type CitationStyle = "apa" | "ieee";
export type ReportLanguage = "id" | "en";
export type ExportFormat = "markdown" | "bibtex" | "ris";

export interface ReportParams {
  style: CitationStyle;
  language: ReportLanguage;
  includeVerification: boolean;
}

export interface ReportParagraph {
  key: string;
  heading: string;
  text: string;
}

export interface Report {
  title: string;
  intro: string;
  paragraphs: ReportParagraph[];
  referencesTitle: string;
  references: string[];
  bibtex: string;
  ris: string;
  markdown: string;
}

export const DEFAULT_REPORT_PARAMS: ReportParams = { style: "apa", language: "id", includeVerification: true };
