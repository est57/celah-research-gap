import type { CSSProperties, ReactNode } from "react";
import { formatDecimal, formatInt } from "../../lib/format";
import type { JournalPick } from "../../lib/journalTypes";
import { t, tk } from "../../i18n";

/** Gap titles shown under a journal name before the rest is summarised. */
const SHOWN_GAPS = 2;

interface Col {
  label: string;
  title: string;
  cell: (j: JournalPick) => ReactNode;
  /** Hidden when the panel is narrow. */
  wide?: boolean;
}

const COLUMNS: Col[] = [
  { label: tk("Kuartil"), title: tk("Kuartil SJR jurnal"), cell: (j) => <span className="q-chip" data-q={j.quartile || "none"}>{j.quartile || "-"}</span> },
  { label: "SJR", title: tk("Nilai SJR jurnal"), cell: (j) => (j.sjr !== null ? formatDecimal(j.sjr, 2) : "-"), wide: true },
  { label: tk("Cocok"), title: tk("Dokumen jurnal ini yang cocok dengan celah"), cell: (j) => formatInt(j.matching) },
  { label: tk("Terbaru"), title: tk("Dokumen cocok dalam tiga tahun terakhir koleksi"), cell: (j) => formatInt(j.recent) },
  { label: tk("Sitasi"), title: tk("Total sitasi dokumen yang cocok"), cell: (j) => formatInt(j.citations), wide: true },
  { label: tk("Dok"), title: tk("Semua dokumen jurnal ini di koleksi"), cell: (j) => formatInt(j.documents), wide: true },
  { label: tk("Skor"), title: tk("Skor peringkat"), cell: (j) => formatDecimal(j.score, 1) },
];

function gapLine(gaps: string[]): string {
  const head = gaps.slice(0, SHOWN_GAPS).join("; ");
  return gaps.length > SHOWN_GAPS ? t("{gaps} +{n} lainnya", { gaps: head, n: gaps.length - SHOWN_GAPS }) : head;
}

/** Ranked target journals; the gaps each journal matches sit under its name. */
export function JournalTable({ rows }: { rows: JournalPick[] }) {
  const template = { "--cols": COLUMNS.length, "--cols-narrow": COLUMNS.filter((c) => !c.wide).length } as CSSProperties;
  const head = (c: Col) => (c.wide ? "cell-right ent-wide" : "cell-right");
  const cls = (c: Col) => `num ${head(c)}`;
  return (
    <div className="ent-table jr-table" role="table" aria-label={t("Jurnal tujuan")}>
      <div className="ent-row ent-head" role="row" style={template}>
        <span role="columnheader">#</span>
        <span role="columnheader">{t("Jurnal dan celah yang cocok")}</span>
        {COLUMNS.map((c) => <span key={c.label} role="columnheader" className={head(c)} title={t(c.title)}>{t(c.label)}</span>)}
      </div>
      {rows.map((j, i) => (
        <div key={j.label} className="ent-row jr-row" role="row" style={template}>
          <span className="num muted" role="cell">{i + 1}</span>
          <span className="jr-name" role="cell">
            <span className="ent-name" title={j.label}>{j.label}</span>
            <span className="jr-gaps muted" title={j.gaps.join("; ")}>{gapLine(j.gaps)}</span>
          </span>
          {COLUMNS.map((c) => <span key={c.label} role="cell" className={cls(c)}>{c.cell(j)}</span>)}
        </div>
      ))}
    </div>
  );
}
