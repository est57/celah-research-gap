import { useEffect, useState } from "react";
import { isDesktop } from "../lib/api";
import { filterOptions, previewFilter, setCollectionFilter } from "../lib/apiCollection";
import { NO_FILTER, type CollectionFilter, type FilterOptions } from "../lib/filterTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleFilter").then((m) => m.SAMPLE_FILTER) : null;

/** Filter options, an editable draft with a live document count, and saving. */
export function useFilter(version: number, onApplied: () => void) {
  const remote = useDesktopData(filterOptions, sample, version);
  const [saved, setSaved] = useState<FilterOptions | null>(null);
  const [draft, setDraft] = useState<CollectionFilter>(NO_FILTER);
  const [preview, setPreview] = useState<number | null>(null);
  const [saving, setSaving] = useState(false);
  const [saveError, setSaveError] = useState<string | null>(null);
  const options = saved ?? remote.data;

  useEffect(() => {
    if (remote.data) {
      setSaved(null);
      setDraft(remote.data.filter);
    }
  }, [remote.data]);

  // Counts the draft shortly after the last change instead of on every keystroke.
  useEffect(() => {
    if (!isDesktop()) return;
    let cancelled = false;
    const timer = window.setTimeout(() => {
      previewFilter(draft)
        .then((n) => !cancelled && setPreview(n))
        .catch(() => !cancelled && setPreview(null));
    }, 250);
    return () => {
      cancelled = true;
      window.clearTimeout(timer);
    };
  }, [draft]);

  const apply = async (f: CollectionFilter) => {
    if (!isDesktop()) return;
    setSaving(true);
    try {
      const next = await setCollectionFilter(f);
      setSaved(next);
      setDraft(next.filter);
      setSaveError(null);
      onApplied();
    } catch (err) {
      setSaveError(err instanceof Error ? err.message : String(err));
    } finally {
      setSaving(false);
    }
  };

  return { ...remote, options, draft, setDraft, preview, saving, saveError, apply };
}
