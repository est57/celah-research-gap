import { useState } from "react";
import { isDesktop } from "../../lib/api";
import { deleteOrigin } from "../../lib/apiCollection";
import { formatInt } from "../../lib/format";
import type { OriginCount } from "../../lib/types";
import { t } from "../../i18n";

/** Collapsed by default: removes every document of one import source after confirming. */
export function OriginRemove({ origins, onDeleted }: { origins: OriginCount[]; onDeleted: (removed: number) => void }) {
  const [pending, setPending] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  if (origins.length === 0) return null;

  const remove = async (origin: string, count: number) => {
    setBusy(true);
    setError(null);
    try {
      const removed = isDesktop() ? await deleteOrigin(origin) : count;
      setPending(null);
      onDeleted(removed);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <details className="origin-remove">
      <summary>{t("Hapus dokumen per sumber impor")}</summary>
      <ul>
        {origins.map((o) => (
          <li key={o.origin}>
            <span>{o.origin}</span>
            <span className="num muted">{t("{n} dokumen", { n: formatInt(o.count) })}</span>
            {pending === o.origin ? (
              <>
                <span className="delete-warn">{t("Hapus semuanya?")}</span>
                <button type="button" className="btn" disabled={busy} onClick={() => setPending(null)}>{t("Batal")}</button>
                <button type="button" className="btn btn-danger" disabled={busy} onClick={() => void remove(o.origin, o.count)}>{t("Ya, hapus")}</button>
              </>
            ) : (
              <button type="button" className="btn" onClick={() => setPending(o.origin)}>{t("Hapus semua")}</button>
            )}
          </li>
        ))}
      </ul>
      {error ? <p className="delete-error" role="alert">{error}</p> : null}
    </details>
  );
}
