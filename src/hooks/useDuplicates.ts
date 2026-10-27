import { useEffect, useState } from "react";
import { isDesktop } from "../lib/api";
import { dismissDuplicate, duplicates, mergeDuplicate } from "../lib/apiCollection";
import type { DupPair } from "../lib/duplicateTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleDuplicates").then((m) => m.SAMPLE_DUPLICATES) : null;

const pairKey = (p: DupPair) => `${p.a.id}-${p.b.id}`;

/** Duplicate candidates with a local list that merges and dismissals update at once. */
export function useDuplicates(version: number, onMerged: () => void) {
  const remote = useDesktopData(duplicates, sample, version);
  const [pairs, setPairs] = useState<DupPair[]>([]);
  const [busy, setBusy] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);

  useEffect(() => setPairs(remote.data?.pairs ?? []), [remote.data]);

  const run = async (pair: DupPair, action: () => Promise<void>, removes: (p: DupPair) => boolean) => {
    setBusy(pairKey(pair));
    setActionError(null);
    try {
      if (isDesktop()) await action();
      setPairs((list) => list.filter((p) => !removes(p)));
    } catch (err) {
      setActionError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(null);
    }
  };

  const merge = (pair: DupPair, keep: number) => {
    const drop = keep === pair.a.id ? pair.b.id : pair.a.id;
    // Every other pair that mentions the removed record is gone too.
    return run(pair, () => mergeDuplicate(keep, drop).then(onMerged), (p) => p.a.id === drop || p.b.id === drop);
  };

  const dismiss = (pair: DupPair) => run(pair, () => dismissDuplicate(pair.a.id, pair.b.id), (p) => pairKey(p) === pairKey(pair));

  return { ...remote, pairs, busy, actionError, merge, dismiss, pairKey };
}
