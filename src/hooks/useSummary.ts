import { reportSummary, type SummaryParams } from "../lib/apiReport";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleSummary").then((m) => m.SAMPLE_SUMMARY) : null;

export function useSummary(version: number, params: SummaryParams) {
  return useDesktopData(() => reportSummary(params), sample, `${version}|${params.style}|${params.language}`);
}
