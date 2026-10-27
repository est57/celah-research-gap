import { useEffect, useState } from "react";
import { gapHistory, isDesktop, setGapReview } from "../lib/api";
import type { ReviewEntry } from "../lib/gapTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleGapHistory").then((m) => m.SAMPLE_GAP_HISTORY) : null;

/** Stored reviews; loads when the history tab opens and after each removal. */
export function useGapHistory(version: number) {
  const remote = useDesktopData(gapHistory, sample, version);
  const [items, setItems] = useState<ReviewEntry[]>([]);
  const [actionError, setActionError] = useState<string | null>(null);
  useEffect(() => setItems(remote.data ?? []), [remote.data]);

  const remove = async (key: string) => {
    try {
      if (isDesktop()) await setGapReview(key, "", "", "");
      setItems((list) => list.filter((e) => e.key !== key));
      setActionError(null);
    } catch (err) {
      setActionError(err instanceof Error ? err.message : String(err));
    }
  };

  return { ...remote, items, remove, actionError };
}
