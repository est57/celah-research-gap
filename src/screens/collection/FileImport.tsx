import { useState } from "react";
import { importFile } from "../../lib/api";
import { formatInt } from "../../lib/format";
import { t } from "../../i18n";

type Msg = { kind: "ok" | "error" | "info"; text: string } | null;

export function FileImport({ onImported }: { onImported: () => void }) {
  const [busy, setBusy] = useState(false);
  const [msg, setMsg] = useState<Msg>(null);

  const run = async () => {
    setBusy(true);
    setMsg({ kind: "info", text: t("Memilih dan membaca file...") });
    try {
      const r = await importFile();
      if (!r) {
        setMsg(null);
        return;
      }
      const vars = { origin: r.origin, fetched: formatInt(r.fetched), inserted: formatInt(r.inserted),
        duplicates: formatInt(r.duplicates), completed: formatInt(r.completed) };
      setMsg({
        kind: "ok",
        text: r.completed
          ? t("{origin}: {fetched} rekaman dibaca, {inserted} baru, {duplicates} sudah ada ({completed} di antaranya dilengkapi, misalnya dengan daftar rujukan).", vars)
          : t("{origin}: {fetched} rekaman dibaca, {inserted} baru, {duplicates} sudah ada.", vars),
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
          {busy ? t("Mengimpor...") : t("Impor file ekspor")}
        </button>
        <span className="muted">{t("Scopus: .csv, .bib, atau .ris. Web of Science: .txt (Plain text, Full record and cited references). PubMed: .nbib atau .txt (PubMed format). Zotero, Mendeley, EndNote: .ris.")}</span>
      </div>
      {msg ? (
        <span className="message" data-kind={msg.kind} role="status">
          {msg.text}
        </span>
      ) : null}
    </div>
  );
}
