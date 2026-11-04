import { useState } from "react";
import { t } from "../../i18n";
import { REASON_LABEL, type Decision, type Reason, type ScreenRow } from "../../lib/screeningTypes";

interface Props {
  row: ScreenRow;
  reason: Reason;
  onDecide: (decision: Decision | null, reason?: Reason, note?: string) => void;
}

/** One document to screen: title, abstract and keywords, with Include and Exclude. */
export function ScreenCard({ row, reason, onDecide }: Props) {
  const [open, setOpen] = useState(false);
  const [note, setNote] = useState(row.note);
  const meta = [row.authors, row.year ?? t("t.t."), row.sourceName].filter(Boolean).join(" · ");
  const long = (row.abstractText?.length ?? 0) > 420;

  return (
    <li className="scr-card" data-decision={row.decision || "none"}>
      <div className="scr-head">
        <strong className="scr-title">{row.title}</strong>
        {row.decision ? (
          <span className="scr-badge" data-decision={row.decision}>
            {row.decision === "include" ? t("Masuk") : t(REASON_LABEL[row.reason || "other"])}
          </span>
        ) : null}
      </div>
      <span className="muted scr-meta">{meta}</span>
      {row.abstractText ? (
        <p className="scr-abstract" data-open={open || !long}>{row.abstractText}</p>
      ) : (
        <p className="muted scr-abstract">{t("Tanpa abstrak; putuskan dari judul atau buka dokumennya.")}</p>
      )}
      {long ? (
        <button type="button" className="opt-link scr-more" onClick={() => setOpen((v) => !v)}>
          {open ? t("Ringkas") : t("Selengkapnya")}
        </button>
      ) : null}
      {row.keywords ? <span className="muted scr-keys">{row.keywords}</span> : null}
      <div className="scr-actions">
        <button type="button" className="btn" aria-pressed={row.decision === "include"} onClick={() => onDecide("include")}>{t("Masuk")}</button>
        <button type="button" className="btn" aria-pressed={row.decision === "exclude"} onClick={() => onDecide("exclude", reason, note)}>
          {t("Keluar: {reason}", { reason: t(REASON_LABEL[reason]).toLowerCase() })}
        </button>
        {row.decision ? <button type="button" className="btn" onClick={() => onDecide(null)}>{t("Batalkan keputusan")}</button> : null}
        {row.decision === "exclude" ? (
          <input className="input scr-note" maxLength={200} value={note} placeholder={t("Catatan alasan (opsional)")}
            aria-label={t("Catatan alasan")} onChange={(e) => setNote(e.target.value)}
            onBlur={() => note !== row.note && onDecide("exclude", (row.reason || reason) as Reason, note)} />
        ) : null}
      </div>
    </li>
  );
}
