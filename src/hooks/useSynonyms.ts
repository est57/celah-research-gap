import { useState } from "react";
import { isDesktop } from "../lib/api";
import { keywordSynonyms, mergeKeywords, removeSynonym } from "../lib/apiEntities";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleSynonyms").then((m) => m.SAMPLE_SYNONYMS) : null;

/** Synonym suggestions and mappings; each change reloads them and tells the app the data changed. */
export function useSynonyms(version: number, onChanged: () => void) {
  const [rev, setRev] = useState(0);
  const remote = useDesktopData(keywordSynonyms, sample, `${version}|${rev}`);
  const [busy, setBusy] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);

  const run = async (task: () => Promise<unknown>) => {
    setBusy(true);
    setActionError(null);
    try {
      if (isDesktop()) await task();
      setRev((r) => r + 1);
      onChanged();
    } catch (err) {
      setActionError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  };

  return {
    ...remote,
    busy,
    actionError,
    merge: (labels: string[], target: string) => run(() => mergeKeywords(labels, target)),
    remove: (fromKey: string) => run(() => removeSynonym(fromKey)),
  };
}
