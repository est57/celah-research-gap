import { isDesktop } from "./api";
import { setNumberLocale } from "./format";
import { setLang, type Lang } from "../i18n";

export type ThemeChoice = "system" | "light" | "dark";
export type NumberLocale = "id" | "en";

export interface DisplaySettings {
  theme: ThemeChoice;
  /** Webview zoom factor. */
  scale: number;
  /** Interface language. */
  language: Lang;
  numbers: NumberLocale;
  reportStyle: "apa" | "ieee";
  reportLanguage: "id" | "en";
  reduceMotion: boolean;
}

export const SCALES = [0.9, 1, 1.1, 1.25];
export const DEFAULT_DISPLAY: DisplaySettings = { theme: "system", scale: 1, language: "id", numbers: "id", reportStyle: "apa", reportLanguage: "id", reduceMotion: false };

const KEY = "celah.display";
const pick = <T>(value: unknown, allowed: readonly T[], fallback: T): T => (allowed.includes(value as T) ? (value as T) : fallback);

/** Stored settings, each value checked against its allowed set; storage errors fall back to defaults. */
export function loadDisplay(): DisplaySettings {
  let raw: Record<string, unknown> = {};
  try {
    const parsed: unknown = JSON.parse(localStorage.getItem(KEY) ?? "{}");
    if (parsed && typeof parsed === "object") raw = parsed as Record<string, unknown>;
  } catch {
    raw = {};
  }
  const d = DEFAULT_DISPLAY;
  return {
    theme: pick(raw.theme, ["system", "light", "dark"] as const, d.theme),
    scale: pick(raw.scale, SCALES, d.scale),
    language: pick(raw.language, ["id", "en"] as const, d.language),
    numbers: pick(raw.numbers, ["id", "en"] as const, d.numbers),
    reportStyle: pick(raw.reportStyle, ["apa", "ieee"] as const, d.reportStyle),
    reportLanguage: pick(raw.reportLanguage, ["id", "en"] as const, d.reportLanguage),
    reduceMotion: raw.reduceMotion === true,
  };
}

export function saveDisplay(s: DisplaySettings): void {
  try {
    localStorage.setItem(KEY, JSON.stringify(s));
  } catch {
    // Storage can be unavailable; the settings still apply for this session.
  }
}

const systemDark = () => typeof window !== "undefined" && window.matchMedia?.("(prefers-color-scheme: dark)").matches === true;

export function applyDisplay(s: DisplaySettings): void {
  const root = document.documentElement;
  root.dataset.theme = s.theme === "dark" || (s.theme === "system" && systemDark()) ? "dark" : "light";
  root.dataset.motion = s.reduceMotion ? "reduce" : "";
  setNumberLocale(s.numbers);
  setLang(s.language);
  if (isDesktop()) {
    // Backend messages and generated explanations follow the interface language.
    import("@tauri-apps/api/core")
      .then(({ invoke }) => invoke("set_language", { language: s.language }))
      .catch(() => undefined);
    import("@tauri-apps/api/webview")
      .then(({ getCurrentWebview }) => getCurrentWebview().setZoom(s.scale))
      .catch(() => undefined);
  }
}

/** Re-applies when the system theme changes while "system" is chosen. Returns the cleanup. */
export function followSystemTheme(get: () => DisplaySettings): () => void {
  const media = window.matchMedia?.("(prefers-color-scheme: dark)");
  if (!media) return () => undefined;
  const onChange = () => get().theme === "system" && applyDisplay(get());
  media.addEventListener("change", onChange);
  return () => media.removeEventListener("change", onChange);
}
