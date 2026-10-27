import type { ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useSourcesDetail } from "../../hooks/useEntities";
import type { SourceRow } from "../../lib/entityTypes";
import { formatDecimal, formatInt } from "../../lib/format";
import { DynamicsChart } from "./DynamicsChart";
import { EntityTable, type Column } from "./EntityTable";
import { ChartExport } from "../../components/ChartExport";

const COLUMNS: Column<SourceRow>[] = [
  { label: "Zona", cell: (r) => <span className="zone-chip" data-zone={r.zone}>{r.zone}</span> },
  { label: "h", cell: (r) => formatInt(r.h), sort: (r) => r.h },
  { label: "g", cell: (r) => formatInt(r.g), sort: (r) => r.g, wide: true },
  { label: "m", cell: (r) => (r.m === null ? "-" : formatDecimal(r.m, 2)), sort: (r) => r.m ?? 0, wide: true },
  { label: "Sitasi", cell: (r) => formatInt(r.citations), sort: (r) => r.citations },
  { label: "Dok", cell: (r) => formatInt(r.documents), sort: (r) => r.documents },
  { label: "Mulai", cell: (r) => r.firstYear ?? "-", wide: true },
];

export function SourcesTab({ version, query }: { version: number; query?: string }) {
  const { data, sample, error, loading } = useSourcesDetail(version);
  let body: ReactNode;
  if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? "Menghitung dampak sumber..." : ""}</ScreenMessage>;
  else if (data.sources.length === 0) body = <EmptyCollection />;
  else {
    body = (
      <>
        <section className="pnl">
          <div className="ph">Dinamika sumber<span className="ph-aside">dokumen kumulatif, sumber teratas</span><ChartExport name="Dinamika sumber" /></div>
          <div className="pnl-body"><DynamicsChart data={data} /></div>
        </section>
        <section className="pnl">
          <div className="ph">Dampak lokal sumber<span className="ph-aside">klik judul kolom untuk mengurutkan</span></div>
          <EntityTable id="src" label="Sumber" rows={data.sources} name={(r) => r.label} columns={COLUMNS} initialSort={1} initialQuery={query} />
          <p className="muted ent-note">Zona Bradford: 1 = inti (sepertiga dokumen dari sedikit sumber), 2 dan 3 makin tersebar. Indeks h, g, dan m dihitung dari sitasi dokumen koleksi di sumber itu.</p>
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
