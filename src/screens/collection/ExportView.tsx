import { useState } from "react";
import { isDesktop } from "../../lib/api";
import { backupDatabase, exportCollection, type CollectionFormat } from "../../lib/apiCollection";
import { formatInt } from "../../lib/format";
import type { CollectionStats } from "../../lib/types";
import { RestorePanel } from "./RestorePanel";
import { t, tk } from "../../i18n";

const FORMATS: [CollectionFormat, string, string, string][] = [
  ["csv", "CSV", ".csv", tk("Semua field per dokumen untuk Excel, R, atau lampiran data. Sel yang mirip rumus dinetralkan.")],
  ["bibtex", "BibTeX", ".bib", tk("Untuk LaTeX, Zotero, atau JabRef.")],
  ["ris", "RIS", ".ris", tk("Untuk Zotero, Mendeley, atau EndNote.")],
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
    return r ? t("{n} dokumen tersimpan di {file}.", { n: formatInt(r.count), file: r.file }) : t("Penyimpanan dibatalkan.");
  });
  const backup = () => run("backup", async () => {
    const file = await backupDatabase();
    return file ? t("Cadangan tersimpan di {file}.", { file }) : t("Penyimpanan dibatalkan.");
  });

  return (
    <div className="year-page">
      {!desktop ? <div className="notice">{t("Ekspor dan cadangan hanya berjalan di aplikasi desktop.")}</div> : null}
      <section className="pnl">
        <div className="ph">{t("Ekspor koleksi")}</div>
        <div className="exp-scope">
          <div className="opt-seg" role="group" aria-label={t("Dokumen yang diekspor")}>
            <button type="button" aria-pressed={scope === "all"} onClick={() => setScope("all")}>{t("Seluruh koleksi ({n})", { n: formatInt(stats.total) })}</button>
            <button type="button" aria-pressed={scope === "filtered"} disabled={!stats.filtered} onClick={() => setScope("filtered")}>
              {t("Hasil filter ({n})", { n: formatInt(stats.analyzed) })}
            </button>
          </div>
          {!stats.filtered ? <span className="muted">{t("Atur Filter analisis untuk mengekspor sebagian koleksi.")}</span> : null}
        </div>
        <ul className="exp-list">
          {FORMATS.map(([f, label, ext, hint]) => (
            <li key={f}>
              <div><strong>{label}</strong><span className="muted">{t(hint)}</span></div>
              <button type="button" className="btn" disabled={!desktop || busy !== null || count === 0} onClick={() => void save(f)}>
                {busy === f ? t("Menyimpan...") : t("Simpan {ext}", { ext })}
              </button>
            </li>
          ))}
        </ul>
      </section>
      <section className="pnl">
        <div className="ph">{t("Cadangan database")}</div>
        <div className="exp-backup">
          <p className="muted">
            {t("Salinan utuh satu file: koleksi, review celah, hasil verifikasi, dan filter. Simpan di luar laptop, misalnya di drive cadangan. Untuk memulihkan, pakai panel Pulihkan dari cadangan di bawah.")}
          </p>
          <button type="button" className="btn btn-primary" disabled={!desktop || busy !== null} onClick={() => void backup()}>
            {busy === "backup" ? t("Mencadangkan...") : t("Cadangkan sekarang")}
          </button>
        </div>
      </section>
      {status ? <div className="notice" role="status">{status}</div> : null}
      <RestorePanel current={stats.total} onRestored={onChanged} />
    </div>
  );
}
