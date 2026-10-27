// Calls for the source, author and affiliation tables (kept apart to keep api.ts short).
import { call } from "./api";
import type { Affiliations } from "./entityTypes";

export function affiliations(): Promise<Affiliations> {
  return call<Affiliations>("affiliations");
}

export interface QuickResults {
  documents: { id: number; title: string; year: number | null; authors: string }[];
  authors: { label: string; documents: number }[];
  sources: { label: string; documents: number }[];
}

/** Title-bar search over the whole collection; 2 to 100 characters. */
export function quickSearch(query: string): Promise<QuickResults> {
  return call<QuickResults>("quick_search", { query });
}

/** Opens the native save dialog for a chart image; resolves to the file name, or null when cancelled. */
export function saveImage(kind: "svg" | "png", data: Uint8Array, name: string): Promise<string | null> {
  return call<string | null>("save_image", { kind, data: Array.from(data), name });
}

export function authorNames(): Promise<import("./entityTypes").AuthorNames> {
  return call("author_names");
}

/** Counts every name key in `keys` as `target`. */
export function mergeAuthors(keys: string[], target: string): Promise<number> {
  return call<number>("merge_authors", { keys, target });
}

export function splitAuthor(fromKey: string): Promise<number> {
  return call<number>("split_author", { fromKey });
}

export function keywordSynonyms(): Promise<import("./synonymTypes").KeywordSynonyms> {
  return call("keyword_synonyms");
}

/** Every keyword in `labels` counts as `target` in the analyses. */
export function mergeKeywords(labels: string[], target: string): Promise<number> {
  return call<number>("merge_keywords", { labels, target });
}

export function removeSynonym(fromKey: string): Promise<void> {
  return call<void>("remove_synonym", { fromKey });
}

export function testIdea(a: string, b: string): Promise<import("./ideaTypes").Idea> {
  return call("test_idea", { params: { a, b } });
}

/** Recent OpenAlex papers combining both concepts that are not in the collection. */
export function checkIdea(a: string, b: string): Promise<import("./ideaTypes").IdeaCheck> {
  return call("check_idea", { params: { a, b } });
}

export interface SavedIdea {
  key: string;
  a: string;
  b: string;
}

/** Saves the idea so Gap Explorer lists it; resolves to its gap key. */
export function saveIdea(a: string, b: string): Promise<string> {
  return call<string>("save_idea", { params: { a, b } });
}

export function savedIdeas(): Promise<SavedIdea[]> {
  return call<SavedIdea[]>("saved_ideas");
}

export function removeIdea(key: string): Promise<void> {
  return call<void>("remove_idea", { key });
}

export function workflowStatus(): Promise<import("./workflow").WorkflowStatus> {
  return call("workflow_status");
}

export function words(params: import("./wordTypes").WordParams): Promise<import("./wordTypes").Words> {
  return call<import("./wordTypes").Words>("words", { params });
}

export function rpys(params: import("./rpysTypes").RpysParams): Promise<import("./rpysTypes").Rpys> {
  return call<import("./rpysTypes").Rpys>("rpys", { params });
}

export function countryTable(): Promise<import("./entityTypes").CountryTable> {
  return call<import("./entityTypes").CountryTable>("country_table");
}

export function topRefs(top: number): Promise<import("./topRefTypes").TopRefs> {
  return call<import("./topRefTypes").TopRefs>("top_refs", { params: { top } });
}

export interface AddReport {
  requested: number;
  found: number;
  inserted: number;
  duplicates: number;
  notFound: string[];
}

/** Adds cited works by DOI, fetched from OpenAlex; at most 50 per call. */
export function addWorksByDoi(dois: string[]): Promise<AddReport> {
  return call<AddReport>("add_works_by_doi", { dois });
}

export function couplingMap(documents: number): Promise<import("./couplingMapTypes").CouplingMap> {
  return call<import("./couplingMapTypes").CouplingMap>("coupling_map", { params: { documents } });
}

export interface CitedEntities {
  references: number;
  sources: { label: string; documents: number }[];
  authors: { label: string; documents: number }[];
}

/** Most cited sources and first authors among the collection's references. */
export function citedEntities(): Promise<CitedEntities> {
  return call<CitedEntities>("cited_entities");
}
