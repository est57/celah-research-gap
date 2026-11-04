import { useState } from "react";
import { SCREEN_PAGE, useScreening } from "../../hooks/useScreening";
import { t } from "../../i18n";
import { formatInt, formatPct } from "../../lib/format";
import { REASON_LABEL, type Reason, type ScreenStatus } from "../../lib/screeningTypes";
import { NumText } from "./NumText";
import { ScreenCard } from "./ScreenCard";
import { useDocLinks } from "../../hooks/useDocLinks";

const STATUSES: ScreenStatus[] = ["none", "include", "exclude", "all"];

/** Title and abstract screening: Include or Exclude each document, with a reason for exclusions. */
export function ScreeningView({ onChanged }: { onChanged: () => void }) {
  const [status, setStatus] = useState<ScreenStatus>("none");
  const [query, setQuery] = useState("");
  const [page, setPage] = useState(0);
  const [reason, setReason] = useState<Reason>("topic");
  const { data, error, loading, decide, sample } = useScreening(status, query, page, onChanged);
  const links = useDocLinks(data.items.map((r) => r.id));
  const c = data.counts;
  const total = c.unscreened + c.included + c.excluded;
  const count: Record<ScreenStatus, number> = { none: c.unscreened, include: c.included, exclude: c.excluded, all: total };
  const label: Record<ScreenStatus, string> = { none: t("Belum disaring"), include: t("Masuk"), exclude: t("Keluar"), all: t("Semua") };
  const pages = Math.max(1, Math.ceil(data.total / SCREEN_PAGE));
  const pick = (s: ScreenStatus) => (setStatus(s), setPage(0));

  return (
    <div className="dup-page scr-page">
      {sample ? <div className="notice">{t("Pratinjau browser: dokumen di halaman ini adalah data contoh, dan keputusan tidak disimpan.")}</div> : null}
      <section className="pnl">
        <div className="ph">
          {t("Skrining judul dan abstrak")}
          <span className="ph-aside">{t("{done} dari {total} disaring ({pct})", { done: formatInt(total - c.unscreened), total: formatInt(total), pct: formatPct(total - c.unscreened, total) })}</span>
        </div>
        <div className="scr-intro">
          <p className="muted">
            {t("Tandai setiap dokumen Masuk atau Keluar. Dokumen yang Keluar tidak ikut dianalisis, dan jumlahnya beserta alasannya masuk ke diagram PRISMA di Laporan & ekspor. Dokumen yang belum disaring tetap dianalisis.")}
          </p>
          <span className="scr-progress" aria-hidden="true"><span style={{ width: formatPct(total - c.unscreened, total).replace(",", ".") }} /></span>
        </div>
      </section>
      <div className="scr-toolbar">
        <div className="opt-seg scr-status" role="group" aria-label={t("Status skrining")}>
          {STATUSES.map((s) => (
            <button key={s} type="button" aria-pressed={status === s} onClick={() => pick(s)}>
              {label[s]} <span className="num">{formatInt(count[s])}</span>
            </button>
          ))}
        </div>
        <input className="input scr-search" type="search" maxLength={200} value={query} placeholder={t("Cari judul, abstrak, kata kunci")}
          aria-label={t("Cari dokumen")} onChange={(e) => (setQuery(e.target.value), setPage(0))} />
        <label className="scr-reason">
          <span className="muted">{t("Alasan keluar")}</span>
          <select className="input" value={reason} onChange={(e) => setReason(e.target.value as Reason)}>
            {(Object.keys(REASON_LABEL) as Reason[]).map((r) => <option key={r} value={r}>{t(REASON_LABEL[r])}</option>)}
          </select>
        </label>
      </div>
      {error ? <div className="notice" role="alert">{error}</div> : null}
      {!loading && data.items.length === 0 ? (
        <p className="muted scr-empty">{status === "none" && total > 0 ? t("Semua dokumen sudah disaring.") : t("Tidak ada dokumen di sini.")}</p>
      ) : null}
      <ul className="dup-list" aria-busy={loading}>
        {data.items.map((row) => (
          <ScreenCard key={`${row.id}-${row.decision}`} row={row} reason={reason} links={links.get(row.id)}
            onDecide={(d, r, note) => void decide([row.id], d, d === "exclude" ? (r ?? reason) : "", note ?? "")} />
        ))}
      </ul>
      {data.items.length > 0 ? (
        <div className="scr-pager">
          {status === "none" ? (
            <button type="button" className="btn" onClick={() => void decide(data.items.map((r) => r.id), "include")}>
              {t("Masukkan semua di halaman ini ({n})", { n: formatInt(data.items.length) })}
            </button>
          ) : <span />}
          <span className="muted"><NumText text={t("Halaman {page} dari {pages}")} vars={{ page: formatInt(page + 1), pages: formatInt(pages) }} /></span>
          <button type="button" className="btn" disabled={page === 0} onClick={() => setPage((p) => p - 1)}>{t("Sebelumnya")}</button>
          <button type="button" className="btn" disabled={page + 1 >= pages} onClick={() => setPage((p) => p + 1)}>{t("Berikutnya")}</button>
        </div>
      ) : null}
    </div>
  );
}
