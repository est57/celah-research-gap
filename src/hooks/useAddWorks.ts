import { useState } from "react";
import { addWorksByDoi } from "../lib/apiEntities";
import { formatInt } from "../lib/format";

const CHUNK = 50;

/** Adds works by DOI in chunks of 50, then reports a one-line summary. */
export function useAddWorks(onAdded: () => void) {
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState<{ text: string; error: boolean } | null>(null);

  const add = async (dois: string[]) => {
    if (!dois.length || busy) return;
    setBusy(true);
    setMessage(null);
    let inserted = 0, duplicates = 0, missing = 0;
    try {
      for (let i = 0; i < dois.length; i += CHUNK) {
        const r = await addWorksByDoi(dois.slice(i, i + CHUNK));
        inserted += r.inserted;
        duplicates += r.duplicates;
        missing += r.notFound.length;
      }
      const parts = [`${formatInt(inserted)} karya ditambahkan`];
      if (duplicates) parts.push(`${formatInt(duplicates)} ternyata sudah ada`);
      if (missing) parts.push(`${formatInt(missing)} tidak ditemukan di OpenAlex`);
      setMessage({ text: `${parts.join(", ")}.`, error: false });
      if (inserted) onAdded();
    } catch (e) {
      const done = inserted ? ` ${formatInt(inserted)} karya sempat ditambahkan.` : "";
      setMessage({ text: `Gagal menambahkan: ${(e instanceof Error ? e.message : String(e)).replace(/\.$/, "")}.${done}`, error: true });
      if (inserted) onAdded();
    } finally {
      setBusy(false);
    }
  };

  return { add, busy, message };
}
