import { docTypeLabel, formatInt } from "../../lib/format";
import { PAGE_SIZE } from "../../hooks/usePapers";
import type { PaperPage } from "../../lib/types";

interface Props {
  data: PaperPage;
  page: number;
  loading: boolean;
  error: string | null;
  onPage: (page: number) => void;
  selected: Set<number>;
  onSelect: (ids: number[], on: boolean) => void;
}

export function DocumentTable({ data, page, loading, error, onPage, selected, onSelect }: Props) {
  const pages = Math.max(1, Math.ceil(data.total / PAGE_SIZE));
  const start = page * PAGE_SIZE;
  const pageIds = data.items.map((p) => p.id);
  const allPicked = pageIds.length > 0 && pageIds.every((id) => selected.has(id));

  return (
    <>
      <div className="doc-table" role="table" aria-label="Dokumen dalam koleksi" aria-busy={loading}>
        <div className="doc-row doc-head" role="row">
          <label role="columnheader" className="doc-pick num">
            <input type="checkbox" checked={allPicked} disabled={pageIds.length === 0}
              aria-label="Pilih semua dokumen di halaman ini" onChange={(e) => onSelect(pageIds, e.target.checked)} />
            #
          </label>
          <span role="columnheader">Judul</span>
          <span role="columnheader" className="col-authors">Penulis</span>
          <span role="columnheader">Tahun</span>
          <span role="columnheader" className="col-source">Sumber</span>
          <span role="columnheader" className="col-type">Jenis</span>
          <span role="columnheader" className="cell-right">TC</span>
          <span role="columnheader" className="col-origin">Asal data</span>
        </div>
        {error ? <div className="empty" role="alert">{error}</div> : null}
        {!error && data.items.length === 0 ? (
          <div className="empty">
            <strong>Belum ada dokumen</strong>
            <span>Cari di OpenAlex di atas untuk mulai mengisi koleksi.</span>
          </div>
        ) : null}
        {data.items.map((p, i) => (
          <div className="doc-row" role="row" key={p.id} data-picked={selected.has(p.id)}>
            <label className="doc-pick num muted">
              <input type="checkbox" checked={selected.has(p.id)} aria-label={`Pilih ${p.title}`}
                onChange={(e) => onSelect([p.id], e.target.checked)} />
              {start + i + 1}
            </label>
            <span className="cell" title={p.title}>{p.title}</span>
            <span className="cell col-authors" style={{ color: "var(--ink-2)" }}>{p.authors || "-"}</span>
            <span className="num">{p.year ?? "-"}</span>
            <span className="cell cell-source col-source">{p.sourceName ?? "-"}</span>
            <span className="cell col-type">{docTypeLabel(p.docType)}</span>
            <span className="num cell-right">{formatInt(p.citedBy)}</span>
            <span className="cell col-origin muted">{p.origin}</span>
          </div>
        ))}
      </div>
      <div className="pager">
        <span className="muted">
          Halaman <span className="num">{page + 1}</span> dari <span className="num">{pages}</span>
        </span>
        <button type="button" className="btn" style={{ marginLeft: "auto" }}
          disabled={page === 0 || loading} onClick={() => onPage(page - 1)}>
          Sebelumnya
        </button>
        <button type="button" className="btn" disabled={page + 1 >= pages || loading} onClick={() => onPage(page + 1)}>
          Berikutnya
        </button>
      </div>
    </>
  );
}
