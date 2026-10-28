import { useState, type ReactNode } from "react";
import { ScreenMessage } from "../../components/ScreenMessage";
import { useGapHistory } from "../../hooks/useGapHistory";
import { t } from "../../i18n";
import type { ReviewEntry } from "../../lib/gapTypes";
import { HistoryRow } from "./HistoryRow";
import { statusLabel } from "./labels";

type StatusFilter = "all" | ReviewEntry["status"];

interface Props {
  version: number;
  onOpen: (key: string) => void;
  onRemoved: (key: string) => void;
}

const FILTERS: StatusFilter[] = ["all", "accepted", "check", "rejected"];

export function HistoryView({ version, onOpen, onRemoved }: Props) {
  const { items, sample, error, loading, data, remove, actionError } = useGapHistory(version);
  const [status, setStatus] = useState<StatusFilter>("all");
  const count = (s: StatusFilter) => (s === "all" ? items.length : items.filter((e) => e.status === s).length);
  const shown = status === "all" ? items : items.filter((e) => e.status === status);
  const gone = items.filter((e) => !e.inAnalysis).length;

  let list: ReactNode;
  if (error) return <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  if (!data) return <ScreenMessage>{loading ? t("Memuat riwayat review...") : ""}</ScreenMessage>;
  if (items.length === 0) {
    return (
      <ScreenMessage>
        <strong>{t("Belum ada review")}</strong>
        <span className="muted">{t("Tandai celah sebagai Diterima, Perlu cek, atau Ditolak di Gap Explorer; keputusannya tersimpan di sini.")}</span>
      </ScreenMessage>
    );
  }
  if (shown.length === 0) list = <p className="muted hist-empty">{t("Tidak ada review dengan status ini.")}</p>;
  else {
    list = (
      <ul className="hist-list">
        {shown.map((e) => (
          <HistoryRow key={e.key} entry={e} onOpen={() => onOpen(e.key)}
            onRemove={() => void remove(e.key).then(() => onRemoved(e.key))} />
        ))}
      </ul>
    );
  }

  return (
    <div className="hist-page">
      {sample ? <div className="notice">{t("Pratinjau browser: review di halaman ini adalah data contoh.")}</div> : null}
      <div className="hist-head">
        <div className="opt-seg hist-filter" role="group" aria-label={t("Saring menurut status")}>
          {FILTERS.map((s) => (
            <button key={s} type="button" aria-pressed={status === s} onClick={() => setStatus(s)}>
              {`${s === "all" ? t("Semua") : statusLabel(s)} (${count(s)})`}
            </button>
          ))}
        </div>
        {gone ? <span className="muted">{t("{n} celah tidak muncul di analisis saat ini karena filter atau koleksi berubah; keputusannya tetap tersimpan.", { n: gone })}</span> : null}
      </div>
      {actionError ? <div className="notice" role="alert">{actionError}</div> : null}
      <section className="pnl">{list}</section>
    </div>
  );
}
