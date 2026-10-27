import { conceptual } from "../lib/api";
import type { ConceptualParams } from "../lib/conceptualTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleConceptual").then((m) => m.SAMPLE_CONCEPTUAL) : null;

/** Recomputes when the collection changes or when new parameters are applied. */
export function useConceptual(version: number, params: ConceptualParams) {
  const key = `${version}|${params.field}|${params.words}|${params.minClusterFreq}|${params.labels}|${params.trendMinFreq}`;
  return useDesktopData(() => conceptual(params), sample, key);
}
