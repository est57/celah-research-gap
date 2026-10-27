import type { CitedDoc, CitedSort } from "../../lib/citedTypes";
import { formatDecimal, formatInt } from "../../lib/format";

export function CitedTable({ docs, sort }: { docs: CitedDoc[]; sort: CitedSort }) {
  const value = (d: CitedDoc) => (sort === "local" ? d.local : sort === "normalized" ? d.normalized : d.global);
  const max = Math.max(...docs.map(value), 1e-9);
  return (
    <div className="cited-table" role="table" aria-label="Dokumen paling disitasi">
      <div className="cited-row cited-head" role="row">
        <span role="columnheader">#</span>
        <span role="columnheader">Dokumen</span>
        <span role="columnheader" className="cell-right" data-on={sort === "global"}>Global</span>
        <span role="columnheader" className="cell-right" data-on={sort === "local"}>Lokal</span>
        <span role="columnheader" className="cell-right cited-wide">Per tahun</span>
        <span role="columnheader" className="cell-right" data-on={sort === "normalized"}>Norm.</span>
      </div>
      {docs.map((d, i) => (
        <div key={d.id} className="cited-row" role="row">
          <span className="num muted" role="cell">{i + 1}</span>
          <div className="cited-doc" role="cell">
            <span className="cited-title">{d.title}</span>
            <span className="muted cited-meta">{[d.authors, d.year, d.source].filter(Boolean).join(" · ")}</span>
            <span className="cited-bar-track" aria-hidden="true"><span style={{ width: `${(value(d) / max) * 100}%` }} /></span>
          </div>
          <span className="num cell-right" role="cell">{formatInt(d.global)}</span>
          <span className="num cell-right" role="cell">{formatInt(d.local)}</span>
          <span className="num cell-right cited-wide" role="cell">{formatDecimal(d.perYear, 1)}</span>
          <span className="num cell-right" role="cell">{formatDecimal(d.normalized, 2)}</span>
        </div>
      ))}
    </div>
  );
}
