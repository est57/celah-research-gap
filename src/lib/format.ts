import { t, tk } from "../i18n";

// "id-ID" writes 1.234,5; "en-US" writes 1,234.5. Changed from the display settings.
let locale = "id-ID";
let intFmt = new Intl.NumberFormat(locale);
let pctFmt = new Intl.NumberFormat(locale, { minimumFractionDigits: 1, maximumFractionDigits: 1 });

export function setNumberLocale(choice: "id" | "en"): void {
  locale = choice === "en" ? "en-US" : "id-ID";
  intFmt = new Intl.NumberFormat(locale);
  pctFmt = new Intl.NumberFormat(locale, { minimumFractionDigits: 1, maximumFractionDigits: 1 });
}

export function formatInt(n: number): string {
  return intFmt.format(n);
}

export function formatDecimal(n: number, digits = 1): string {
  return n.toLocaleString(locale, { minimumFractionDigits: digits, maximumFractionDigits: digits });
}

export function formatPct(part: number, total: number): string {
  return `${pctFmt.format(total > 0 ? (part / total) * 100 : 0)}%`;
}

export interface Quality {
  label: string;
  color: string;
}

// Thresholds follow the bibliometrix missing-data convention.
export function quality(missing: number, total: number): Quality {
  if (total === 0) return { label: t("Tidak ada data"), color: "#8a929c" };
  const pct = (missing / total) * 100;
  if (pct === 0) return { label: t("Sangat baik"), color: "var(--ok)" };
  if (pct < 10) return { label: t("Baik"), color: "var(--ok-2)" };
  if (pct < 20) return { label: t("Cukup"), color: "var(--warn)" };
  if (pct < 50) return { label: t("Buruk"), color: "var(--bad)" };
  if (pct < 100) return { label: t("Kritis"), color: "var(--bad)" };
  return { label: t("Tidak ada"), color: "#8a929c" };
}

const docTypeLabels: Record<string, string> = {
  article: tk("Artikel"),
  review: tk("Review"),
  preprint: tk("Pra-cetak"),
  proceedings: tk("Prosiding"),
  "book-chapter": tk("Bab buku"),
};

export function docTypeLabel(type: string | null): string {
  if (!type) return "-";
  const label = docTypeLabels[type];
  return label ? t(label) : type;
}
