import { useMemo, useState, type CSSProperties, type ReactNode } from "react";
import { formatInt } from "../../lib/format";
import { t } from "../../i18n";

export interface Column<T> {
  label: string;
  cell: (row: T) => ReactNode;
  /** Numeric value used when sorting by this column. */
  sort?: (row: T) => number;
  /** Hidden when the table is narrow. */
  wide?: boolean;
}

interface Props<T> {
  id: string;
  label: string;
  rows: T[];
  name: (row: T) => string;
  columns: Column<T>[];
  initialSort: number;
  pageSize?: number;
  initialQuery?: string;
}

/** Searchable, sortable, paged table; the first grid column is the name. `label` is translated by the caller, column labels here. */
export function EntityTable<T>({ id, label, rows, name, columns, initialSort, pageSize = 50, initialQuery = "" }: Props<T>) {
  const [query, setQuery] = useState(initialQuery);
  const [sortBy, setSortBy] = useState(initialSort);
  const [page, setPage] = useState(0);
  const shown = useMemo(() => {
    const q = query.trim().toLowerCase();
    const list = q ? rows.filter((r) => name(r).toLowerCase().includes(q)) : rows;
    const key = columns[sortBy]?.sort;
    return key ? [...list].sort((a, b) => key(b) - key(a) || name(a).localeCompare(name(b))) : list;
  }, [rows, query, sortBy, columns, name]);
  const pages = Math.max(1, Math.ceil(shown.length / pageSize));
  const current = Math.min(page, pages - 1);
  // Column counts for the full and the narrow layout (wide columns hidden), read by entities.css.
  const template = { "--cols": columns.length, "--cols-narrow": columns.filter((c) => !c.wide).length } as CSSProperties;

  return (
    <div className="ent">
      <div className="ent-bar">
        <label htmlFor={`${id}-q`} className="sr-only">{t("Cari {label}", { label: label.toLowerCase() })}</label>
        <input id={`${id}-q`} className="input" type="search" maxLength={100} placeholder={t("Cari {label}", { label: label.toLowerCase() })} value={query}
          onChange={(e) => { setQuery(e.target.value); setPage(0); }} />
        <span className="muted">{`${formatInt(shown.length)} ${label.toLowerCase()}`}</span>
      </div>
      <div className="ent-table" role="table" aria-label={label}>
        <div className="ent-row ent-head" role="row" style={template}>
          <span role="columnheader">#</span>
          <span role="columnheader">{label}</span>
          {columns.map((c, i) => (
            <span key={c.label} role="columnheader" className={c.wide ? "cell-right ent-wide" : "cell-right"}>
              {c.sort ? (
                <button type="button" className="ent-sort" aria-pressed={sortBy === i} onClick={() => { setSortBy(i); setPage(0); }}>{t(c.label)}</button>
              ) : t(c.label)}
            </span>
          ))}
        </div>
        {shown.slice(current * pageSize, (current + 1) * pageSize).map((r, i) => (
          <div key={`${name(r)}-${i}`} className="ent-row" role="row" style={template}>
            <span className="num muted" role="cell">{current * pageSize + i + 1}</span>
            <span className="ent-name" role="cell" title={name(r)}>{name(r)}</span>
            {columns.map((c) => <span key={c.label} role="cell" className={c.wide ? "num cell-right ent-wide" : "num cell-right"}>{c.cell(r)}</span>)}
          </div>
        ))}
        {shown.length === 0 ? <p className="muted ent-empty">{t("Tidak ada yang cocok.")}</p> : null}
      </div>
      {pages > 1 ? (
        <div className="pager">
          <button type="button" className="btn" disabled={current === 0} onClick={() => setPage(current - 1)}>{t("Sebelumnya")}</button>
          <span className="num muted">{`${current + 1} / ${pages}`}</span>
          <button type="button" className="btn" disabled={current >= pages - 1} onClick={() => setPage(current + 1)}>{t("Berikutnya")}</button>
        </div>
      ) : null}
    </div>
  );
}
