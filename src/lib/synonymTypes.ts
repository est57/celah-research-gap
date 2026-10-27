export interface KeywordTerm {
  label: string;
  documents: number;
}

export interface KeywordSuggestion {
  /** "ejaan" (spelling or plural) or "singkatan" (acronym and expansion). */
  kind: "ejaan" | "singkatan";
  /** Most frequent first; the first is the suggested name to keep. */
  terms: KeywordTerm[];
}

export interface KeywordSynonym {
  fromKey: string;
  fromLabel: string;
  toKey: string;
  toLabel: string;
}

export interface KeywordSynonyms {
  suggestions: KeywordSuggestion[];
  synonyms: KeywordSynonym[];
  terms: KeywordTerm[];
}
