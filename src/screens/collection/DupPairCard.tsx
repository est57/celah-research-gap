import { useState } from "react";
import type { DupPair } from "../../lib/duplicateTypes";
import { DupRecord } from "./DupRecord";
import { t } from "../../i18n";

interface Props {
  pair: DupPair;
  busy: boolean;
  onMerge: (keep: number) => void;
  onDismiss: () => void;
}

export function DupPairCard({ pair, busy, onMerge, onDismiss }: Props) {
  const [keep, setKeep] = useState(pair.keep);
  const [confirming, setConfirming] = useState(false);
  const name = `keep-${pair.a.id}-${pair.b.id}`;
  const reason = pair.reason === "same_title"
    ? t("Judul sama, tahun berbeda (kemungkinan preprint dan versi terbit)")
    : t("Judul {pct}% mirip", { pct: Math.round(pair.similarity * 100) });
  const choose = (id: number) => {
    setKeep(id);
    setConfirming(false);
  };

  return (
    <li className="pnl dup-card">
      <div className="ph">
        <span className="dup-reason" data-reason={pair.reason}>{reason}</span>
        {keep === pair.keep ? <span className="ph-aside">{t("disarankan: pertahankan yang lebih lengkap")}</span> : null}
      </div>
      <div className="dup-cols">
        <DupRecord paper={pair.a} name={name} kept={keep === pair.a.id} disabled={busy} onKeep={() => choose(pair.a.id)} />
        <DupRecord paper={pair.b} name={name} kept={keep === pair.b.id} disabled={busy} onKeep={() => choose(pair.b.id)} />
      </div>
      <div className="dup-actions">
        {confirming ? (
          <>
            <span className="dup-confirm">{t("Dokumen yang tidak dipilih akan dihapus dari koleksi. Field kosong diisi dari dokumen itu.")}</span>
            <button type="button" className="btn" disabled={busy} onClick={() => setConfirming(false)}>{t("Batal")}</button>
            <button type="button" className="btn btn-primary" disabled={busy} onClick={() => onMerge(keep)}>
              {busy ? t("Menggabungkan...") : t("Ya, gabungkan")}
            </button>
          </>
        ) : (
          <>
            <button type="button" className="btn" disabled={busy} onClick={onDismiss}>{t("Bukan duplikat")}</button>
            <button type="button" className="btn btn-primary" disabled={busy} onClick={() => setConfirming(true)}>{t("Gabungkan")}</button>
          </>
        )}
      </div>
    </li>
  );
}
