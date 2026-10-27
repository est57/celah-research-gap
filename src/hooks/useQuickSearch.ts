import { useEffect, useState } from "react";
import { isDesktop } from "../lib/api";
import { quickSearch, type QuickResults } from "../lib/apiEntities";
import { matchCommands, type Command, type Jump } from "../lib/commands";

export interface SearchItem {
  key: string;
  group: string;
  label: string;
  sub: string;
  jump: Jump;
}

const fromCommand = (c: Command): SearchItem => ({ key: `cmd-${c.label}`, group: "Layar", label: c.label, sub: c.place, jump: c.jump });

function fromResults(r: QuickResults): SearchItem[] {
  return [
    ...r.documents.map((d) => ({ key: `doc-${d.id}`, group: "Dokumen", label: d.title, sub: [d.authors, d.year].filter(Boolean).join(" · "),
      jump: { screen: "collection" as const, tab: "collection", query: d.title } })),
    ...r.authors.map((a) => ({ key: `aut-${a.label}`, group: "Penulis", label: a.label, sub: `${a.documents} dokumen`,
      jump: { screen: "authors" as const, tab: "authors", query: a.label } })),
    ...r.sources.map((s) => ({ key: `src-${s.label}`, group: "Sumber", label: s.label, sub: `${s.documents} dokumen`,
      jump: { screen: "sources" as const, tab: "sources", query: s.label } })),
  ];
}

/** Screen commands at once; collection matches shortly after typing stops. */
export function useQuickSearch(query: string) {
  const [found, setFound] = useState<SearchItem[]>([]);
  const text = query.trim();
  useEffect(() => {
    setFound([]);
    if (!isDesktop() || text.length < 2) return;
    let cancelled = false;
    const timer = window.setTimeout(() => {
      quickSearch(text)
        .then((r) => !cancelled && setFound(fromResults(r)))
        .catch(() => !cancelled && setFound([]));
    }, 200);
    return () => {
      cancelled = true;
      window.clearTimeout(timer);
    };
  }, [text]);
  return [...matchCommands(text).map(fromCommand), ...found];
}
