import { historiograph } from "../lib/api";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleHistoriograph").then((m) => m.SAMPLE_HISTORIOGRAPH) : null;

export function useHistoriograph(version: number, nodes: number) {
  return useDesktopData(() => historiograph(nodes), sample, `${version}|${nodes}`);
}
