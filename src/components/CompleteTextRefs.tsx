import { useEffect, useState } from "react";
import { isDesktop } from "../lib/api";
import { completeTextReferences, textReferencesPending, type EnrichReport } from "../lib/apiCollection";
import { formatInt } from "../lib/format";

/** Fetches reference lists from Crossref by DOI for documents that have none, then reloads the analyses. */
export function CompleteTextRefs({ onDone }: { onDone: () => void }) {
  const [pending, setPending] = useState<number | null>(null);
  const [running, setRunning] = useState(false);
  const [report, setReport] = useState<EnrichReport | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (isDesktop()) textReferencesPending().then(setPending).catch(() => setPending(null));
  }, []);

  if (!isDesktop()) return <p className="muted refs-help">Melengkapi rujukan dari Crossref hanya berjalan di aplikasi desktop.</p>;

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
        {pending === null ? "Memeriksa dokumen ber-DOI tanpa rujukan..."
          : pending === 0 ? "Semua dokumen ber-DOI sudah dicek ke Crossref."
          : `${formatInt(pending)} dokumen punya DOI tetapi belum punya rujukan. Crossref menyimpan daftar rujukan untuk banyak penerbit.`}
      </span>
      {pending ? (
        <button type="button" className="btn btn-primary" disabled={running} onClick={() => void run()}>
          {running ? "Mengambil rujukan..." : "Lengkapi rujukan dari Crossref"}
        </button>
      ) : null}
      {running ? <span className="muted refs-help">Sekitar 15 detik per 600 dokumen. Bisa dijalankan lagi untuk sisanya.</span> : null}
      {report ? (
        <span className="refs-help" role="status">
          {`${formatInt(report.checked)} dokumen dicek, ${formatInt(report.gained)} kini punya daftar rujukan.`}
          {report.remaining ? ` ${formatInt(report.remaining)} lagi menunggu; jalankan sekali lagi.` : ""}
        </span>
      ) : null}
      {report?.error || error ? <span className="refs-error" role="alert">{report?.error ?? error}</span> : null}
    </div>
  );
}
