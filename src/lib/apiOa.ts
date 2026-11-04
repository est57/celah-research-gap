// Calls for open-access copies (Unpaywall) and opening documents in the browser.
import { call } from "./api";

export interface OaStatus {
  withDoi: number;
  checked: number;
  open: number;
}

export interface OaReport {
  checked: number;
  found: number;
  remaining: number;
  error: string | null;
}

export interface DocLinks {
  id: number;
  open: boolean;
  pdf: boolean;
  doi: boolean;
}

export function openAccessStatus(): Promise<OaStatus> {
  return call<OaStatus>("open_access_status");
}

export function findOpenAccess(): Promise<OaReport> {
  return call<OaReport>("find_open_access");
}

export function documentLinks(ids: number[]): Promise<DocLinks[]> {
  return call<DocLinks[]>("document_links", { ids });
}

/** Opens the PDF, open page or DOI page; the backend picks the address from the database. */
export function openDocument(id: number): Promise<void> {
  return call<void>("open_document", { id });
}
