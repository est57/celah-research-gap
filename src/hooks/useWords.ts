import { words } from "../lib/apiEntities";
import type { WordParams } from "../lib/wordTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleWords").then((m) => m.SAMPLE_WORDS) : null;

export function useWords(version: number, p: WordParams) {
  return useDesktopData(() => words(p), sample, `${version}|${p.field}|${p.ngram}|${p.top}`);
}
