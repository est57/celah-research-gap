import { docTypeLabel, formatInt } from "../../lib/format";
import { PAGE_SIZE } from "../../hooks/usePapers";
import type { PaperPage } from "../../lib/types";
import { NumText } from "./NumText";
import { t } from "../../i18n";

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
      <div className="doc-table" role="table" aria-label={t("Dokumen dalam koleksi")} aria-busy={loading}>
        <div className="doc-row doc-head" role="row">
          <label role="columnheader" className="doc-pick num">
            <input type="checkbox" checked={allPicked} disabled={pageIds.length === 0}
              aria-label={t("Pilih semua dokumen di halaman ini")} onChange={(e) => onSelect(pageIds, e.target.checked)} />
            #
          </label>
          <span role="columnheader">{t("Judul")}</span>
          <span role="columnheader" className="col-authors">{t("Penulis")}</span>
          <span role="columnheader">{t("Tahun")}</span>
          <span role="columnheader" className="col-source">{t("Sumber")}</span>
          <span role="columnheader" className="col-type">{t("Jenis")}</span>
          <span role="columnheader" className="cell-right">TC</span>
          <span role="columnheader" className="col-origin">{t("Asal data")}</span>
        </div>
        {error ? <div className="empty" role="alert">{error}</div> : null}
        {!error && data.items.length === 0 ? (
          <div className="empty">
            <strong>{t("Belum ada dokumen")}</strong>
            <span>{t("Cari di OpenAlex di atas untuk mulai mengisi koleksi.")}</span>
          </div>
        ) : null}
        {data.items.map((p, i) => (
          <div className="doc-row" role="row" key={p.id} data-picked={selected.has(p.id)}>
            <label className="doc-pick num muted">
              <input type="checkbox" checked={selected.has(p.id)} aria-label={t("Pilih {title}", { title: p.title })}
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
          <NumText text={t("Halaman {page} dari {pages}")} vars={{ page: page + 1, pages }} />
        </span>
        <button type="button" className="btn" style={{ marginLeft: "auto" }}
          disabled={page === 0 || loading} onClick={() => onPage(page - 1)}>
          {t("Sebelumnya")}
        </button>
        <button type="button" className="btn" disabled={page + 1 >= pages || loading} onClick={() => onPage(page + 1)}>
          {t("Berikutnya")}
        </button>
      </div>
    </>
  );
}
