import { useState } from "react";
import { KIND_INFO, STATUS_LABEL, VERDICT_LABEL, type GapKind, type ReviewEntry } from "../../lib/gapTypes";

interface Props {
  entry: ReviewEntry;
  onOpen: () => void;
  onRemove: () => void;
}

const date = (s: string) => s.slice(0, 16);

export function HistoryRow({ entry, onOpen, onRemove }: Props) {
  const [confirming, setConfirming] = useState(false);
  const kind = KIND_INFO[entry.kind as GapKind];
  return (
    <li className="hist-row">
      <span className="status-pill" data-status={entry.status}>{STATUS_LABEL[entry.status]}</span>
      <div className="hist-main">
        <p className="hist-title">{entry.title}</p>
        <p className="hist-meta">
          {[kind?.label ?? entry.kind, `diperbarui ${date(entry.updatedAt)}`, entry.score !== null ? `skor ${Math.round(entry.score * 100)}` : null]
            .filter(Boolean).join(" · ")}
        </p>
        {entry.note ? <p className="hist-note">{entry.note}</p> : null}
        <div className="hist-tags">
          {entry.verdict ? (
            <span className="verdict-pill" data-verdict={entry.verdict}>
              {`${VERDICT_LABEL[entry.verdict]}${entry.checkedAt ? ` (${date(entry.checkedAt)})` : ""}`}
            </span>
          ) : <span className="muted">Belum diverifikasi</span>}
          {!entry.inAnalysis ? <span className="hist-gone">Tidak muncul di analisis saat ini</span> : null}
        </div>
      </div>
      <div className="hist-actions">
        {confirming ? (
          <>
            <button type="button" className="btn" onClick={() => setConfirming(false)}>Batal</button>
            <button type="button" className="btn btn-danger" onClick={onRemove}>Hapus</button>
          </>
        ) : (
          <>
            <button type="button" className="btn" disabled={!entry.inAnalysis} onClick={onOpen}
              title={entry.inAnalysis ? undefined : "Celah ini tidak muncul dengan filter dan koleksi saat ini"}>
              Buka di Explorer
            </button>
            <button type="button" className="btn" onClick={() => setConfirming(true)}>Hapus review</button>
          </>
        )}
      </div>
    </li>
  );
}
