// Collection management calls: metadata quality, duplicates and the analysis filter.
import { call } from "./api";
import type { DuplicateList } from "./duplicateTypes";
import type { CollectionFilter, FilterOptions } from "./filterTypes";
import type { MetadataQuality } from "./qualityTypes";

export function metadataQuality(): Promise<MetadataQuality> {
  return call<MetadataQuality>("metadata_quality");
}

export function filterOptions(): Promise<FilterOptions> {
  return call<FilterOptions>("filter_options");
}

/** Documents the filter would keep, without saving it. */
export function previewFilter(filter: CollectionFilter): Promise<number> {
  return call<number>("preview_filter", { filter });
}

/** Saves the filter for every analysis; an empty filter removes it. */
export function setCollectionFilter(filter: CollectionFilter): Promise<FilterOptions> {
  return call<FilterOptions>("set_collection_filter", { filter });
}

export function duplicates(): Promise<DuplicateList> {
  return call<DuplicateList>("duplicates");
}

/** Keeps `keep`, fills its empty fields from `drop`, then removes `drop` from the collection. */
export function mergeDuplicate(keep: number, drop: number): Promise<void> {
  return call<void>("merge_duplicate", { keep, drop });
}

export function dismissDuplicate(a: number, b: number): Promise<void> {
  return call<void>("dismiss_duplicate", { a, b });
}

export interface EnrichReport {
  checked: number;
  gained: number;
  remaining: number;
  error: string | null;
}

/** Documents with an OpenAlex id or DOI whose cited works were never looked up. */
export function referencesPending(): Promise<number> {
  return call<number>("references_pending");
}

/** Fetches cited works from OpenAlex for up to 2,000 such documents. */
export function completeReferences(): Promise<EnrichReport> {
  return call<EnrichReport>("complete_references");
}

export function textReferencesPending(): Promise<number> {
  return call<number>("text_references_pending");
}

/** Reference lists from Crossref for documents with a DOI and none yet; up to 600 per run. */
export function completeTextReferences(): Promise<EnrichReport> {
  return call<EnrichReport>("complete_text_references");
}

/** Removes the selected documents; resolves to how many were removed. */
export function deletePapers(ids: number[]): Promise<number> {
  return call<number>("delete_papers", { ids });
}

/** Removes every document imported from one source, e.g. "Crossref". */
export function deleteOrigin(origin: string): Promise<number> {
  return call<number>("delete_origin", { origin });
}

export type CollectionFormat = "csv" | "bibtex" | "ris";

/** Opens the native save dialog; resolves to null when cancelled. */
export function exportCollection(format: CollectionFormat, scope: "all" | "filtered"): Promise<{ file: string; count: number } | null> {
  return call<{ file: string; count: number } | null>("export_collection", { format, scope });
}

/** Copies the whole database to a chosen file; resolves to its name, or null when cancelled. */
export function backupDatabase(): Promise<string | null> {
  return call<string | null>("backup_database");
}

export interface BackupInfo {
  documents: number;
  reviews: number;
  schema: number;
}

/** Step 1 of a restore: pick and check a backup file (the path stays in the backend). */
export function chooseBackup(): Promise<BackupInfo | null> {
  return call<BackupInfo | null>("choose_backup");
}

export function cancelRestore(): Promise<void> {
  return call<void>("cancel_restore");
}

/** Step 2: saves the current data automatically, then replaces it with the chosen backup. */
export function restoreBackup(): Promise<{ documents: number; safetyFile: string }> {
  return call<{ documents: number; safetyFile: string }>("restore_backup");
}
