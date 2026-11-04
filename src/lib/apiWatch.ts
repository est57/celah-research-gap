// Topic monitor: saved searches re-run to catch new papers.
import { call } from "./api";
import type { DocType, SourceId } from "./types";

export interface Watch {
  id: number;
  query: string;
  sources: SourceId[];
  docType: DocType | null;
  yearFrom: number;
  maxResults: number;
  /** UTC, "YYYY-MM-DD HH:MM:SS". */
  createdAt: string;
  lastRunAt: string | null;
  lastNew: number;
  totalNew: number;
}

export interface WatchHit {
  id: number;
  title: string;
  authors: string;
  year: number | null;
  sourceName: string | null;
  origin: string;
  foundAt: string;
}

export interface WatchRun {
  added: number;
  fetched: number;
  errors: string[];
}

export interface WatchInput {
  query: string;
  yearFrom: number;
  docType: DocType | null;
  maxResults: number;
  sources: SourceId[];
}

export function saveWatch(input: WatchInput): Promise<Watch> {
  return call<Watch>("save_watch", { input });
}

export function listWatches(): Promise<Watch[]> {
  return call<Watch[]>("list_watches");
}

export function deleteWatch(id: number): Promise<boolean> {
  return call<boolean>("delete_watch", { id });
}

export function watchHits(id: number): Promise<WatchHit[]> {
  return call<WatchHit[]>("watch_hits", { id });
}

export function runWatch(id: number): Promise<WatchRun> {
  return call<WatchRun>("run_watch", { id });
}
