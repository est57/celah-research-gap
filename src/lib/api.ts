import { invoke } from "@tauri-apps/api/core";
import type {
  CollectionStats,
  FileImportReport,
  KeyStatus,
  ListInput,
  PaperPage,
  SearchInput,
  ServiceId,
  SourceReport,
} from "./types";
import type { Overview } from "./overviewTypes";
import type { SourcesAuthors } from "./sourcesTypes";
import type { Conceptual, ConceptualParams } from "./conceptualTypes";
import type { Structure, StructureParams } from "./structureTypes";
import type { Countries } from "./countriesTypes";
import type { GapList, ReviewEntry, ReviewStatus, VerifyReport } from "./gapTypes";
import type { ExportFormat, Report, ReportParams } from "./reportTypes";
import type { Evolution, EvolutionParams } from "./evolutionTypes";
import type { Factorial, FactorialParams } from "./factorialTypes";
import type { Historiograph } from "./histTypes";
import type { Cited, CitedSort } from "./citedTypes";
import type { ThreeField, ThreeFieldParams } from "./overviewTypes";
import type { AuthorRow, SourcesDetail } from "./entityTypes";
import { t } from "../i18n";

export function isDesktop(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isDesktop()) {
    throw new Error(t("Backend desktop tidak tersedia. Jalankan dengan npm run tauri dev."));
  }
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    // The backend serializes errors as safe, user-facing strings.
    throw new Error(typeof err === "string" ? err : t("Terjadi kesalahan yang tidak dikenal."));
  }
}

// One report per source; a failing source carries `error` instead of rejecting the call.
export function searchSources(input: SearchInput): Promise<SourceReport[]> {
  return call<SourceReport[]>("search_sources", { input });
}

// The backend opens the native file picker itself; resolves to null when cancelled.
export function importFile(): Promise<FileImportReport | null> {
  return call<FileImportReport | null>("import_file");
}

export function listPapers(input: ListInput): Promise<PaperPage> {
  return call<PaperPage>("list_papers", { input });
}

export function collectionStats(): Promise<CollectionStats> {
  return call<CollectionStats>("collection_stats");
}

export function overview(): Promise<Overview> {
  return call<Overview>("overview");
}

export function conceptual(params: ConceptualParams): Promise<Conceptual> {
  return call<Conceptual>("conceptual", { params });
}

export function evolution(params: EvolutionParams): Promise<Evolution> {
  return call<Evolution>("evolution", { params });
}

export function factorial(params: FactorialParams): Promise<Factorial> {
  return call<Factorial>("factorial", { params });
}

export function cited(sort: CitedSort, limit: number): Promise<Cited> {
  return call<Cited>("cited", { params: { sort, limit } });
}

export function sourcesDetail(): Promise<SourcesDetail> {
  return call<SourcesDetail>("sources_detail");
}

export function authorTable(): Promise<AuthorRow[]> {
  return call<AuthorRow[]>("author_table");
}

export function threeField(params: ThreeFieldParams): Promise<ThreeField> {
  return call<ThreeField>("three_field", { params });
}

export function historiograph(nodes: number): Promise<Historiograph> {
  return call<Historiograph>("historiograph", { params: { nodes } });
}

export function structure(params: StructureParams): Promise<Structure> {
  return call<Structure>("structure", { params });
}

export function gaps(): Promise<GapList> {
  return call<GapList>("gaps");
}

/** An empty status clears the review. `title` is kept so the review history stays readable. */
export function setGapReview(key: string, status: ReviewStatus, note: string, title: string): Promise<void> {
  return call<void>("set_gap_review", { key, status, note, title });
}

export function gapHistory(): Promise<ReviewEntry[]> {
  return call<ReviewEntry[]>("gap_history");
}

/** Verifies the given gaps, or every accepted gap when `keys` is empty. */
export function verifyGaps(keys: string[]): Promise<VerifyReport[]> {
  return call<VerifyReport[]>("verify_gaps", { keys });
}

export function report(params: ReportParams): Promise<Report> {
  return call<Report>("report", { params });
}

// The backend opens the native save dialog itself; resolves to the file name, or null when cancelled.
export function exportReport(params: ReportParams, format: ExportFormat): Promise<string | null> {
  return call<string | null>("export_report", { params, format });
}

export function countries(): Promise<Countries> {
  return call<Countries>("countries");
}

export function sourcesAuthors(): Promise<SourcesAuthors> {
  return call<SourcesAuthors>("sources_authors");
}

// The backend only ever returns a masked hint, never the stored key.
export function apiKeyStatus(service: ServiceId): Promise<KeyStatus> {
  return call<KeyStatus>("api_key_status", { service });
}

export function setApiKey(service: ServiceId, key: string): Promise<KeyStatus> {
  return call<KeyStatus>("set_api_key", { service, key });
}

export function clearApiKey(service: ServiceId): Promise<KeyStatus> {
  return call<KeyStatus>("clear_api_key", { service });
}
