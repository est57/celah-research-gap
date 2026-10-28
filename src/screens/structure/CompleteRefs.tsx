import { useEffect, useState } from "react";
import { isDesktop } from "../../lib/api";
import { completeReferences, referencesPending, type EnrichReport } from "../../lib/apiCollection";
import { formatInt } from "../../lib/format";
import { t } from "../../i18n";

/** Fetches cited works and institutions from OpenAlex for documents missing them, then reloads the analyses. */
export function CompleteRefs({ version, onDone }: { version: number; onDone: () => void }) {
  const [pending, setPending] = useState<number | null>(null);
  const [running, setRunning] = useState(false);
  const [report, setReport] = useState<EnrichReport | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!isDesktop()) return;
    referencesPending().then(setPending).catch(() => setPending(null));
  }, [version]);

  const run = async () => {
    setRunning(true);
    setError(null);
    try {
      const r = await completeReferences();
      setReport(r);
      setPending(r.remaining);
      if (r.gained > 0) onDone();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setRunning(false);
    }
  };

  if (!isDesktop()) {
    return (
      <div className="refs-box">
        <p className="muted refs-note">{t("Melengkapi rujukan hanya berjalan di aplikasi desktop.")}</p>
      </div>
    );
  }
  if (pending === 0 && !report) return null;
  return (
    <div className="refs-box">
      <p className="refs-note">
        {pending === null ? t("Memeriksa dokumen tanpa daftar rujukan...")
          : t("{n} dokumen punya ID OpenAlex atau DOI tetapi belum punya daftar rujukan atau institusi.", { n: formatInt(pending) })}
      </p>
      {pending ? (
        <button type="button" className="btn btn-primary" disabled={running} onClick={() => void run()}>
          {running ? t("Mengambil data...") : t("Lengkapi rujukan & institusi dari OpenAlex")}
        </button>
      ) : null}
      {running ? <p className="muted refs-note">{t("Sekitar 1 detik per 50 dokumen, paling banyak 2.000 sekali jalan.")}</p> : null}
      {report ? (
        <p className="refs-note" role="status">
          {t("{checked} dokumen diperiksa, {gained} kini punya rujukan baru.", { checked: formatInt(report.checked), gained: formatInt(report.gained) })}
          {report.remaining ? ` ${t("{n} lagi menunggu; jalankan sekali lagi.", { n: formatInt(report.remaining) })}` : ""}
        </p>
      ) : null}
      {report?.error || error ? <p className="refs-error" role="alert">{report?.error ?? error}</p> : null}
    </div>
  );
}
