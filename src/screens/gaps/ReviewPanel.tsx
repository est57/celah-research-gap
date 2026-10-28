import { useEffect, useState } from "react";
import { formatDecimal } from "../../lib/format";
import { t } from "../../i18n";
import type { Gap, ReviewStatus } from "../../lib/gapTypes";
import { statusLabel } from "./labels";

interface Props {
  gap: Gap;
  saveError: string | null;
  verifying: boolean;
  onReview: (status: ReviewStatus, note: string) => void;
  onVerify(): Promise<void>;
}

const STATUSES: Exclude<ReviewStatus, "">[] = ["accepted", "check", "rejected"];

export function ReviewPanel({ gap, saveError, verifying, onReview, onVerify }: Props) {
  const [note, setNote] = useState(gap.note);
  useEffect(() => setNote(gap.note), [gap.key, gap.note]);
  const dirty = note !== gap.note;
  const [verifyError, setVerifyError] = useState<string | null>(null);
  useEffect(() => setVerifyError(null), [gap.key]);
  const verifyNow = () => {
    setVerifyError(null);
    onVerify().catch((err: unknown) => setVerifyError(err instanceof Error ? err.message : String(err)));
  };

  return (
    <aside className="inspector gap-inspector" aria-label={t("Skor dan review")}>
      <div className="ph">{t("Skor celah")}<span className="num score-big">{formatDecimal(gap.score, 2)}</span></div>
      <div className="score-parts">
        {gap.parts.map((p) => (
          <div key={p.label} className="score-row">
            <span>{p.label}</span>
            <span className="score-track"><span style={{ width: `${p.value * 100}%` }} /></span>
            <span className="num">{formatDecimal(p.value, 2)}</span>
          </div>
        ))}
        <p className="muted score-note">{t("Kebaruan: seberapa jarang dibahas. Bukti: banyaknya data pendukung. Momentum: seberapa baru dokumennya dibanding koleksi.")}</p>
      </div>
      <div className="ph">{t("Review")}</div>
      <div className="review-box">
        <div className="segmented" role="group" aria-label={t("Status review")}>
          {STATUSES.map((s) => (
            <button key={s} type="button" data-status={s} aria-pressed={gap.status === s}
              onClick={() => onReview(gap.status === s ? "" : s, note)}>
              {statusLabel(s)}
            </button>
          ))}
        </div>
        <label htmlFor="gap-note" className="muted">{t("Catatan")}</label>
        <textarea id="gap-note" className="input note-input" rows={4} maxLength={2000} value={note}
          placeholder={t("mis. cek ketersediaan dataset lokal")} onChange={(e) => setNote(e.target.value)} />
        <button type="button" className="btn" disabled={!dirty} onClick={() => onReview(gap.status, note)}>{t("Simpan catatan")}</button>
        {saveError ? <p className="message" data-kind="error" role="alert">{saveError}</p> : null}
      </div>
      <div className="ph">{t("Verifikasi")}</div>
      <div className="review-box">
        <span className="muted">{t("Cari paper 2 tahun terakhir di luar koleksi yang mungkin sudah menjawab celah ini.")}</span>
        <button type="button" className="btn" disabled={verifying} onClick={verifyNow}>
          {verifying ? t("Memverifikasi...") : gap.check ? t("Cek ulang sekarang") : t("Verifikasi sekarang")}
        </button>
        {verifyError ? <p className="message" data-kind="error" role="alert">{verifyError}</p> : null}
      </div>
    </aside>
  );
}
