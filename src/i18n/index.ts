// Interface language. Indonesian source text is the key: t("Simpan") returns it as is in
// Indonesian and looks up the English dictionary otherwise. Every key appears once as a
// string literal, in t("...") or tk("..."), so the completeness test (i18n.test.ts) finds it.
// Never call t() at module level: the language can change after the module loads.
import { EN } from "./en";

export type Lang = "id" | "en";

let lang: Lang = "id";
const listeners = new Set<(l: Lang) => void>();

export function getLang(): Lang {
  return lang;
}

/** Switches the language; the app root re-renders through onLangChange. */
export function setLang(next: Lang): void {
  if (next === lang) return;
  lang = next;
  document.documentElement.lang = next;
  listeners.forEach((f) => f(next));
}

export function onLangChange(f: (l: Lang) => void): () => void {
  listeners.add(f);
  return () => {
    listeners.delete(f);
  };
}

/** The text in the current language; "{name}" placeholders are filled from `vars`. */
export function t(text: string, vars?: Record<string, string | number>): string {
  const out = lang === "en" ? (EN[text] ?? text) : text;
  return vars ? out.replace(/\{(\w+)\}/g, (m, k: string) => (k in vars ? String(vars[k]) : m)) : out;
}


/** Marks a key kept in a constant and translated later with t(value) at render time. */
export function tk(text: string): string {
  return text;
}
