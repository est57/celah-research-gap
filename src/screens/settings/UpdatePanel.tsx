import { useState } from "react";
import { isDesktop } from "../../lib/api";
import { checkUpdate, installUpdate, type UpdateInfo } from "../../lib/apiApp";

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
      <div className="ph">Pembaruan aplikasi</div>
      <div className="about-body">
        <p className="muted">
          Celah mengambil versi baru dari halaman rilis resminya di GitHub. Setiap berkas diperiksa tanda tangannya sebelum
          dipasang, dan koleksi Anda tidak ikut berubah.
        </p>
        {!desktop ? <div className="notice">Pemeriksaan pembaruan hanya tersedia di aplikasi desktop.</div> : null}
        <div className="about-actions">
          {state.kind === "available" ? (
            <button type="button" className="btn btn-primary" onClick={() => void install()}>
              {`Pasang versi ${state.info.version} dan mulai ulang`}
            </button>
          ) : (
            <button type="button" className="btn" disabled={busy || !desktop} onClick={() => void check()}>
              {state.kind === "checking" ? "Memeriksa..." : state.kind === "installing" ? "Mengunduh dan memasang..." : "Periksa pembaruan"}
            </button>
          )}
          {state.kind === "current" ? <span className="muted" role="status">Anda sudah memakai versi terbaru.</span> : null}
          {state.kind === "available" ? (
            <span className="muted" role="status">{`Versi ${state.info.version} tersedia (sekarang ${state.info.current}).`}</span>
          ) : null}
          {state.kind === "error" ? <span className="refs-error" role="alert">{state.text}</span> : null}
        </div>
        {state.kind === "available" && state.info.notes ? <p className="muted update-notes">{state.info.notes}</p> : null}
      </div>
    </section>
  );
}
