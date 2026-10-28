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
import { t, tk } from "../../i18n";

const COLUMNS: Column<AffiliationRow>[] = [
  { label: tk("Negara"), cell: (r) => <span className="ent-country">{r.country ? countryName(r.country) : "-"}</span>, wide: true },
  { label: tk("Dok"), cell: (r) => formatInt(r.documents), sort: (r) => r.documents },
  { label: tk("Sitasi"), cell: (r) => formatInt(r.citations), sort: (r) => r.citations },
  { label: tk("Mulai"), cell: (r) => r.firstYear ?? "-", wide: true },
];

export function AffiliationsTab({ version, onDataChanged }: { version: number; onDataChanged: () => void }) {
  const { data, sample, error, loading } = useAffiliations(version);
  let body: ReactNode;
  if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? t("Menghitung afiliasi...") : ""}</ScreenMessage>;
  else if (data.documents === 0) body = <EmptyCollection />;
  else {
    const coverage = data.withAffiliation / data.documents;
    body = (
      <>
        <section className="pnl">
          <div className="ph">{t("Cakupan data institusi")}<span className="ph-aside">{t("{n} dari {total} dokumen ({pct})", { n: formatInt(data.withAffiliation), total: formatInt(data.documents), pct: formatPct(data.withAffiliation, data.documents) })}</span></div>
          {coverage < 0.9 ? (
            <p className="muted ent-note">{t("Dokumen lama belum menyimpan nama institusi. Lengkapi dari OpenAlex untuk dokumen yang punya ID OpenAlex atau DOI; ekspor Scopus/WoS dan pencarian baru sudah menyimpannya.")}</p>
          ) : null}
          {coverage < 0.9 ? <CompleteRefs version={version} onDone={onDataChanged} /> : null}
        </section>
        {data.rows.length ? (
          <>
            <section className="pnl">
              <div className="ph">{t("Dinamika afiliasi")}<span className="ph-aside">{t("dokumen kumulatif, institusi teratas")}</span><ChartExport name={t("Dinamika afiliasi")} /></div>
              <div className="pnl-body"><DynamicsChart data={data} label={t("institusi")} /></div>
            </section>
            <section className="pnl">
              <div className="ph">{t("Afiliasi paling produktif")}<span className="ph-aside">{t("tiap dokumen dihitung sekali per institusi")}</span></div>
              <EntityTable id="aff" label={t("Institusi")} rows={data.rows} name={(r) => r.label} columns={COLUMNS} initialSort={1} />
              <p className="muted ent-note">{t("Ejaan yang berbeda antar sumber digabung bila sama setelah singkatan dibaca penuh (Univ Malaya = University of Malaya). Nama yang benar-benar berbeda tetap terpisah.")}</p>
            </section>
          </>
        ) : null}
      </>
    );
  }
  return (
    <div className="year-page">
      {sample ? <div className="notice">{t("Pratinjau browser: angka di halaman ini adalah data contoh.")}</div> : null}
      {body}
    </div>
  );
}
