import { useCallback, useEffect, useState } from "react";
import { t } from "../i18n";
import { isDesktop } from "../lib/api";
import { deleteWatch, listWatches, runWatch, type Watch, type WatchRun } from "../lib/apiWatch";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleWatches").then((m) => m.SAMPLE_WATCHES) : null;

/** Saved topic monitors with run and delete; the preview keeps them in memory. */
export function useWatches(onAdded: () => void) {
  const [watches, setWatches] = useState<Watch[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [running, setRunning] = useState<number | null>(null);
  const desktop = isDesktop();

  const load = useCallback(() => {
    const get = desktop ? listWatches() : (sample?.() ?? Promise.resolve([]));
    get.then((w) => (setWatches(w), setError(null))).catch((e: unknown) => setError(e instanceof Error ? e.message : String(e)));
  }, [desktop]);
  useEffect(load, [load]);

  const run = async (id: number): Promise<WatchRun | null> => {
    if (!desktop) {
      setError(t("Pratinjau browser: pencarian hanya berjalan di aplikasi desktop."));
      return null;
    }
    setRunning(id);
    setError(null);
    try {
      const r = await runWatch(id);
      if (r.added > 0) onAdded();
      load();
      return r;
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
      return null;
    } finally {
      setRunning(null);
    }
  };
  const remove = async (id: number) => {
    if (desktop) await deleteWatch(id).catch((e: unknown) => setError(e instanceof Error ? e.message : String(e)));
    setWatches((w) => (w ?? []).filter((x) => x.id !== id));
  };
  return { watches, error, running, run, remove, reload: load, sample: !desktop };
}
