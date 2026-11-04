import { journalTargets } from "../lib/apiJournal";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleJournals").then((m) => m.SAMPLE_JOURNALS) : null;

export function useJournalTargets(version: number) {
  return useDesktopData(journalTargets, sample, version);
}
