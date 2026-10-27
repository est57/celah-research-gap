import { report } from "../lib/api";
import type { ReportParams } from "../lib/reportTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleReport").then((m) => m.SAMPLE_REPORT) : null;

/** Rebuilds the draft when the collection or any option changes. */
export function useReport(version: number, params: ReportParams) {
  const key = `${version}|${params.style}|${params.language}|${params.includeVerification}`;
  return useDesktopData(() => report(params), sample, key);
}
