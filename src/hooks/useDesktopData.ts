import { useEffect, useState } from "react";
import { isDesktop } from "../lib/api";

/**
 * Loads data from the backend, reloading whenever `reloadKey` changes.
 * In the browser preview it falls back to `sample`, which callers pass as null
 * outside dev builds so the sample module never reaches production bundles.
 */
export function useDesktopData<T>(fetcher: () => Promise<T>, sample: (() => Promise<T>) | null, reloadKey: number | string) {
  const [data, setData] = useState<T | null>(null);
  const [isSample, setIsSample] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    let cancelled = false;
    const desktop = isDesktop();
    const run = async (): Promise<T> => {
      if (desktop) return fetcher();
      if (sample) return sample();
      throw new Error("Analisis dihitung dari database lokal dan hanya tersedia di aplikasi desktop.");
    };
    setLoading(true);
    run()
      .then((res) => {
        if (cancelled) return;
        setData(res);
        setIsSample(!desktop);
        setError(null);
      })
      .catch((err: unknown) => {
        if (!cancelled) setError(err instanceof Error ? err.message : String(err));
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
    // Callers encode everything the fetch depends on in `reloadKey`.
  }, [reloadKey]);

  return { data, sample: isSample, error, loading };
}
