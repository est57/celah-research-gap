// Calls for the methods and results draft (kept apart to keep api.ts short).
import { call } from "./api";
import type { CitationStyle, ExportFormat, Report, ReportLanguage } from "./reportTypes";

export interface SummaryParams {
  style: CitationStyle;
  language: ReportLanguage;
}

/** Draft methods and results paragraphs; same shape as the gap draft. */
export function reportSummary(params: SummaryParams): Promise<Report> {
  return call<Report>("report_summary", { params });
}

export function exportSummary(params: SummaryParams, format: ExportFormat): Promise<string | null> {
  return call<string | null>("export_summary", { params, format });
}
