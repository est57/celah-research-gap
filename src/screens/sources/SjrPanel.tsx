import { useState } from "react";
import { isDesktop } from "../../lib/api";
import { clearSjr, importSjr } from "../../lib/apiJournal";
import { formatInt } from "../../lib/format";
import type { SjrStatus } from "../../lib/quartileTypes";
import { t } from "../../i18n";

/** The full Scimago list has well over 25,000 journals; fewer means a filtered download. */
const PARTIAL_BELOW = 15_000;

/** Status of the imported Scimago list, with import and remove actions. */
export function SjrPanel({ status, onChanged }: { status: SjrStatus; onChanged: () => void }) {
  const [busy, setBusy] = useState(false);
  const [msg, setMsg] = useState<{ text: string; error: boolean } | null>(null);
  const desktop = isDesktop();
  // The first step keeps its link and button name as elements inside one translated sentence.
  const step1 = t("Buka {url} tanpa filter (semua bidang, semua wilayah, semua jenis akses), lalu klik {button} (gratis).").split(/(\{url\}|\{button\})/);

  const run = async (action: typeof importSjr, done: (s: SjrStatus) => string) => {
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
        {t("Data SJR (Scimago Journal Rank)")}
        <span className="ph-aside">
          {status.journals
            ? status.year
              ? t("{n} jurnal, peringkat {year}", { n: formatInt(status.journals), year: status.year })
              : t("{n} jurnal", { n: formatInt(status.journals) })
            : t("belum diimpor")}
        </span>
      </div>
      <div className="sjr-body">
        <ol className="sjr-steps">
          <li>
            {step1.map((part, i) =>
              part === "{url}" ? <span key={i} className="num select">{"scimagojr.com/journalrank.php"}</span>
                : part === "{button}" ? <strong key={i}>{"Download data"}</strong>
                : part)}
          </li>
          <li>{t("Impor file CSV itu di sini. Kuartil dicocokkan lewat ISSN, lalu lewat judul jurnal.")}</li>
        </ol>
        <div className="sjr-actions">
          <button type="button" className="btn btn-primary" disabled={busy || !desktop}
            onClick={() => void run(importSjr, (s) => (s.year
              ? t("{n} jurnal dimuat (peringkat {year}).", { n: formatInt(s.journals), year: s.year })
              : t("{n} jurnal dimuat.", { n: formatInt(s.journals) })))}>
            {busy ? t("Memproses...") : status.journals ? t("Ganti data SJR") : t("Impor data SJR")}
          </button>
          {status.journals ? (
            <button type="button" className="btn" disabled={busy || !desktop} onClick={() => void run(clearSjr, () => t("Data SJR dihapus."))}>{t("Hapus data SJR")}</button>
          ) : null}
          {msg ? <span className={msg.error ? "refs-error" : "muted"} role="status">{msg.text}</span> : null}
        </div>
        {status.journals > 0 && status.journals < PARTIAL_BELOW ? (
          <p className="notice sjr-note" role="note">
            {t("Data SJR ini hanya berisi {n} jurnal, sedangkan daftar lengkap Scimago berisi lebih dari 25.000. Kemungkinan file diunduh dengan filter aktif (misalnya bidang ilmu, wilayah, atau Open Access), sehingga jurnal seperti IEEE Access tidak ikut. Unduh ulang tanpa filter, lalu klik Ganti data SJR.", { n: formatInt(status.journals) })}
          </p>
        ) : null}
        {!desktop ? <p className="muted sjr-note">{t("Impor data SJR hanya berjalan di aplikasi desktop.")}</p> : null}
        <p className="muted sjr-note">
          {t("Kuartil berlaku untuk jurnal, per bidang ilmu dan per tahun. Celah memakai kuartil terbaik jurnal (SJR Best Quartile) pada tahun peringkat yang diimpor. Prosiding dan buku umumnya tidak punya kuartil.")}
        </p>
      </div>
    </section>
  );
}
