import { sourcesAuthors } from "../lib/api";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleSourcesAuthors").then((m) => m.SAMPLE_SOURCES_AUTHORS) : null;

export function useSourcesAuthors(version: number) {
  return useDesktopData(sourcesAuthors, sample, version);
}
