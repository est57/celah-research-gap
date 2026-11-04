// Title and abstract screening and the PRISMA 2020 flow figures.
import { tk } from "../i18n";

export type Decision = "include" | "exclude";
export type ScreenStatus = "none" | "include" | "exclude" | "all";
export type Reason = "topic" | "type" | "language" | "access" | "design" | "other";

export interface ScreenRow {
  id: number;
  title: string;
  authors: string;
  year: number | null;
  sourceName: string | null;
  abstractText: string | null;
  keywords: string | null;
  /** "" when not screened yet. */
  decision: Decision | "";
  reason: Reason | "";
  note: string;
}

export interface ScreenCounts {
  unscreened: number;
  included: number;
  excluded: number;
}

export interface ScreenPage {
  items: ScreenRow[];
  total: number;
  counts: ScreenCounts;
}

/** Exclusion reasons, in the order offered; labels go through t() where shown. */
export const REASON_LABEL: Record<Reason, string> = {
  topic: tk("Di luar topik"),
  type: tk("Bukan artikel penelitian"),
  language: tk("Bahasa tidak sesuai"),
  access: tk("Teks lengkap tidak tersedia"),
  design: tk("Desain studi tidak sesuai"),
  other: tk("Alasan lain"),
};

export interface PrismaCount {
  /** Import origin or reason code; "" for records merged before logging began. */
  label: string;
  n: number;
}

export interface Prisma {
  sources: PrismaCount[];
  identified: number;
  duplicates: number;
  removedOther: number;
  screened: number;
  excluded: number;
  reasons: PrismaCount[];
  unscreened: number;
  filterActive: boolean;
  filteredOut: number;
  included: number;
}
