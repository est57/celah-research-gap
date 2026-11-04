import { useState } from "react";
import { t, getLang } from "../../i18n";
import { isDesktop } from "../../lib/api";
import { watchHits, type Watch, type WatchHit, type WatchRun } from "../../lib/apiWatch";
import { formatInt } from "../../lib/format";
import { SOURCES } from "./SourcePicker";

const STALE_DAYS = 30;
const sampleHits = import.meta.env.DEV ? () => import("../../dev/sampleWatches").then((m) => m.SAMPLE_HITS) : null;

/** Stored UTC time ("2026-10-30 07:45:00") as a local date. */
const date = (utc: string) => new Date(`${utc.replace(" ", "T")}Z`).toLocaleDateString(getLang() === "en" ? "en-GB" : "id-ID", { day: "numeric", month: "short", year: "numeric" });
const age = (utc: string) => (Date.now() - new Date(`${utc.replace(" ", "T")}Z`).getTime()) / 86_400_000;

interface Props {
  watch: Watch;
  running: boolean;
  onRun: () => Promise<WatchRun | null>;
  onDelete: () => void;
}

/** One saved search: what it looks for, when it last ran, and what it added. */
export function WatchCard({ watch: w, running, onRun, onDelete }: Props) {
  const [hits, setHits] = useState<WatchHit[] | null>(null);
  const [result, setResult] = useState<string | null>(null);
  const [confirm, setConfirm] = useState(false);
  const sources = w.sources.map((s) => SOURCES.find((x) => x.id === s)?.label ?? s).join(", ");
  const stale = !w.lastRunAt || age(w.lastRunAt) > STALE_DAYS;

  const toggleHits = async () => {
    if (hits) return setHits(null);
    const load = isDesktop() ? watchHits(w.id) : (sampleHits?.() ?? Promise.resolve([]));
    setHits(await load.catch(() => []));
  };
  const run = async () => {
    const r = await onRun();
    if (!r) return;
    setHits(null);
    setResult(r.errors.length ? r.errors.join(" · ") : t("{added} dokumen baru dari {fetched} hasil.", { added: formatInt(r.added), fetched: formatInt(r.fetched) }));
  };

  return (
    <li className="watch-card">
      <div className="watch-head">
        <strong className="watch-query">{w.query}</strong>
        <span className="watch-state" data-stale={stale}>
          {w.lastRunAt ? t("Dicek {date}", { date: date(w.lastRunAt) }) : t("Belum pernah dijalankan")}
        </span>
      </div>
      <span className="muted watch-meta">
        {t("{sources} · sejak {year} · {n} per sumber", { sources, year: w.yearFrom, n: formatInt(w.maxResults) })}
      </span>
      <span className="watch-counts">
        {t("Terakhir: {last} baru · Total: {total} dokumen ditambahkan", { last: formatInt(w.lastNew), total: formatInt(w.totalNew) })}
      </span>
      <div className="scr-actions">
        <button type="button" className="btn btn-primary" disabled={running} onClick={() => void run()}>
          {running ? t("Mencari...") : t("Jalankan sekarang")}
        </button>
        <button type="button" className="btn" disabled={w.totalNew === 0} onClick={() => void toggleHits()}>
          {hits ? t("Sembunyikan dokumen") : t("Dokumen yang ditambahkan ({n})", { n: formatInt(w.totalNew) })}
        </button>
        {confirm ? (
          <>
            <span className="muted">{t("Hapus pantauan ini? Dokumennya tetap di koleksi.")}</span>
            <button type="button" className="btn" onClick={() => setConfirm(false)}>{t("Batal")}</button>
            <button type="button" className="btn btn-danger" onClick={onDelete}>{t("Hapus")}</button>
          </>
        ) : <button type="button" className="btn" onClick={() => setConfirm(true)}>{t("Hapus pantauan")}</button>}
      </div>
      {result ? <span className="message" role="status">{result}</span> : null}
      {hits ? (
        <ol className="watch-hits">
          {hits.map((h) => (
            <li key={h.id}>
              <span className="watch-hit-title">{h.title}</span>
              <span className="muted">{[h.authors, h.year, h.sourceName, h.origin].filter(Boolean).join(" · ")}</span>
            </li>
          ))}
        </ol>
      ) : null}
    </li>
  );
}
