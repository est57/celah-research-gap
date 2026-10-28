import { useState } from "react";
import { t } from "../../i18n";
import { isDesktop } from "../../lib/api";
import { KIND_INFO, type Gap, type VerifyReport } from "../../lib/gapTypes";
import { CheckResult } from "./CheckResult";

interface Props {
  gaps: Gap[];
  verifying: boolean;
  onVerify(keys: string[]): Promise<VerifyReport[]>;
}

export function VerifyView({ gaps, verifying, onVerify }: Props) {
  const accepted = gaps.filter((g) => g.status === "accepted");
  const [message, setMessage] = useState<{ kind: "ok" | "error"; text: string } | null>(null);

  const run = async (keys: string[]) => {
    setMessage(null);
    try {
      const reports = await onVerify(keys);
      const failed = reports.filter((r) => r.error);
      setMessage(
        failed.length
          ? { kind: "error", text: t("{ok} celah dicek, {failed} gagal: {error}", { ok: reports.length - failed.length, failed: failed.length, error: failed[0].error ?? "" }) }
          : { kind: "ok", text: t("{n} celah selesai dicek terhadap literatur terbaru.", { n: reports.length }) },
      );
    } catch (err) {
      setMessage({ kind: "error", text: err instanceof Error ? err.message : String(err) });
    }
  };

  return (
    <div className="verify-page">
      <div className="verify-intro">
        <div>
          <strong>{t("Verifikasi celah yang diterima")}</strong>
          <p className="muted">
            {t("Untuk setiap celah, Celah mencari paper OpenAlex 2 tahun terakhir yang belum ada di koleksi. Paper yang judul atau abstraknya memuat semua kata kunci celah ditandai \"cocok\". Tiga atau lebih paper cocok berarti celah itu mungkin sudah mulai dijawab, dan skornya diturunkan.")}
          </p>
        </div>
        <button type="button" className="btn btn-primary" disabled={verifying || accepted.length === 0 || !isDesktop()}
          onClick={() => void run([])}>
          {verifying ? t("Memverifikasi...") : t("Verifikasi semua ({n})", { n: accepted.length })}
        </button>
      </div>
      {message ? <p className="message verify-msg" data-kind={message.kind} role="status">{message.text}</p> : null}
      {!isDesktop() ? <div className="notice">{t("Pratinjau browser: verifikasi hanya berjalan di aplikasi desktop.")}</div> : null}
      {accepted.length === 0 ? (
        <p className="muted verify-empty">{t("Belum ada celah yang diterima. Tandai celah sebagai \"Diterima\" di Gap Explorer untuk memverifikasinya.")}</p>
      ) : (
        accepted.map((g) => (
          <section key={g.key} className="pnl verify-item">
            <div className="ph">
              <span className="kind-badge" style={{ background: KIND_INFO[g.kind].color }}>{KIND_INFO[g.kind].letter}</span>
              <span className="verify-title">{g.title}</span>
              <button type="button" className="btn" disabled={verifying || !isDesktop()} onClick={() => void run([g.key])}>
                {g.check ? t("Cek ulang") : t("Verifikasi")}
              </button>
            </div>
            <div className="pnl-body">
              <div className="term-row">
                {g.terms.map((term) => <span key={term} className="term-chip">{term}</span>)}
              </div>
              {g.check ? <CheckResult check={g.check} limit={5} /> : <p className="muted">{t("Belum pernah diverifikasi.")}</p>}
            </div>
          </section>
        ))
      )}
    </div>
  );
}
