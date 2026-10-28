import { useState } from "react";
import { isDesktop } from "../../lib/api";
import { checkUpdate, installUpdate, type UpdateInfo } from "../../lib/apiApp";
import { t } from "../../i18n";

type State =
  | { kind: "idle" | "checking" | "current" | "installing" }
  | { kind: "available"; info: UpdateInfo }
  | { kind: "error"; text: string };

const message = (e: unknown) => (e instanceof Error ? e.message : String(e));

/** Checks the signed release feed and installs a newer version on request. */
export function UpdatePanel() {
  const [state, setState] = useState<State>({ kind: "idle" });
  const desktop = isDesktop();
  const busy = state.kind === "checking" || state.kind === "installing";

  const check = async () => {
    setState({ kind: "checking" });
    try {
      const info = await checkUpdate();
      setState(info ? { kind: "available", info } : { kind: "current" });
    } catch (e) {
      setState({ kind: "error", text: message(e) });
    }
  };
  const install = async () => {
    setState({ kind: "installing" });
    try {
      await installUpdate();
    } catch (e) {
      setState({ kind: "error", text: message(e) });
    }
  };

  return (
    <section className="pnl key-panel">
      <div className="ph">{t("Pembaruan aplikasi")}</div>
      <div className="about-body">
        <p className="muted">
          {t("Celah mengambil versi baru dari halaman rilis resminya di GitHub. Setiap berkas diperiksa tanda tangannya sebelum dipasang, dan koleksi Anda tidak ikut berubah.")}
        </p>
        {!desktop ? <div className="notice">{t("Pemeriksaan pembaruan hanya tersedia di aplikasi desktop.")}</div> : null}
        <div className="about-actions">
          {state.kind === "available" ? (
            <button type="button" className="btn btn-primary" onClick={() => void install()}>
              {t("Pasang versi {version} dan mulai ulang", { version: state.info.version })}
            </button>
          ) : (
            <button type="button" className="btn" disabled={busy || !desktop} onClick={() => void check()}>
              {state.kind === "checking" ? t("Memeriksa...") : state.kind === "installing" ? t("Mengunduh dan memasang...") : t("Periksa pembaruan")}
            </button>
          )}
          {state.kind === "current" ? <span className="muted" role="status">{t("Anda sudah memakai versi terbaru.")}</span> : null}
          {state.kind === "available" ? (
            <span className="muted" role="status">{t("Versi {version} tersedia (sekarang {current}).", { version: state.info.version, current: state.info.current })}</span>
          ) : null}
          {state.kind === "error" ? <span className="refs-error" role="alert">{state.text}</span> : null}
        </div>
        {state.kind === "available" && state.info.notes ? <p className="muted update-notes">{state.info.notes}</p> : null}
      </div>
    </section>
  );
}
