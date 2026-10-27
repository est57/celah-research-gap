import { useState } from "react";
import { isDesktop } from "../../lib/api";
import { deletePapers } from "../../lib/apiCollection";
import { formatInt } from "../../lib/format";

interface Props {
  selected: Set<number>;
  onClear: () => void;
  onDeleted: (removed: number) => void;
}

/** Shown while documents are ticked: clear the selection or remove them after confirming. */
export function DeleteBar({ selected, onClear, onDeleted }: Props) {
  const [confirming, setConfirming] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const n = selected.size;
  if (n === 0) return null;

  const remove = async () => {
    setBusy(true);
    setError(null);
    try {
      const removed = isDesktop() ? await deletePapers([...selected]) : n;
      setConfirming(false);
      onDeleted(removed);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="delete-bar" role="region" aria-label="Dokumen terpilih">
      {confirming ? (
        <span className="delete-warn">{`Hapus ${formatInt(n)} dokumen dari koleksi? Rujukan, penulis, dan institusinya ikut terhapus. Tindakan ini tidak bisa dibatalkan.`}</span>
      ) : (
        <span>{`${formatInt(n)} dokumen dipilih`}</span>
      )}
      {error ? <span className="delete-error" role="alert">{error}</span> : null}
      {confirming ? (
        <>
          <button type="button" className="btn" disabled={busy} onClick={() => setConfirming(false)}>Batal</button>
          <button type="button" className="btn btn-danger" disabled={busy} onClick={() => void remove()}>{busy ? "Menghapus..." : "Ya, hapus"}</button>
        </>
      ) : (
        <>
          <button type="button" className="btn" onClick={onClear}>Batalkan pilihan</button>
          <button type="button" className="btn btn-danger" onClick={() => setConfirming(true)}>{`Hapus ${formatInt(n)} dokumen`}</button>
        </>
      )}
    </div>
  );
}
