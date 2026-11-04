import { useEffect, useState } from "react";
import { isDesktop } from "../lib/api";
import { screeningPage, setScreening } from "../lib/apiScreening";
import type { Decision, Reason, ScreenPage, ScreenRow, ScreenStatus } from "../lib/screeningTypes";

export const SCREEN_PAGE = 20;
const EMPTY: ScreenPage = { items: [], total: 0, counts: { unscreened: 0, included: 0, excluded: 0 } };

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleScreening").then((m) => m.SAMPLE_SCREEN_ROWS) : null;

/** The preview keeps decisions in memory so the screen can be tried without the backend. */
function local(rows: ScreenRow[], status: ScreenStatus, query: string, page: number): ScreenPage {
  const q = query.trim().toLowerCase();
  const hit = rows.filter((r) => (status === "all" || (status === "none" ? r.decision === "" : r.decision === status)) && (!q || r.title.toLowerCase().includes(q)));
  const counts = { unscreened: rows.filter((r) => !r.decision).length, included: rows.filter((r) => r.decision === "include").length, excluded: rows.filter((r) => r.decision === "exclude").length };
  return { items: hit.slice(page * SCREEN_PAGE, (page + 1) * SCREEN_PAGE), total: hit.length, counts };
}

/** One page of documents to screen, and the action that records decisions. */
export function useScreening(status: ScreenStatus, query: string, page: number, onChanged: () => void) {
  const [data, setData] = useState<ScreenPage>(EMPTY);
  const [rows, setRows] = useState<ScreenRow[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [version, setVersion] = useState(0);
  const desktop = isDesktop();

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    const load = desktop
      ? screeningPage(status, query, SCREEN_PAGE, page * SCREEN_PAGE)
      : (rows ? Promise.resolve(rows) : (sample?.() ?? Promise.resolve([]))).then((all) => {
          if (!rows) setRows(all);
          return local(all, status, query, page);
        });
    load
      .then((res) => !cancelled && (setData(res), setError(null)))
      .catch((e: unknown) => !cancelled && setError(e instanceof Error ? e.message : String(e)))
      .finally(() => !cancelled && setLoading(false));
    return () => {
      cancelled = true;
    };
  }, [desktop, status, query, page, version, rows]);

  const decide = async (ids: number[], decision: Decision | null, reason: Reason | "" = "", note = "") => {
    if (!desktop) {
      setRows((all) => (all ?? []).map((r) => (ids.includes(r.id) ? { ...r, decision: decision ?? "", reason: decision === "exclude" ? reason : "", note: decision === "exclude" ? note : "" } : r)));
      return;
    }
    try {
      await setScreening(ids, decision, reason, note);
      setVersion((v) => v + 1);
      onChanged();
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  };
  return { data, error, loading, decide, sample: !desktop };
}
