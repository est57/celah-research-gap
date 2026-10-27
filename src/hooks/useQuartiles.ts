import { quartiles } from "../lib/apiJournal";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleQuartiles").then((m) => m.SAMPLE_QUARTILES) : null;

export function useQuartiles(version: number) {
  return useDesktopData(quartiles, sample, version);
}
