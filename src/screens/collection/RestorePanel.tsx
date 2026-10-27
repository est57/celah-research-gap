import { useState } from "react";
import { isDesktop } from "../../lib/api";
import { cancelRestore, chooseBackup, restoreBackup, type BackupInfo } from "../../lib/apiCollection";
import { formatInt } from "../../lib/format";

interface Props {
  current: number;
  onRestored: () => void;
}

/** Restore in two steps: pick and check a backup, then confirm replacing the current data. */
export function RestorePanel({ current, onRestored }: Props) {
  const [info, setInfo] = useState<BackupInfo | null>(null);
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const desktop = isDesktop();

  const step = async (task: () => Promise<void>) => {
    setBusy(true);
    setError(null);
    try {
      await task();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  };
  const choose = () => step(async () => {
    setMessage(null);
    setInfo(await chooseBackup());
  });
  const cancel = () => step(async () => {
    await cancelRestore();
    setInfo(null);
  });
  const restore = () => step(async () => {
    const r = await restoreBackup();
    setInfo(null);
    setMessage(`Koleksi dipulihkan: ${formatInt(r.documents)} dokumen. Data sebelumnya disimpan otomatis sebagai ${r.safetyFile} di folder cadangan-otomatis.`);
    onRestored();
  });

  return (
    <section className="pnl">
      <div className="ph">Pulihkan dari cadangan</div>
      <div className="exp-backup">
        {info ? (
          <p className="delete-warn">
            {`File cadangan berisi ${formatInt(info.documents)} dokumen dan ${formatInt(info.reviews)} review celah. Data saat ini (${formatInt(current)} dokumen) akan diganti seluruhnya. Sebelum itu, data saat ini disimpan otomatis ke folder cadangan-otomatis.`}
          </p>
        ) : (
          <p className="muted">Ganti koleksi, review, dan verifikasi dengan isi file cadangan. File diperiksa dulu, lalu Anda diminta mengonfirmasi.</p>
        )}
        {info ? (
          <div className="exp-actions">
            <button type="button" className="btn" disabled={busy} onClick={() => void cancel()}>Batal</button>
            <button type="button" className="btn btn-danger" disabled={busy} onClick={() => void restore()}>{busy ? "Memulihkan..." : "Ya, pulihkan"}</button>
          </div>
        ) : (
          <button type="button" className="btn" disabled={!desktop || busy} onClick={() => void choose()}>{busy ? "Memeriksa..." : "Pilih file cadangan"}</button>
        )}
      </div>
      {message ? <p className="exp-msg" role="status">{message}</p> : null}
      {error ? <p className="exp-msg delete-error" role="alert">{error}</p> : null}
    </section>
  );
}
