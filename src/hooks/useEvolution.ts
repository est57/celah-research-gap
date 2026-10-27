import { evolution } from "../lib/api";
import type { EvolutionParams } from "../lib/evolutionTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleEvolution").then((m) => m.SAMPLE_EVOLUTION) : null;

export function useEvolution(version: number, p: EvolutionParams) {
  const key = `${version}|${p.field}|${p.words}|${p.minClusterFreq}|${p.labels}|${p.cuts.join(",")}`;
  return useDesktopData(() => evolution(p), sample, key);
}
