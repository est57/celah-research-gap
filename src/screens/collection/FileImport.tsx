import { useState } from "react";
import { importFile } from "../../lib/api";
import { formatInt } from "../../lib/format";

type Msg = { kind: "ok" | "error" | "info"; text: string } | null;

export function FileImport({ onImported }: { onImported: () => void }) {
  const [busy, setBusy] = useState(false);
  const [msg, setMsg] = useState<Msg>(null);

  const run = async () => {
    setBusy(true);
    setMsg({ kind: "info", text: "Memilih dan membaca file..." });
    try {
      const r = await importFile();
      if (!r) {
        setMsg(null);
        return;
      }
      setMsg({
        kind: "ok",
        text: `${r.origin}: ${formatInt(r.fetched)} rekaman dibaca, ${formatInt(r.inserted)} baru, ${formatInt(r.duplicates)} sudah ada`
          + (r.completed ? ` (${formatInt(r.completed)} di antaranya dilengkapi, misalnya dengan daftar rujukan).` : "."),
      });
      onImported();
    } catch (err) {
      setMsg({ kind: "error", text: err instanceof Error ? err.message : String(err) });
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="file-import">
      <div className="file-import-row">
        <button type="button" className="btn" disabled={busy} onClick={() => void run()}>
          {busy ? "Mengimpor..." : "Impor file ekspor"}
        </button>
        <span className="muted">Scopus: .csv, .bib, atau .ris. Web of Science: .txt (Plain text, Full record and cited references). PubMed: .nbib atau .txt (PubMed format). Zotero, Mendeley, EndNote: .ris.</span>
      </div>
      {msg ? (
        <span className="message" data-kind={msg.kind} role="status">
          {msg.text}
        </span>
      ) : null}
    </div>
  );
}
