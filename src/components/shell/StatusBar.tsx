import { formatInt } from "../../lib/format";
import { isDesktop } from "../../lib/api";

interface Props {
  total: number;
  /** Documents used by the analyses when a filter is active. */
  analyzed: number | null;
  sources: string[];
  keyConfigured: boolean;
}

export function StatusBar({ total, analyzed, sources, keyConfigured }: Props) {
  const desktop = isDesktop();
  return (
    <footer className="statusbar">
      <span style={{ display: "inline-flex", alignItems: "center", gap: 6 }}>
        <span
          aria-hidden="true"
          style={{
            width: 7,
            height: 7,
            borderRadius: "50%",
            background: desktop ? "var(--ok)" : "var(--warn)",
          }}
        />
        {desktop ? "Tersimpan lokal" : "Mode pratinjau browser"}
      </span>
      <span className="num">
        {analyzed === null ? `${formatInt(total)} dokumen` : `${formatInt(analyzed)} dari ${formatInt(total)} dokumen dianalisis`}
      </span>
      <span className="wide-only">
        {sources.length ? `Sumber: ${sources.join(", ")}` : "Belum ada sumber"}
      </span>
      <span className="wide-only">
        {keyConfigured ? "OpenAlex: kunci API aktif" : "OpenAlex: kuota bersama"}
      </span>
      <span className="num" style={{ marginLeft: "auto" }}>
        v0.1
      </span>
    </footer>
  );
}
