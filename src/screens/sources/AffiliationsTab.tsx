import type { ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useAffiliations } from "../../hooks/useEntities";
import { countryName } from "../../lib/countryNames";
import type { AffiliationRow } from "../../lib/entityTypes";
import { formatInt, formatPct } from "../../lib/format";
import { CompleteRefs } from "../structure/CompleteRefs";
import { DynamicsChart } from "./DynamicsChart";
import { EntityTable, type Column } from "./EntityTable";
import { ChartExport } from "../../components/ChartExport";

const COLUMNS: Column<AffiliationRow>[] = [
  { label: "Negara", cell: (r) => <span className="ent-country">{r.country ? countryName(r.country) : "-"}</span>, wide: true },
  { label: "Dok", cell: (r) => formatInt(r.documents), sort: (r) => r.documents },
  { label: "Sitasi", cell: (r) => formatInt(r.citations), sort: (r) => r.citations },
  { label: "Mulai", cell: (r) => r.firstYear ?? "-", wide: true },
];

export function AffiliationsTab({ version, onDataChanged }: { version: number; onDataChanged: () => void }) {
  const { data, sample, error, loading } = useAffiliations(version);
  let body: ReactNode;
  if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? "Menghitung afiliasi..." : ""}</ScreenMessage>;
  else if (data.documents === 0) body = <EmptyCollection />;
  else {
    const coverage = data.withAffiliation / data.documents;
    body = (
      <>
        <section className="pnl">
          <div className="ph">Cakupan data institusi<span className="ph-aside">{`${formatInt(data.withAffiliation)} dari ${formatInt(data.documents)} dokumen (${formatPct(data.withAffiliation, data.documents)})`}</span></div>
          {coverage < 0.9 ? (
            <p className="muted ent-note">Dokumen lama belum menyimpan nama institusi. Lengkapi dari OpenAlex untuk dokumen yang punya ID OpenAlex atau DOI; ekspor Scopus/WoS dan pencarian baru sudah menyimpannya.</p>
          ) : null}
          {coverage < 0.9 ? <CompleteRefs version={version} onDone={onDataChanged} /> : null}
        </section>
        {data.rows.length ? (
          <>
            <section className="pnl">
              <div className="ph">Dinamika afiliasi<span className="ph-aside">dokumen kumulatif, institusi teratas</span><ChartExport name="Dinamika afiliasi" /></div>
              <div className="pnl-body"><DynamicsChart data={data} label="institusi" /></div>
            </section>
            <section className="pnl">
              <div className="ph">Afiliasi paling produktif<span className="ph-aside">tiap dokumen dihitung sekali per institusi</span></div>
              <EntityTable id="aff" label="Institusi" rows={data.rows} name={(r) => r.label} columns={COLUMNS} initialSort={1} />
              <p className="muted ent-note">Ejaan yang berbeda antar sumber digabung bila sama setelah singkatan dibaca penuh (Univ Malaya = University of Malaya). Nama yang benar-benar berbeda tetap terpisah.</p>
            </section>
          </>
        ) : null}
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
