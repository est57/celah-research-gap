import { formatInt } from "../../lib/format";
import { isDesktop } from "../../lib/api";
import { t } from "../../i18n";

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
        {desktop ? t("Tersimpan lokal") : t("Mode pratinjau browser")}
      </span>
      <span className="num">
        {analyzed === null ? t("{n} dokumen", { n: formatInt(total) }) : t("{n} dari {total} dokumen dianalisis", { n: formatInt(analyzed), total: formatInt(total) })}
      </span>
      <span className="wide-only">
        {sources.length ? t("Sumber: {list}", { list: sources.join(", ") }) : t("Belum ada sumber")}
      </span>
      <span className="wide-only">
        {keyConfigured ? t("OpenAlex: kunci API aktif") : t("OpenAlex: kuota bersama")}
      </span>
      <span className="num" style={{ marginLeft: "auto" }}>
        v0.1
      </span>
    </footer>
  );
}
