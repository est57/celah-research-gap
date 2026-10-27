import { useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { useSourcesAuthors } from "../../hooks/useSourcesAuthors";
import { formatInt } from "../../lib/format";
import { AuthorTimeline } from "./AuthorTimeline";
import { BradfordChart } from "./BradfordChart";
import { ImpactTable } from "./ImpactTable";
import { LotkaChart } from "./LotkaChart";
import { TopSources } from "./TopSources";
import { SourcesTab } from "./SourcesTab";
import { AuthorsTab } from "./AuthorsTab";
import { AffiliationsTab } from "./AffiliationsTab";
import { CountriesTab } from "./CountriesTab";

export type SourcesTabId = "overview" | "sources" | "authors" | "affiliations" | "countries";
type TabId = SourcesTabId;

const TABS: TabDef<TabId>[] = [
  { id: "overview", label: "Ikhtisar" },
  { id: "sources", label: "Sumber" },
  { id: "authors", label: "Penulis" },
  { id: "affiliations", label: "Afiliasi" },
  { id: "countries", label: "Negara" },
];

function Panel({ title, aside, wide, children }: { title: string; aside?: string; wide?: boolean; children: ReactNode }) {
  return (
    <section className={wide ? "pnl sa-wide" : "pnl"}>
      <div className="ph">
        {title}
        {aside ? <span className="ph-aside">{aside}</span> : null}
      </div>
      <div className="pnl-body">{children}</div>
    </section>
  );
}

interface Props {
  version: number;
  initialTab?: TabId;
  /** Prefilled table search, from the title-bar search. */
  initialQuery?: string;
  /** Called after completing references here, so the app reloads its counts. */
  onDataChanged: () => void;
}

export function SourcesAuthorsScreen({ version, initialTab = "overview", initialQuery, onDataChanged }: Props) {
  const [tab, setTab] = useState<TabId>(initialTab);
  const { data, sample, error, loading } = useSourcesAuthors(version);

  let body: ReactNode;
  if (tab === "sources" || tab === "authors") {
    body = tab === "sources" ? <SourcesTab version={version} query={initialQuery} /> : <AuthorsTab version={version} query={initialQuery} onDataChanged={onDataChanged} />;
  } else if (tab === "countries") {
    body = <CountriesTab version={version} />;
  } else if (tab === "affiliations") {
    body = <AffiliationsTab version={version} onDataChanged={onDataChanged} />;
  } else if (error) {
    body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  } else if (!data) {
    body = <ScreenMessage>{loading ? "Menghitung analisis sumber dan penulis..." : ""}</ScreenMessage>;
  } else if (data.totalSources === 0 && data.lotka.totalAuthors === 0) {
    body = <EmptyCollection />;
  } else {
    body = (
      <div className="sa-page">
        {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
        <div className="sa-grid">
          <Panel title="Sumber paling relevan" aside={`${formatInt(data.totalSources)} sumber`}>
            <TopSources items={data.topSources} />
          </Panel>
          <Panel title="Hukum Bradford" aside={`${data.bradford.ranked.length} sumber teratas`}>
            <BradfordChart ranked={data.bradford.ranked} zones={data.bradford.zones} />
          </Panel>
          <Panel title="Hukum Lotka" aside="produktivitas penulis">
            <LotkaChart rows={data.lotka.rows} totalAuthors={data.lotka.totalAuthors} beta={data.lotka.beta} />
          </Panel>
          <Panel title="Produksi penulis dari waktu ke waktu" aside="ukuran = dokumen, warna = sitasi per tahun" wide>
            <AuthorTimeline rows={data.production} currentYear={data.currentYear} />
          </Panel>
          <Panel title="Dampak lokal penulis" aside="urut indeks h">
            <ImpactTable rows={data.impact} />
          </Panel>
        </div>
      </div>
    );
  }

  return (
    <>
      <Tabs label="Sumber dan penulis" tabs={TABS} active={tab} onChange={setTab} />
      {body}
    </>
  );
}
