import { useEffect, useState } from "react";
import { isDesktop } from "../lib/api";
import { documentLinks, type DocLinks } from "../lib/apiOa";

/** What can be opened for each document id (PDF, open page, DOI page), keyed by id. */
export function useDocLinks(ids: number[]): Map<number, DocLinks> {
  const [links, setLinks] = useState<Map<number, DocLinks>>(new Map());
  const key = ids.join(",");
  useEffect(() => {
    let cancelled = false;
    const list = key ? key.split(",").map(Number) : [];
    if (!isDesktop()) {
      // The preview shows the buttons with made-up availability so the layout can be checked.
      if (import.meta.env.DEV) setLinks(new Map(list.map((id) => [id, { id, open: id % 2 === 0, pdf: id % 2 === 0, doi: true }])));
      return;
    }
    if (!list.length) return setLinks(new Map());
    documentLinks(list)
      .then((rows) => !cancelled && setLinks(new Map(rows.map((r) => [r.id, r]))))
      .catch(() => !cancelled && setLinks(new Map()));
    return () => {
      cancelled = true;
    };
  }, [key]);
  return links;
}
