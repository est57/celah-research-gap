import { useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage as Message } from "../../components/ScreenMessage";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { useOverview } from "../../hooks/useOverview";
import { formatDecimal } from "../../lib/format";
import { AnnualChart } from "./AnnualChart";
import { CitationChart } from "./CitationChart";
import { MainInfoPanel } from "./MainInfoPanel";
import { ThreeFieldPlot } from "./ThreeFieldPlot";
import { CitedView } from "./CitedView";
import { AnnualView } from "./AnnualView";
import { CitationsView } from "./CitationsView";
import { ThreeFieldView } from "./ThreeFieldView";
import { ChartExport } from "../../components/ChartExport";

export type OverviewTab = "main" | "annual" | "citations" | "threefield" | "cited";
type TabId = OverviewTab;

const TABS: TabDef<TabId>[] = [
  { id: "main", label: "Informasi utama" },
  { id: "annual", label: "Produksi tahunan" },
  { id: "citations", label: "Sitasi per tahun" },
  { id: "threefield", label: "Plot tiga bidang" },
  { id: "cited", label: "Dokumen paling disitasi" },
];

/** `version` changes whenever the collection changes, which triggers a reload. */
export function OverviewScreen({ version, initialTab = "main" }: { version: number; initialTab?: TabId }) {
  const [tab, setTab] = useState<TabId>(initialTab);
  const { data, sample, error, loading } = useOverview(version);

  let body: ReactNode;
  if (tab === "cited") {
    body = <CitedView version={version} />;
  } else if (tab === "threefield") {
    body = <ThreeFieldView version={version} />;
  } else if (error) {
    body = <Message><span role="alert">{error}</span></Message>;
  } else if (!data) {
    body = <Message>{loading ? "Menghitung ringkasan..." : ""}</Message>;
  } else if (data.info.documents === 0) {
    body = <EmptyCollection />;
  } else if (tab === "annual" || tab === "citations") {
    body = tab === "annual" ? <AnnualView data={data} sample={sample} /> : <CitationsView data={data} sample={sample} />;
  } else {
    body = (
      <div className="overview">
        <MainInfoPanel info={data.info} />
        <div className="overview-charts">
          {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
          <div className="overview-row">
            <section className="pnl pnl-grow">
              <div className="ph">
                Produksi ilmiah tahunan
                {data.info.growthRate !== null ? (
                  <span className="ph-aside">
                    Laju pertumbuhan tahunan <b className="num">{formatDecimal(data.info.growthRate)}%</b>
                  </span>
                ) : null}
              </div>
              <AnnualChart points={data.annual} />
            </section>
            <section className="pnl pnl-side">
              <div className="ph">Rata-rata sitasi per tahun<ChartExport name="Rata-rata sitasi per tahun" /></div>
              <div className="pnl-body">
                <CitationChart points={data.annual} />
              </div>
            </section>
          </div>
          <section className="pnl">
            <div className="ph">
              Plot tiga bidang
              <span className="ph-aside">penulis, kata kunci, dan sumber teratas</span>
              <ChartExport name="Plot tiga bidang" />
            </div>
            <div className="pnl-body">
              <ThreeFieldPlot data={data.threeField} />
            </div>
          </section>
        </div>
      </div>
    );
  }

  return (
    <>
      <Tabs label="Ringkasan" tabs={TABS} active={tab} onChange={setTab} />
      {body}
    </>
  );
}
