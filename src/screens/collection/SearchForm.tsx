import { useState, type FormEvent } from "react";
import { searchSources } from "../../lib/api";
import { formatInt } from "../../lib/format";
import type { DocType, SourceId, SourceReport } from "../../lib/types";
import { SourcePicker } from "./SourcePicker";

const THIS_YEAR = new Date().getFullYear();

export function SearchForm({ onImported }: { onImported: () => void }) {
  const [query, setQuery] = useState("");
  const [yearFrom, setYearFrom] = useState(2015);
  const [yearTo, setYearTo] = useState(THIS_YEAR);
  const [docType, setDocType] = useState<DocType | "">("");
  const [maxResults, setMaxResults] = useState(200);
  const [sources, setSources] = useState<SourceId[]>(["openalex", "crossref"]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [reports, setReports] = useState<SourceReport[]>([]);

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    const q = query.trim();
    if (q.length < 2) return setError("Kata kunci minimal 2 karakter.");
    if (yearFrom > yearTo) return setError("Tahun awal harus sebelum tahun akhir.");
    if (sources.length === 0) return setError("Pilih minimal satu sumber data.");
    setBusy(true);
    setError(null);
    setReports([]);
    try {
      const res = await searchSources({ query: q, yearFrom, yearTo, docType: docType || null, maxResults, sources });
      setReports(res);
      if (res.some((r) => r.inserted > 0)) onImported();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <form className="search-form" onSubmit={submit}>
      <label className="field field-grow">
        Kata kunci atau pertanyaan riset
        <input className="input" type="search" value={query} maxLength={300}
          placeholder="mis. tuberculosis deep learning screening" onChange={(e) => setQuery(e.target.value)} />
      </label>
      <label className="field">
        Dari tahun
        <input className="input num" type="number" min={1900} max={THIS_YEAR} value={yearFrom}
          style={{ width: 84 }} onChange={(e) => setYearFrom(Number(e.target.value))} />
      </label>
      <label className="field">
        Sampai
        <input className="input num" type="number" min={1900} max={THIS_YEAR} value={yearTo}
          style={{ width: 84 }} onChange={(e) => setYearTo(Number(e.target.value))} />
      </label>
      <label className="field">
        Jenis
        <select className="input" value={docType} onChange={(e) => setDocType(e.target.value as DocType | "")}>
          <option value="">Semua jenis</option>
          <option value="article">Artikel</option>
          <option value="review">Review</option>
          <option value="proceedings">Prosiding</option>
          <option value="preprint">Pra-cetak</option>
          <option value="book-chapter">Bab buku</option>
        </select>
      </label>
      <label className="field">
        Maks. per sumber
        <select className="input" value={maxResults} onChange={(e) => setMaxResults(Number(e.target.value))}>
          {[100, 200, 500, 1000].map((n) => (
            <option key={n} value={n}>{formatInt(n)}</option>
          ))}
        </select>
      </label>
      <SourcePicker value={sources} onChange={setSources} />
      <button type="submit" className="btn btn-primary" disabled={busy}>
        {busy ? "Mencari..." : "Cari"}
      </button>
      <div className="search-results" role="status">
        {busy ? <span className="message">Mengambil data dari {sources.length} sumber...</span> : null}
        {error ? <span className="message" data-kind="error">{error}</span> : null}
        {reports.map((r) => (
          <span key={r.source} className="message" data-kind={r.error ? "error" : "ok"}>
            <strong>{r.source}:</strong>{" "}
            {r.error ??
              `${formatInt(r.fetched)} ditemukan, ${formatInt(r.inserted)} baru, ${formatInt(r.duplicates)} duplikat dilewati.`}
          </span>
        ))}
      </div>
    </form>
  );
}
