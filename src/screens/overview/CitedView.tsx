import { useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useCited } from "../../hooks/useCited";
import type { CitedSort } from "../../lib/citedTypes";
import { formatDecimal, formatInt } from "../../lib/format";
import { CitedTable } from "./CitedTable";

const SORTS: [CitedSort, string][] = [["global", "Sitasi global"], ["local", "Sitasi lokal"], ["normalized", "Ternormalisasi per tahun"]];
const LIMITS = [20, 50, 100];

const HELP: Record<CitedSort, string> = {
  global: "Jumlah sitasi menurut sumber data (OpenAlex, Scopus, dan lainnya).",
  local: "Jumlah sitasi dari dokumen di koleksi Anda. Dokumen yang tinggi di sini tetapi rendah secara global penting khusus untuk bidang Anda.",
  normalized: "Sitasi global dibagi rata-rata dokumen koleksi yang terbit di tahun yang sama; 1,0 berarti rata-rata. Paper baru tidak kalah hanya karena umurnya.",
};

export function CitedView({ version }: { version: number }) {
  const [sort, setSort] = useState<CitedSort>("global");
  const [limit, setLimit] = useState(20);
  const { data, sample, error, loading } = useCited(version, sort, limit);

  let body: ReactNode;
  if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? "Menghitung sitasi..." : ""}</ScreenMessage>;
  else if (data.documents === 0) body = <EmptyCollection />;
  else body = <CitedTable docs={data.docs} sort={sort} />;

  return (
    <div className="cited-page">
      {sample ? <div className="notice">Pratinjau browser: dokumen di halaman ini adalah data contoh.</div> : null}
      <div className="cited-bar">
        <div className="opt-seg" role="group" aria-label="Urutkan menurut">
          {SORTS.map(([s, label]) => <button key={s} type="button" aria-pressed={sort === s} onClick={() => setSort(s)}>{label}</button>)}
        </div>
        <label htmlFor="cited-limit" className="muted">Tampilkan</label>
        <select id="cited-limit" className="input" value={limit} onChange={(e) => setLimit(Number(e.target.value))}>
          {LIMITS.map((n) => <option key={n} value={n}>{n}</option>)}
        </select>
        {data ? <span className="muted cited-count">{`dari ${formatInt(data.documents)} dokumen`}</span> : null}
      </div>
      <p className="muted cited-help">{HELP[sort]}</p>
      {data && data.localLinks === 0 ? (
        <div className="notice">
          Sitasi lokal belum tersedia karena koleksi belum punya daftar rujukan. Buka Intelektual, tab Historiograf, lalu
          jalankan Lengkapi rujukan & institusi dari OpenAlex.
        </div>
      ) : null}
      {data && data.localLinks > 0 && sort === "local" ? (
        <p className="muted cited-help">{`${formatInt(data.localLinks)} sitasi antar dokumen koleksi ditemukan; rata-rata ${formatDecimal(data.localLinks / Math.max(1, data.documents), 1)} per dokumen.`}</p>
      ) : null}
      <section className="pnl">{body}</section>
    </div>
  );
}
