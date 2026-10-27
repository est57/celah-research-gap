import type { CheckPaper } from "./gapTypes";

export interface DocRef {
  id: number;
  title: string;
  authors: string;
  year: number | null;
  citedBy: number;
}

export type IdeaVerdict = "kurang-data" | "celah" | "jarang" | "banyak";

export interface Idea {
  a: string[];
  b: string[];
  documents: number;
  withA: number;
  withB: number;
  withBoth: number;
  /** Documents with both if the concepts were unrelated. */
  expected: number;
  verdict: IdeaVerdict;
  recentA: number;
  recentB: number;
  recentAll: number;
  years: number[];
  /** Per year: [concept A, concept B, both]. */
  perYear: [number[], number[], number[]];
  bothDocs: DocRef[];
  aOnly: DocRef[];
  bOnly: DocRef[];
}

export interface IdeaCheck {
  query: string;
  found: number;
  matching: number;
  verdict: "open" | "new" | "answered";
  papers: CheckPaper[];
}

export const IDEA_VERDICT: Record<IdeaVerdict, { title: string; text: string }> = {
  celah: { title: "Kemungkinan celah", text: "Kedua konsep sama-sama diteliti, tetapi jauh lebih jarang digabungkan dari yang diharapkan." },
  jarang: { title: "Agak jarang digabungkan", text: "Kombinasi ini sudah ada, tetapi di bawah perkiraan. Celahnya mungkin pada aspek yang lebih spesifik." },
  banyak: { title: "Sudah banyak diteliti", text: "Kombinasi ini muncul sesering atau lebih sering dari perkiraan. Cari sudut yang lebih sempit." },
  "kurang-data": { title: "Data belum cukup", text: "Salah satu konsep muncul di kurang dari 3 dokumen. Perluas koleksi atau tambah sebutan lain untuk konsep itu." },
};
