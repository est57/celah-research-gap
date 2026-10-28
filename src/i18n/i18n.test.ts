import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { EN } from "./en";

// Every key in t("...") or tk("...") must have an English text, and visible text in components must
// go through t(). Names that read the same in both languages are allowed as they are.
const ROOT = join(__dirname, "..");
const SAME_IN_BOTH = /^(Celah|OpenAlex|Scopus|Web of Science|WoS|PubMed|Crossref|arXiv|Europe PMC|Semantic Scholar|NCBI|SJR|APA 7|IEEE|BibTeX|RIS|CSV|PNG|SVG|DOI|ISSN|SQLite|MeSH|Cmd K|English|Indonesia|Q[1-4]|TC|PY|DE|ID|AB|CR|C1|SO|AU|TI|DT|-|#|%|v[\d.]+)$/;

// Matches between ">" and "<" that are TypeScript (generics such as "=> Promise<void>",
// comparisons, calls), not JSX text.
const CODE = /^Promise$|[;=[\]{}]|\s\?\s\(|&&|\|\||\?\?|\?\.|[A-Za-z_]\(/;

function files(dir: string): string[] {
  return readdirSync(dir).flatMap((name) => {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) return name === "dev" || name === "i18n" ? [] : files(path);
    return /\.tsx?$/.test(name) && !name.endsWith(".test.ts") ? [path] : [];
  });
}

const sources = files(ROOT).map((path) => ({ path: path.slice(ROOT.length + 1), text: readFileSync(path, "utf8") }));
const placeholders = (s: string) => [...s.matchAll(/\{(\w+)\}/g)].map((m) => m[1]).sort();

describe("English interface texts", () => {
  const keys = new Set<string>();
  for (const { text } of sources) {
    for (const m of text.matchAll(/\bt[k]?\(\s*("(?:[^"\\]|\\.)*")/g)) keys.add(JSON.parse(m[1]) as string);
  }

  it("has an English text for every key, with the same placeholders", () => {
    const missing = [...keys].filter((k) => !(k in EN));
    expect(missing, `tanpa terjemahan:\n${missing.join("\n")}`).toEqual([]);
    const wrong = [...keys].filter((k) => k in EN && placeholders(k).join() !== placeholders(EN[k]).join());
    expect(wrong, `placeholder berbeda:\n${wrong.join("\n")}`).toEqual([]);
  });

  it("keeps no unused entries", () => {
    const unused = Object.keys(EN).filter((k) => !keys.has(k));
    expect(unused, `tidak dipakai:\n${unused.join("\n")}`).toEqual([]);
  });

  it("leaves no visible text outside t() in components", () => {
    const loose: string[] = [];
    for (const { path, text } of sources.filter((s) => s.path.endsWith(".tsx"))) {
      const jsxText = [...text.matchAll(/>([^<>{}]*[A-Za-z]{2,}[^<>{}]*)</g)].map((m) => m[1].trim());
      const attrs = [...text.matchAll(/\b(?:aria-label|title|placeholder|alt)="([^"]*[A-Za-z]{2,}[^"]*)"/g)].map((m) => m[1].trim());
      for (const s of [...jsxText, ...attrs]) if (s && !SAME_IN_BOTH.test(s) && !CODE.test(s)) loose.push(`${path}: ${s}`);
    }
    expect(loose, `teks belum lewat t():\n${loose.join("\n")}`).toEqual([]);
  });
});
