import { useEffect, useState } from "react";
import { isDesktop } from "../lib/api";
import { completeTextReferences, textReferencesPending, type EnrichReport } from "../lib/apiCollection";
import { formatInt } from "../lib/format";
import { t } from "../i18n";

/** Fetches reference lists from Crossref by DOI for documents that have none, then reloads the analyses. */
export function CompleteTextRefs({ onDone }: { onDone: () => void }) {
  const [pending, setPending] = useState<number | null>(null);
  const [running, setRunning] = useState(false);
  const [report, setReport] = useState<EnrichReport | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (isDesktop()) textReferencesPending().then(setPending).catch(() => setPending(null));
  }, []);

  if (!isDesktop()) return <p className="muted refs-help">{t("Melengkapi rujukan dari Crossref hanya berjalan di aplikasi desktop.")}</p>;

  const run = async () => {
    setRunning(true);
    setError(null);
    try {
      const r = await completeTextReferences();
      setReport(r);
      setPending(r.remaining);
      if (r.gained > 0) onDone();
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setRunning(false);
    }
  };

  return (
    <div className="text-refs">
      <span className="refs-help">
        {pending === null ? t("Memeriksa dokumen ber-DOI tanpa rujukan...")
          : pending === 0 ? t("Semua dokumen ber-DOI sudah dicek ke Crossref.")
          : t("{n} dokumen punya DOI tetapi belum punya rujukan. Crossref menyimpan daftar rujukan untuk banyak penerbit.", { n: formatInt(pending) })}
      </span>
      {pending ? (
        <button type="button" className="btn btn-primary" disabled={running} onClick={() => void run()}>
          {running ? t("Mengambil rujukan...") : t("Lengkapi rujukan dari Crossref")}
        </button>
      ) : null}
      {running ? <span className="muted refs-help">{t("Sekitar 15 detik per 600 dokumen. Bisa dijalankan lagi untuk sisanya.")}</span> : null}
      {report ? (
        <span className="refs-help" role="status">
          {t("{n} dokumen dicek, {gained} kini punya daftar rujukan.", { n: formatInt(report.checked), gained: formatInt(report.gained) })}
          {report.remaining ? ` ${t("{n} lagi menunggu; jalankan sekali lagi.", { n: formatInt(report.remaining) })}` : ""}
        </span>
      ) : null}
      {report?.error || error ? <span className="refs-error" role="alert">{report?.error ?? error}</span> : null}
    </div>
  );
}
