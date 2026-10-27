import { useCallback, useEffect, useState } from "react";
import { collectionStats, isDesktop } from "../lib/api";
import type { CollectionStats } from "../lib/types";

const EMPTY: CollectionStats = { total: 0, filtered: false, analyzed: 0, origins: [], fields: [] };

export function useCollectionStats() {
  const [stats, setStats] = useState<CollectionStats>(EMPTY);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const refresh = useCallback(async () => {
    if (!isDesktop()) return;
    setLoading(true);
    try {
      setStats(await collectionStats());
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  return { stats, error, loading, refresh };
}
