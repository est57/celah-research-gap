import { t } from "../../i18n";
import { REASON_LABEL, type Prisma, type Reason } from "../../lib/screeningTypes";

export interface Box {
  kind: "main" | "side";
  /** First line is the heading. */
  lines: string[];
}

export interface Row {
  /** 0 identification, 1 screening, 2 included. */
  phase: number;
  main: Box;
  side: Box | null;
}

/** "- Scopus (n = 160)": one indented item of a box. */
const item = (label: string, n: string) => t("- {label} (n = {n})", { label, n });

/** The rows of the PRISMA 2020 diagram, texts in the interface language. */
export function prismaBoxes(p: Prisma, fmt: (n: number) => string): Row[] {
  const source = (label: string) => (label ? label : t("Rekaman lama yang sudah digabung"));
  const rows: Row[] = [
    {
      phase: 0,
      main: { kind: "main", lines: [t("Rekaman dari basis data (n = {n})", { n: fmt(p.identified) }), ...p.sources.map((s) => item(source(s.label), fmt(s.n)))] },
      side: {
        kind: "side",
        lines: [
          t("Rekaman dihapus sebelum skrining:"),
          item(t("Duplikat"), fmt(p.duplicates)),
          item(t("Dihapus karena alasan lain"), fmt(p.removedOther)),
        ],
      },
    },
    {
      phase: 1,
      main: { kind: "main", lines: [t("Rekaman disaring (n = {n})", { n: fmt(p.screened) }), t("judul dan abstrak")] },
      side: {
        kind: "side",
        lines: [
          t("Rekaman dikecualikan (n = {n})", { n: fmt(p.excluded) }),
          ...p.reasons.map((r) => item(t(REASON_LABEL[r.label as Reason] ?? REASON_LABEL.other), fmt(r.n))),
        ],
      },
    },
  ];
  if (p.filterActive) {
    rows.push({
      phase: 1,
      main: { kind: "main", lines: [t("Rekaman dinilai kelayakannya (n = {n})", { n: fmt(p.screened - p.excluded) }), t("kriteria filter analisis")] },
      side: { kind: "side", lines: [t("Dikecualikan oleh kriteria (n = {n})", { n: fmt(p.filteredOut) }), t("tahun, jenis dokumen, sumber, atau kuartil")] },
    });
  }
  const unscreened = p.unscreened > 0 ? [t("termasuk {n} yang belum disaring", { n: fmt(p.unscreened) })] : [];
  rows.push({ phase: 2, main: { kind: "main", lines: [t("Studi dalam analisis (n = {n})", { n: fmt(p.included) }), ...unscreened] }, side: null });
  return rows;
}
