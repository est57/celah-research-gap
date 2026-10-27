import type { ReactNode } from "react";

export interface YearColumn {
  label: string;
  cell: (row: number) => ReactNode;
  /** Hidden when the table is narrow. */
  wide?: boolean;
}

/** Newest year first; `years` and the columns' row index refer to the same order. */
export function YearTable({ years, columns, label }: { years: number[]; columns: YearColumn[]; label: string }) {
  const order = years.map((_, i) => i).reverse();
  return (
    <div className="year-table" role="table" aria-label={label}>
      <div className="year-row year-head" role="row" style={{ gridTemplateColumns: `64px repeat(${columns.length}, minmax(0, 1fr))` }}>
        <span role="columnheader">Tahun</span>
        {columns.map((c) => <span key={c.label} role="columnheader" className={c.wide ? "cell-right year-wide" : "cell-right"}>{c.label}</span>)}
      </div>
      {order.map((i) => (
        <div key={years[i]} className="year-row" role="row" style={{ gridTemplateColumns: `64px repeat(${columns.length}, minmax(0, 1fr))` }}>
          <span className="num" role="cell">{years[i]}</span>
          {columns.map((c) => <span key={c.label} role="cell" className={c.wide ? "num cell-right year-wide" : "num cell-right"}>{c.cell(i)}</span>)}
        </div>
      ))}
    </div>
  );
}
