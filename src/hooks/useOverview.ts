import { overview } from "../lib/api";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleOverview").then((m) => m.SAMPLE_OVERVIEW) : null;

/** Reloads whenever `version` changes (for example the collection size). */
export function useOverview(version: number) {
  return useDesktopData(overview, sample, version);
}
