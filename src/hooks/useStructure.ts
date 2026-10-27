import { structure } from "../lib/api";
import type { StructureParams } from "../lib/structureTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const loadSample = import.meta.env.DEV ? () => import("../dev/sampleStructure") : null;

export function useStructure(version: number, params: StructureParams) {
  const sample = loadSample ? () => loadSample().then((m) => m.sampleStructure(params.unit)) : null;
  return useDesktopData(() => structure(params), sample, `${version}|${params.unit}|${params.nodes}`);
}
