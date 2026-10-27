import { useEffect, useState } from "react";
import { gaps, isDesktop, setGapReview, verifyGaps } from "../lib/api";
import type { Gap, ReviewStatus, VerifyReport } from "../lib/gapTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleGaps").then((m) => m.SAMPLE_GAPS) : null;

/** Gap candidates plus a local copy that review changes update immediately. */
export function useGaps(version: number) {
  const [refresh, setRefresh] = useState(0);
  const remote = useDesktopData(gaps, sample, `${version}|${refresh}`);
  const [items, setItems] = useState<Gap[]>([]);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [verifying, setVerifying] = useState(false);

  useEffect(() => setItems(remote.data?.gaps ?? []), [remote.data]);

  const review = async (key: string, status: ReviewStatus, note: string) => {
    const before = items;
    setItems((list) => list.map((g) => (g.key === key ? { ...g, status, note } : g)));
    if (!isDesktop()) return;
    try {
      await setGapReview(key, status, note, before.find((g) => g.key === key)?.title ?? "");
      setSaveError(null);
    } catch (err) {
      setItems(before);
      setSaveError(err instanceof Error ? err.message : String(err));
    }
  };

  /** Runs verification, then reloads the gaps so scores and results update. */
  const verify = async (keys: string[]): Promise<VerifyReport[]> => {
    setVerifying(true);
    try {
      const reports = await verifyGaps(keys);
      setRefresh((r) => r + 1);
      return reports;
    } finally {
      setVerifying(false);
    }
  };

  /** Reloads candidates after a change made elsewhere, such as removing a review in the history. */
  const reload = () => setRefresh((r) => r + 1);

  return { ...remote, items, review, saveError, verify, verifying, reload };
}
