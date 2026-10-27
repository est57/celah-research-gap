import { useState } from "react";
import { isDesktop } from "../lib/api";
import { authorNames, mergeAuthors, splitAuthor } from "../lib/apiEntities";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleAuthorNames").then((m) => m.SAMPLE_AUTHOR_NAMES) : null;

/** Name suggestions and merges; each change reloads them and tells the app the data changed. */
export function useAuthorNames(version: number, onChanged: () => void) {
  const [rev, setRev] = useState(0);
  const remote = useDesktopData(authorNames, sample, `${version}|${rev}`);
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
    merge: (keys: string[], target: string) => run(() => mergeAuthors(keys, target)),
    /** Several merges in a row, e.g. every reordered-name suggestion. */
    mergeAll: (groups: { keys: string[]; target: string }[]) => run(async () => {
      for (const g of groups) await mergeAuthors(g.keys, g.target);
    }),
    split: (fromKey: string) => run(() => splitAuthor(fromKey)),
  };
}
