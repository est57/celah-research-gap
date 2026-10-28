import { tk } from "../i18n";
export type WordField = "author_keywords" | "keywords_plus" | "title" | "abstract";

export interface WordParams {
  field: WordField;
  /** Words per phrase for titles and abstracts (1 to 3); keyword fields ignore it. */
  ngram: number;
  top: number;
}

export const DEFAULT_WORDS: WordParams = { field: "author_keywords", ngram: 1, top: 30 };

export const WORD_FIELDS: { id: WordField; label: string }[] = [
  { id: "author_keywords", label: tk("Kata kunci penulis") },
  { id: "keywords_plus", label: tk("Keywords Plus") },
  { id: "title", label: tk("Judul") },
  { id: "abstract", label: tk("Abstrak") },
];

export const isFreeText = (f: WordField) => f === "title" || f === "abstract";

export interface Words {
  field: string;
  ngram: number;
  documents: number;
  withField: number;
  distinct: number;
  words: { label: string; documents: number }[];
  years: number[];
  /** Documents per year for the top words, one value per year in `years`. */
  series: { label: string; perYear: number[] }[];
}
