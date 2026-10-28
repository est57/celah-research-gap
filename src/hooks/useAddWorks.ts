import { useState } from "react";
import { t } from "../i18n";
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
      const parts = [t("{n} karya ditambahkan", { n: formatInt(inserted) })];
      if (duplicates) parts.push(t("{n} ternyata sudah ada", { n: formatInt(duplicates) }));
      if (missing) parts.push(t("{n} tidak ditemukan di OpenAlex", { n: formatInt(missing) }));
      setMessage({ text: `${parts.join(", ")}.`, error: false });
      if (inserted) onAdded();
    } catch (e) {
      const done = inserted ? ` ${t("{n} karya sempat ditambahkan.", { n: formatInt(inserted) })}` : "";
      setMessage({ text: `${t("Gagal menambahkan: {reason}.", { reason: (e instanceof Error ? e.message : String(e)).replace(/\.$/, "") })}${done}`, error: true });
      if (inserted) onAdded();
    } finally {
      setBusy(false);
    }
  };

  return { add, busy, message };
}
