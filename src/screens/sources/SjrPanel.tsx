import { useState } from "react";
import { isDesktop } from "../../lib/api";
import { clearSjr, importSjr } from "../../lib/apiJournal";
import { formatInt } from "../../lib/format";
import type { SjrStatus } from "../../lib/quartileTypes";

/** Status of the imported Scimago list, with import and remove actions. */
export function SjrPanel({ status, onChanged }: { status: SjrStatus; onChanged: () => void }) {
  const [busy, setBusy] = useState(false);
  const [msg, setMsg] = useState<{ text: string; error: boolean } | null>(null);
  const desktop = isDesktop();

  const run = async (action: () => Promise<SjrStatus | null>, done: (s: SjrStatus) => string) => {
    setBusy(true);
    setMsg(null);
    try {
      const s = await action();
      if (s) {
        setMsg({ text: done(s), error: false });
        onChanged();
      }
    } catch (e) {
      setMsg({ text: e instanceof Error ? e.message : String(e), error: true });
    } finally {
      setBusy(false);
    }
  };

  return (
    <section className="pnl">
      <div className="ph">
        Data SJR (Scimago Journal Rank)
        <span className="ph-aside">
          {status.journals ? `${formatInt(status.journals)} jurnal${status.year ? `, peringkat ${status.year}` : ""}` : "belum diimpor"}
        </span>
      </div>
      <div className="sjr-body">
        <ol className="sjr-steps">
          <li>Buka <span className="num select">scimagojr.com/journalrank.php</span>, lalu klik <strong>Download data</strong> (gratis).</li>
          <li>Impor file CSV itu di sini. Kuartil dicocokkan lewat ISSN, lalu lewat judul jurnal.</li>
        </ol>
        <div className="sjr-actions">
          <button type="button" className="btn btn-primary" disabled={busy || !desktop}
            onClick={() => void run(importSjr, (s) => `${formatInt(s.journals)} jurnal dimuat${s.year ? ` (peringkat ${s.year})` : ""}.`)}>
            {busy ? "Memproses..." : status.journals ? "Ganti data SJR" : "Impor data SJR"}
          </button>
          {status.journals ? (
            <button type="button" className="btn" disabled={busy || !desktop} onClick={() => void run(clearSjr, () => "Data SJR dihapus.")}>Hapus data SJR</button>
          ) : null}
          {msg ? <span className={msg.error ? "refs-error" : "muted"} role="status">{msg.text}</span> : null}
        </div>
        {!desktop ? <p className="muted sjr-note">Impor data SJR hanya berjalan di aplikasi desktop.</p> : null}
        <p className="muted sjr-note">
          Kuartil berlaku untuk jurnal, per bidang ilmu dan per tahun. Celah memakai kuartil terbaik jurnal (SJR Best Quartile)
          pada tahun peringkat yang diimpor. Prosiding dan buku umumnya tidak punya kuartil.
        </p>
      </div>
    </section>
  );
}
