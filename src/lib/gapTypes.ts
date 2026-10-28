import { tk } from "../i18n";
import type { ScreenId } from "../nav";

export type GapKind = "matrix" | "emerging" | "niche" | "structural" | "front" | "explicit" | "idea";
export type ReviewStatus = "accepted" | "check" | "rejected" | "";

export interface GapSignal {
  analysis: string;
  finding: string;
  screen: ScreenId;
  /** Tab of that screen, when not its first one. */
  tab?: string;
}

export interface ScorePart {
  label: string;
  value: number;
}

export interface GapEvidence {
  paperId: number;
  title: string;
  year: number | null;
  source: string | null;
  authors: string;
  citedBy: number;
  role: "mendukung" | "hampir menjawab" | "konteks";
  quote: string | null;
}

export type Verdict = "open" | "new" | "answered";

export interface CheckPaper {
  title: string;
  year: number | null;
  doi: string | null;
  source: string | null;
  citedBy: number;
  matches: boolean;
}

export interface CheckSummary {
  checkedAt: string;
  query: string;
  found: number;
  matching: number;
  verdict: Verdict;
  papers: CheckPaper[];
}

export interface VerifyReport {
  key: string;
  found: number;
  matching: number;
  verdict: Verdict | "";
  error: string | null;
}

export interface Gap {
  key: string;
  kind: GapKind;
  title: string;
  statement: string;
  terms: string[];
  check: CheckSummary | null;
  score: number;
  confidence: "Kuat" | "Sedang" | "Lemah";
  parts: ScorePart[];
  signals: GapSignal[];
  evidence: GapEvidence[];
  questions: string[];
  status: ReviewStatus;
  note: string;
}

export interface GapList {
  documents: number;
  field: string;
  gaps: Gap[];
}

export const KIND_INFO: Record<GapKind, { label: string; letter: string; color: string }> = {
  matrix: { label: tk("Kombinasi jarang"), letter: "K", color: "#2456A6" },
  emerging: { label: tk("Baru muncul"), letter: "B", color: "#7A5BB0" },
  niche: { label: tk("Niche"), letter: "N", color: "#2E8B6E" },
  structural: { label: tk("Struktural"), letter: "S", color: "#D9622B" },
  front: { label: tk("Front terisolasi"), letter: "F", color: "#4F6D2E" },
  explicit: { label: tk("Eksplisit"), letter: "E", color: "#B8457A" },
  idea: { label: tk("Ide peneliti"), letter: "I", color: "#3B8FB5" },
};

export const STATUS_LABEL: Record<Exclude<ReviewStatus, "">, string> = {
  accepted: tk("Diterima"),
  check: tk("Perlu cek"),
  rejected: tk("Ditolak"),
};

export const VERDICT_LABEL: Record<Verdict, string> = {
  open: tk("Masih terbuka"),
  new: tk("Ada studi baru, perlu dibaca"),
  answered: tk("Mungkin sudah dijawab"),
};

/** A stored review, including gaps no longer among the current candidates. */
export interface ReviewEntry {
  key: string;
  kind: GapKind | string;
  title: string;
  status: Exclude<ReviewStatus, "">;
  note: string;
  updatedAt: string;
  inAnalysis: boolean;
  score: number | null;
  verdict: Verdict | null;
  checkedAt: string | null;
}
