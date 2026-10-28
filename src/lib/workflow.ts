import type { Jump } from "./commands";
import { formatInt } from "./format";
import { t, tk } from "../i18n";

export interface WorkflowStatus {
  documents: number;
  origins: number;
  withAbstract: number;
  withKeywords: number;
  duplicates: number;
  synonymSuggestions: number;
  synonyms: number;
  nameSuggestions: number;
  nameMerges: number;
  gaps: number;
  /** Most cited references checked (up to 50), and those missing from the collection. */
  coreListed: number;
  coreMissing: number;
  reviewed: number;
  accepted: number;
  verified: number;
}

export type StepState = "done" | "todo" | "attention" | "ready";

export interface Step {
  title: string;
  text: string;
  state: StepState;
  facts: string[];
  actions: { label: string; jump: Jump }[];
}

/** Keys; render with t(). */
export const STATE_LABEL: Record<StepState, string> = { done: tk("Selesai"), todo: tk("Belum"), attention: tk("Perlu perhatian"), ready: tk("Siap") };

const n = formatInt;
const pct = (part: number, total: number) => (total ? `${Math.round((part / total) * 100)}%` : "0%");

/** The six workflow steps with their state, worked out from the counts. */
export function steps(w: WorkflowStatus): Step[] {
  const empty = w.documents === 0;
  const cleaning = w.duplicates > 0 || (w.synonymSuggestions > 0 && w.synonyms === 0) || (w.nameSuggestions > 0 && w.nameMerges === 0);
  return [
    { title: t("Kumpulkan dokumen"), text: t("Cari dari OpenAlex, PubMed, dan sumber lain, atau impor ekspor Scopus/WoS. Kumpulan yang memadai biasanya ratusan dokumen."),
      state: empty ? "todo" : w.documents < 50 ? "attention" : "done",
      facts: [t("{n} dokumen dari {origins} sumber", { n: n(w.documents), origins: n(w.origins) }), t("abstrak {pct}", { pct: pct(w.withAbstract, w.documents) }), t("kata kunci {pct}", { pct: pct(w.withKeywords, w.documents) })],
      actions: [{ label: t("Koleksi & impor"), jump: { screen: "collection", tab: "collection" } }, { label: t("Kualitas metadata"), jump: { screen: "collection", tab: "quality" } }] },
    { title: t("Rapikan data"), text: t("Gabungkan duplikat, satukan sinonim kata kunci dan ejaan nama penulis, supaya analisis tidak terpecah dan celah palsu tidak muncul."),
      state: empty ? "todo" : cleaning ? "attention" : "done",
      facts: [t("{n} kandidat duplikat", { n: n(w.duplicates) }), t("{n} saran sinonim, {active} aktif", { n: n(w.synonymSuggestions), active: n(w.synonyms) }), t("{n} saran nama, {merged} disatukan", { n: n(w.nameSuggestions), merged: n(w.nameMerges) })],
      actions: [{ label: t("Duplikat"), jump: { screen: "collection", tab: "duplicates" } }, { label: t("Sinonim"), jump: { screen: "conceptual", tab: "synonyms" } }, { label: t("Nama penulis"), jump: { screen: "authors", tab: "authors" } }] },
    { title: t("Jelajahi struktur bidang"), text: t("Pahami tema, tokoh, dan aliran riset: peta tematik, ko-sitasi, kopling, dan historiograf. Tambahkan rujukan inti yang belum ada di koleksi."),
      state: empty ? "todo" : "ready",
      facts: w.coreListed ? [t("{n} dari {total} rujukan teratas belum ada di koleksi", { n: n(w.coreMissing), total: n(w.coreListed) })] : [],
      actions: [{ label: t("Ringkasan"), jump: { screen: "overview" } }, { label: t("Peta tematik"), jump: { screen: "conceptual", tab: "map" } },
        { label: t("Rujukan teratas"), jump: { screen: "intellectual", tab: "toprefs" } }, { label: t("Kopling"), jump: { screen: "intellectual", tab: "coupling" } }] },
    { title: t("Temukan dan review celah"), text: t("Tinjau kandidat dari Gap Explorer atau uji ide Anda sendiri, lalu tandai Diterima, Perlu cek, atau Ditolak."),
      state: w.gaps === 0 && w.reviewed === 0 ? "todo" : w.accepted === 0 ? "attention" : "done",
      facts: [t("{n} kandidat", { n: n(w.gaps) }), t("{n} sudah direview", { n: n(w.reviewed) }), t("{n} diterima", { n: n(w.accepted) })],
      actions: [{ label: t("Tema peluang"), jump: { screen: "opportunities" } }, { label: t("Gap Explorer"), jump: { screen: "gaps", tab: "explorer" } },
        { label: t("Uji ide celah"), jump: { screen: "gaps", tab: "idea" } }] },
    { title: t("Verifikasi celah yang diterima"), text: t("Pastikan celah belum dijawab oleh paper terbaru di luar koleksi Anda."),
      state: w.accepted === 0 ? "todo" : w.verified < w.accepted ? "attention" : "done",
      facts: [t("{n} dari {total} celah diterima sudah diverifikasi", { n: n(w.verified), total: n(w.accepted) })],
      actions: [{ label: t("Verifikasi"), jump: { screen: "verify" } }] },
    { title: t("Susun laporan"), text: t("Draf metode dan hasil bibliometrik serta paragraf research gap dengan sitasi APA atau IEEE, lalu simpan sebagai Markdown, BibTeX, atau RIS."),
      state: empty ? "todo" : "ready", facts: w.accepted ? [t("{n} celah siap dimasukkan", { n: n(w.accepted) })] : [t("Draf metode dan hasil sudah bisa disusun")],
      actions: [{ label: t("Draf metode & hasil"), jump: { screen: "report", tab: "summary" } }, { label: t("Draf research gap"), jump: { screen: "report", tab: "draft" } }] },
  ];
}
