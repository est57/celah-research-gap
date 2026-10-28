import { useState, type FormEvent } from "react";
import { searchSources } from "../../lib/api";
import { formatInt } from "../../lib/format";
import type { DocType, SourceId, SourceReport } from "../../lib/types";
import { SourcePicker } from "./SourcePicker";
import { t } from "../../i18n";

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
    if (q.length < 2) return setError(t("Kata kunci minimal 2 karakter."));
    if (yearFrom > yearTo) return setError(t("Tahun awal harus sebelum tahun akhir."));
    if (sources.length === 0) return setError(t("Pilih minimal satu sumber data."));
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
        {t("Kata kunci atau pertanyaan riset")}
        <input className="input" type="search" value={query} maxLength={300}
          placeholder={t("mis. tuberculosis deep learning screening")} onChange={(e) => setQuery(e.target.value)} />
      </label>
      <label className="field">
        {t("Dari tahun")}
        <input className="input num" type="number" min={1900} max={THIS_YEAR} value={yearFrom}
          style={{ width: 84 }} onChange={(e) => setYearFrom(Number(e.target.value))} />
      </label>
      <label className="field">
        {t("Sampai")}
        <input className="input num" type="number" min={1900} max={THIS_YEAR} value={yearTo}
          style={{ width: 84 }} onChange={(e) => setYearTo(Number(e.target.value))} />
      </label>
      <label className="field">
        {t("Jenis")}
        <select className="input" value={docType} onChange={(e) => setDocType(e.target.value as DocType | "")}>
          <option value="">{t("Semua jenis")}</option>
          <option value="article">{t("Artikel")}</option>
          <option value="review">{t("Review")}</option>
          <option value="proceedings">{t("Prosiding")}</option>
          <option value="preprint">{t("Pra-cetak")}</option>
          <option value="book-chapter">{t("Bab buku")}</option>
        </select>
      </label>
      <label className="field">
        {t("Maks. per sumber")}
        <select className="input" value={maxResults} onChange={(e) => setMaxResults(Number(e.target.value))}>
          {[100, 200, 500, 1000].map((n) => (
            <option key={n} value={n}>{formatInt(n)}</option>
          ))}
        </select>
      </label>
      <SourcePicker value={sources} onChange={setSources} />
      <button type="submit" className="btn btn-primary" disabled={busy}>
        {busy ? t("Mencari...") : t("Cari")}
      </button>
      <div className="search-results" role="status">
        {busy ? <span className="message">{t("Mengambil data dari {n} sumber...", { n: sources.length })}</span> : null}
        {error ? <span className="message" data-kind="error">{error}</span> : null}
        {reports.map((r) => (
          <span key={r.source} className="message" data-kind={r.error ? "error" : "ok"}>
            <strong>{r.source}:</strong>{" "}
            {r.error ??
              t("{fetched} ditemukan, {inserted} baru, {duplicates} duplikat dilewati.",
                { fetched: formatInt(r.fetched), inserted: formatInt(r.inserted), duplicates: formatInt(r.duplicates) })}
          </span>
        ))}
      </div>
    </form>
  );
}
