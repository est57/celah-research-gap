import type { Jump } from "./commands";
import { formatInt } from "./format";

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

export const STATE_LABEL: Record<StepState, string> = { done: "Selesai", todo: "Belum", attention: "Perlu perhatian", ready: "Siap" };

const n = formatInt;
const pct = (part: number, total: number) => (total ? `${Math.round((part / total) * 100)}%` : "0%");

/** The six workflow steps with their state, worked out from the counts. */
export function steps(w: WorkflowStatus): Step[] {
  const empty = w.documents === 0;
  const cleaning = w.duplicates > 0 || (w.synonymSuggestions > 0 && w.synonyms === 0) || (w.nameSuggestions > 0 && w.nameMerges === 0);
  return [
    { title: "Kumpulkan dokumen", text: "Cari dari OpenAlex, PubMed, dan sumber lain, atau impor ekspor Scopus/WoS. Kumpulan yang memadai biasanya ratusan dokumen.",
      state: empty ? "todo" : w.documents < 50 ? "attention" : "done",
      facts: [`${n(w.documents)} dokumen dari ${n(w.origins)} sumber`, `abstrak ${pct(w.withAbstract, w.documents)}`, `kata kunci ${pct(w.withKeywords, w.documents)}`],
      actions: [{ label: "Koleksi & impor", jump: { screen: "collection", tab: "collection" } }, { label: "Kualitas metadata", jump: { screen: "collection", tab: "quality" } }] },
    { title: "Rapikan data", text: "Gabungkan duplikat, satukan sinonim kata kunci dan ejaan nama penulis, supaya analisis tidak terpecah dan celah palsu tidak muncul.",
      state: empty ? "todo" : cleaning ? "attention" : "done",
      facts: [`${n(w.duplicates)} kandidat duplikat`, `${n(w.synonymSuggestions)} saran sinonim, ${n(w.synonyms)} aktif`, `${n(w.nameSuggestions)} saran nama, ${n(w.nameMerges)} disatukan`],
      actions: [{ label: "Duplikat", jump: { screen: "collection", tab: "duplicates" } }, { label: "Sinonim", jump: { screen: "conceptual", tab: "synonyms" } }, { label: "Nama penulis", jump: { screen: "authors", tab: "authors" } }] },
    { title: "Jelajahi struktur bidang", text: "Pahami tema, tokoh, dan aliran riset: peta tematik, ko-sitasi, kopling, dan historiograf. Tambahkan rujukan inti yang belum ada di koleksi.",
      state: empty ? "todo" : "ready",
      facts: w.coreListed ? [`${n(w.coreMissing)} dari ${n(w.coreListed)} rujukan teratas belum ada di koleksi`] : [],
      actions: [{ label: "Ringkasan", jump: { screen: "overview" } }, { label: "Peta tematik", jump: { screen: "conceptual", tab: "map" } },
        { label: "Rujukan teratas", jump: { screen: "intellectual", tab: "toprefs" } }, { label: "Kopling", jump: { screen: "intellectual", tab: "coupling" } }] },
    { title: "Temukan dan review celah", text: "Tinjau kandidat dari Gap Explorer atau uji ide Anda sendiri, lalu tandai Diterima, Perlu cek, atau Ditolak.",
      state: w.gaps === 0 && w.reviewed === 0 ? "todo" : w.accepted === 0 ? "attention" : "done",
      facts: [`${n(w.gaps)} kandidat`, `${n(w.reviewed)} sudah direview`, `${n(w.accepted)} diterima`],
      actions: [{ label: "Gap Explorer", jump: { screen: "gaps", tab: "explorer" } }, { label: "Uji ide celah", jump: { screen: "gaps", tab: "idea" } }] },
    { title: "Verifikasi celah yang diterima", text: "Pastikan celah belum dijawab oleh paper terbaru di luar koleksi Anda.",
      state: w.accepted === 0 ? "todo" : w.verified < w.accepted ? "attention" : "done",
      facts: [`${n(w.verified)} dari ${n(w.accepted)} celah diterima sudah diverifikasi`],
      actions: [{ label: "Verifikasi", jump: { screen: "verify", tab: "verify" } }] },
    { title: "Susun laporan", text: "Draf metode dan hasil bibliometrik serta paragraf research gap dengan sitasi APA atau IEEE, lalu simpan sebagai Markdown, BibTeX, atau RIS.",
      state: empty ? "todo" : "ready", facts: w.accepted ? [`${n(w.accepted)} celah siap dimasukkan`] : ["Draf metode dan hasil sudah bisa disusun"],
      actions: [{ label: "Draf metode & hasil", jump: { screen: "report", tab: "summary" } }, { label: "Draf research gap", jump: { screen: "report", tab: "draft" } }] },
  ];
}
