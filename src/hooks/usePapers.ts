import { useCallback, useEffect, useState } from "react";
import { isDesktop, listPapers } from "../lib/api";
import type { PaperPage } from "../lib/types";

export const PAGE_SIZE = 50;

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/samplePapers") : null;

// Loads one page of papers. `version` changes whenever the collection changes.
export function usePapers(filter: string, page: number, version: number) {
  const [data, setData] = useState<PaperPage>({ items: [], total: 0 });
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const load = useCallback(async () => {
    if (!isDesktop()) {
      if (sample) setData((await sample()).samplePapers(filter));
      return;
    }
    setLoading(true);
    try {
      const trimmed = filter.trim();
      const res = await listPapers({
        filter: trimmed.length ? trimmed : null,
        limit: PAGE_SIZE,
        offset: page * PAGE_SIZE,
      });
      setData(res);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [filter, page]);

  useEffect(() => {
    const t = window.setTimeout(() => void load(), 200);
    return () => window.clearTimeout(t);
  }, [load, version]);

  return { data, error, loading };
}
