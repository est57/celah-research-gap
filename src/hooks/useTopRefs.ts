import { topRefs } from "../lib/apiEntities";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleTopRefs").then((m) => m.SAMPLE_TOP_REFS) : null;

export function useTopRefs(version: number, top: number) {
  return useDesktopData(() => topRefs(top), sample, `${version}|${top}`);
}
