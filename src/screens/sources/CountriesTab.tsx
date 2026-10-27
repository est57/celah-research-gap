import type { ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { ChartExport } from "../../components/ChartExport";
import { useCountryTable } from "../../hooks/useEntities";
import { countryName } from "../../lib/countryNames";
import type { CountryRow } from "../../lib/entityTypes";
import { formatDecimal, formatInt, formatPct } from "../../lib/format";
import { DynamicsChart } from "./DynamicsChart";
import { EntityTable, type Column } from "./EntityTable";

const COLUMNS: Column<CountryRow>[] = [
  { label: "Dok", cell: (r) => formatInt(r.documents), sort: (r) => r.documents },
  { label: "Sitasi", cell: (r) => formatInt(r.citations), sort: (r) => r.citations },
  { label: "Rata-rata", cell: (r) => formatDecimal(r.avgCitations), sort: (r) => r.avgCitations },
  { label: "Kores.", cell: (r) => formatInt(r.corresponding), sort: (r) => r.corresponding },
  { label: "MCP", cell: (r) => (r.corresponding ? formatPct(r.mcp, r.corresponding) : "-"), sort: (r) => (r.corresponding ? r.mcp / r.corresponding : 0) },
  { label: "Mulai", cell: (r) => r.firstYear ?? "-", wide: true },
];

/** Production, impact and collaboration per country, with the top countries over time. */
export function CountriesTab({ version }: { version: number }) {
  const { data, sample, error, loading } = useCountryTable(version);
  let body: ReactNode;
  if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? "Menghitung produksi negara..." : ""}</ScreenMessage>;
  else if (data.documents === 0) body = <EmptyCollection />;
  else if (data.withCountry === 0) {
    body = (
      <ScreenMessage>
        <strong>Belum ada data negara</strong>
        <span className="muted">Negara diambil dari afiliasi penulis. Lengkapi afiliasi di tab Afiliasi, atau impor ekspor Scopus/WoS yang memuat alamat.</span>
      </ScreenMessage>
    );
  } else {
    const named = { years: data.years, dynamics: data.dynamics.map((s) => ({ ...s, label: countryName(s.label) })) };
    body = (
      <>
        <section className="pnl">
          <div className="ph">Produksi negara dari waktu ke waktu
            <span className="ph-aside">{`dokumen kumulatif · ${formatInt(data.withCountry)} dari ${formatInt(data.documents)} dokumen punya negara`}</span>
            <ChartExport name="Produksi negara dari waktu ke waktu" />
          </div>
          <div className="pnl-body"><DynamicsChart data={named} label="negara" /></div>
        </section>
        <section className="pnl">
          <div className="ph">Negara paling produktif dan paling disitasi<span className="ph-aside">urutkan dengan klik judul kolom</span></div>
          <EntityTable id="cty" label="Negara" rows={data.rows} name={(r) => countryName(r.code)} columns={COLUMNS} initialSort={0} />
          <p className="muted ent-note">
            Dok dan sitasi menghitung dokumen dengan minimal satu penulis dari negara itu. Kores. (dokumen dengan penulis korespondensi dari negara itu) dan MCP (publikasi
            multi-negara) memakai negara penulis korespondensi, seperti bibliometrix. MCP rendah berarti negara itu jarang
            memimpin kolaborasi internasional.
          </p>
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
