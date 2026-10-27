import { useState } from "react";
import { isDesktop } from "../../lib/api";
import { backupDatabase, exportCollection, type CollectionFormat } from "../../lib/apiCollection";
import { formatInt } from "../../lib/format";
import type { CollectionStats } from "../../lib/types";
import { RestorePanel } from "./RestorePanel";

const FORMATS: [CollectionFormat, string, string, string][] = [
  ["csv", "CSV", ".csv", "Semua field per dokumen untuk Excel, R, atau lampiran data. Sel yang mirip rumus dinetralkan."],
  ["bibtex", "BibTeX", ".bib", "Untuk LaTeX, Zotero, atau JabRef."],
  ["ris", "RIS", ".ris", "Untuk Zotero, Mendeley, atau EndNote."],
];

export function ExportView({ stats, onChanged }: { stats: CollectionStats; onChanged: () => void }) {
  const [scope, setScope] = useState<"all" | "filtered">("all");
  const [busy, setBusy] = useState<string | null>(null);
  const [status, setStatus] = useState<string | null>(null);
  const desktop = isDesktop();
  const count = scope === "filtered" ? stats.analyzed : stats.total;

  const run = async (key: string, task: () => Promise<string>) => {
    setBusy(key);
    setStatus(null);
    try {
      setStatus(await task());
    } catch (err) {
      setStatus(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(null);
    }
  };
  const save = (f: CollectionFormat) => run(f, async () => {
    const r = await exportCollection(f, scope);
    return r ? `${formatInt(r.count)} dokumen tersimpan di ${r.file}.` : "Penyimpanan dibatalkan.";
  });
  const backup = () => run("backup", async () => {
    const file = await backupDatabase();
    return file ? `Cadangan tersimpan di ${file}.` : "Penyimpanan dibatalkan.";
  });

  return (
    <div className="year-page">
      {!desktop ? <div className="notice">Ekspor dan cadangan hanya berjalan di aplikasi desktop.</div> : null}
      <section className="pnl">
        <div className="ph">Ekspor koleksi</div>
        <div className="exp-scope">
          <div className="opt-seg" role="group" aria-label="Dokumen yang diekspor">
            <button type="button" aria-pressed={scope === "all"} onClick={() => setScope("all")}>{`Seluruh koleksi (${formatInt(stats.total)})`}</button>
            <button type="button" aria-pressed={scope === "filtered"} disabled={!stats.filtered} onClick={() => setScope("filtered")}>
              {`Hasil filter (${formatInt(stats.analyzed)})`}
            </button>
          </div>
          {!stats.filtered ? <span className="muted">Atur Filter analisis untuk mengekspor sebagian koleksi.</span> : null}
        </div>
        <ul className="exp-list">
          {FORMATS.map(([f, label, ext, hint]) => (
            <li key={f}>
              <div><strong>{label}</strong><span className="muted">{hint}</span></div>
              <button type="button" className="btn" disabled={!desktop || busy !== null || count === 0} onClick={() => void save(f)}>
                {busy === f ? "Menyimpan..." : `Simpan ${ext}`}
              </button>
            </li>
          ))}
        </ul>
      </section>
      <section className="pnl">
        <div className="ph">Cadangan database</div>
        <div className="exp-backup">
          <p className="muted">
            Salinan utuh satu file: koleksi, review celah, hasil verifikasi, dan filter. Simpan di luar laptop, misalnya
            di drive cadangan. Untuk memulihkan, pakai panel Pulihkan dari cadangan di bawah.
          </p>
          <button type="button" className="btn btn-primary" disabled={!desktop || busy !== null} onClick={() => void backup()}>
            {busy === "backup" ? "Mencadangkan..." : "Cadangkan sekarang"}
          </button>
        </div>
      </section>
      {status ? <div className="notice" role="status">{status}</div> : null}
      <RestorePanel current={stats.total} onRestored={onChanged} />
    </div>
  );
}
