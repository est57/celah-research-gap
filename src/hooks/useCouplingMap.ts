import { couplingMap } from "../lib/apiEntities";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleCouplingMap").then((m) => m.SAMPLE_COUPLING_MAP) : null;

export function useCouplingMap(version: number, documents: number) {
  return useDesktopData(() => couplingMap(documents), sample, `${version}|${documents}`);
}
