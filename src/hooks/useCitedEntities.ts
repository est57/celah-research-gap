import { citedEntities } from "../lib/apiEntities";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleCitedEntities").then((m) => m.SAMPLE_CITED_ENTITIES) : null;

export function useCitedEntities(version: number) {
  return useDesktopData(citedEntities, sample, version);
}
