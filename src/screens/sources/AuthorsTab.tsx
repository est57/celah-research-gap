import type { ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useAuthorTable } from "../../hooks/useEntities";
import type { AuthorRow } from "../../lib/entityTypes";
import { formatDecimal, formatInt } from "../../lib/format";
import { EntityTable, type Column } from "./EntityTable";
import { AuthorNamesPanel } from "./AuthorNamesPanel";

const COLUMNS: Column<AuthorRow>[] = [
  { label: "Dok", cell: (r) => formatInt(r.documents), sort: (r) => r.documents },
  { label: "Terfraksi", cell: (r) => formatDecimal(r.fractional, 2), sort: (r) => r.fractional },
  { label: "Sitasi", cell: (r) => formatInt(r.citations), sort: (r) => r.citations },
  { label: "h", cell: (r) => formatInt(r.h), sort: (r) => r.h },
  { label: "g", cell: (r) => formatInt(r.g), sort: (r) => r.g, wide: true },
  { label: "m", cell: (r) => (r.m === null ? "-" : formatDecimal(r.m, 2)), sort: (r) => r.m ?? 0, wide: true },
  { label: "Aktif", cell: (r) => (r.firstYear === null ? "-" : r.firstYear === r.lastYear ? `${r.firstYear}` : `${r.firstYear}-${String(r.lastYear).slice(2)}`), wide: true },
];

export function AuthorsTab({ version, query, onDataChanged }: { version: number; query?: string; onDataChanged: () => void }) {
  const { data, sample, error, loading } = useAuthorTable(version);
  let body: ReactNode;
  if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? "Menghitung produktivitas penulis..." : ""}</ScreenMessage>;
  else if (data.length === 0) body = <EmptyCollection />;
  else {
    body = (
      <>
        <AuthorNamesPanel version={version} onChanged={onDataChanged} />
        <section className="pnl">
          <div className="ph">Semua penulis<span className="ph-aside">klik judul kolom untuk mengurutkan</span></div>
          <EntityTable id="aut" label="Penulis" rows={data} name={(r) => r.label} columns={COLUMNS} initialSort={0} initialQuery={query} />
          <p className="muted ent-note">Terfraksi: tiap dokumen dibagi rata ke semua penulisnya, sehingga penulis di tim besar tidak tampak lebih produktif dari yang sebenarnya. Ejaan nama yang berbeda antar sumber dihitung terpisah sampai disatukan di panel Satukan nama penulis.</p>
        </section>
      </>
    );
  }
  return (
    <div className="year-page">
      {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
      {body}
    </div>
  );
}
