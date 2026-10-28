import { useState } from "react";
import { t } from "../../i18n";
import { KIND_INFO, type GapKind, type ReviewEntry } from "../../lib/gapTypes";
import { kindLabel, statusLabel, verdictLabel } from "./labels";

interface Props {
  entry: ReviewEntry;
  onOpen: () => void;
  onRemove: () => void;
}

const date = (s: string) => s.slice(0, 16);

export function HistoryRow({ entry, onOpen, onRemove }: Props) {
  const [confirming, setConfirming] = useState(false);
  const kind = entry.kind in KIND_INFO ? kindLabel(entry.kind as GapKind) : entry.kind;
  return (
    <li className="hist-row">
      <span className="status-pill" data-status={entry.status}>{statusLabel(entry.status)}</span>
      <div className="hist-main">
        <p className="hist-title">{entry.title}</p>
        <p className="hist-meta">
          {[kind, t("diperbarui {date}", { date: date(entry.updatedAt) }), entry.score !== null ? t("skor {n}", { n: Math.round(entry.score * 100) }) : null]
            .filter(Boolean).join(" · ")}
        </p>
        {entry.note ? <p className="hist-note">{entry.note}</p> : null}
        <div className="hist-tags">
          {entry.verdict ? (
            <span className="verdict-pill" data-verdict={entry.verdict}>
              {`${verdictLabel(entry.verdict)}${entry.checkedAt ? ` (${date(entry.checkedAt)})` : ""}`}
            </span>
          ) : <span className="muted">{t("Belum diverifikasi")}</span>}
          {!entry.inAnalysis ? <span className="hist-gone">{t("Tidak muncul di analisis saat ini")}</span> : null}
        </div>
      </div>
      <div className="hist-actions">
        {confirming ? (
          <>
            <button type="button" className="btn" onClick={() => setConfirming(false)}>{t("Batal")}</button>
            <button type="button" className="btn btn-danger" onClick={onRemove}>{t("Hapus")}</button>
          </>
        ) : (
          <>
            <button type="button" className="btn" disabled={!entry.inAnalysis} onClick={onOpen}
              title={entry.inAnalysis ? undefined : t("Celah ini tidak muncul dengan filter dan koleksi saat ini")}>
              {t("Buka di Explorer")}
            </button>
            <button type="button" className="btn" onClick={() => setConfirming(true)}>{t("Hapus review")}</button>
          </>
        )}
      </div>
    </li>
  );
}
