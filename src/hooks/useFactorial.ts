import { factorial } from "../lib/api";
import type { FactorialParams } from "../lib/factorialTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleFactorial").then((m) => m.SAMPLE_FACTORIAL) : null;

export function useFactorial(version: number, p: FactorialParams) {
  return useDesktopData(() => factorial(p), sample, `${version}|${p.field}|${p.words}|${p.clusters}`);
}
