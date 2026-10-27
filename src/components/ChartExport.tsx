import { useState, type MouseEvent } from "react";
import { isDesktop } from "../lib/api";
import { saveImage } from "../lib/apiEntities";
import { serializeSvg, svgToPng } from "../lib/chartExport";

/** SVG and PNG buttons for a panel header; they export the first SVG in the same panel. */
export function ChartExport({ name }: { name: string }) {
  const [busy, setBusy] = useState(false);
  const [note, setNote] = useState<string | null>(null);
  // Shown in the dev preview too, so the export path can be checked there; saving needs the desktop app.
  if (!isDesktop() && !import.meta.env.DEV) return null;

  const save = async (kind: "svg" | "png", e: MouseEvent<HTMLButtonElement>) => {
    const svg = e.currentTarget.closest(".pnl")?.querySelector("svg");
    if (!svg) return;
    setBusy(true);
    setNote(null);
    try {
      const image = serializeSvg(svg);
      const data = kind === "svg" ? new TextEncoder().encode(image.text) : await svgToPng(image);
      const file = await saveImage(kind, data, name);
      setNote(file ? `Tersimpan: ${file}` : null);
    } catch (err) {
      setNote(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <span className="chart-export">
      {note ? <span className="muted chart-export-note" role="status">{note}</span> : null}
      <button type="button" className="btn chart-export-btn" disabled={busy} onClick={(e) => void save("svg", e)} title="Simpan sebagai SVG (vektor)">SVG</button>
      <button type="button" className="btn chart-export-btn" disabled={busy} onClick={(e) => void save("png", e)} title="Simpan sebagai PNG resolusi 2x">PNG</button>
    </span>
  );
}
