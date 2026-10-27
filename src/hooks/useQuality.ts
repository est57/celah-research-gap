import { metadataQuality } from "../lib/apiCollection";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleQuality").then((m) => m.SAMPLE_QUALITY) : null;

export function useQuality(version: number) {
  return useDesktopData(metadataQuality, sample, version);
}
