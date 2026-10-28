import { getLang, type Lang } from "../i18n";

// One DisplayNames per interface language, made on first use; null where Intl lacks it.
const cache = new Map<Lang, Intl.DisplayNames | null>();

function names(lang: Lang): Intl.DisplayNames | null {
  if (!cache.has(lang)) {
    let made: Intl.DisplayNames | null = null;
    try {
      made = new Intl.DisplayNames([lang], { type: "region" });
    } catch {
      made = null;
    }
    cache.set(lang, made);
  }
  return cache.get(lang) ?? null;
}

/** Country name in the interface language for an ISO alpha-2 code ("CN" -> "Tiongkok"), or the code itself. */
export function countryName(code: string): string {
  try {
    return names(getLang())?.of(code) ?? code;
  } catch {
    return code;
  }
}
