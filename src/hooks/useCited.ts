import { cited } from "../lib/api";
import type { CitedSort } from "../lib/citedTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const loadSample = import.meta.env.DEV ? () => import("../dev/sampleCited") : null;

export function useCited(version: number, sort: CitedSort, limit: number) {
  const sample = loadSample ? () => loadSample().then((m) => m.sampleCited(sort)) : null;
  return useDesktopData(() => cited(sort, limit), sample, `${version}|${sort}|${limit}`);
}
