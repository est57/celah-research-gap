import { useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { useConceptual } from "../../hooks/useConceptual";
import { formatInt } from "../../lib/format";
import { DEFAULT_PARAMS, type ConceptualParams } from "../../lib/conceptualTypes";
import { ParamsForm } from "./ParamsForm";
import { ThematicMap } from "./ThematicMap";
import { ThemeDetail } from "./ThemeDetail";
import { TrendTopics } from "./TrendTopics";
import { NetworkView } from "../structure/NetworkView";
import { EvolutionView } from "./EvolutionView";
import { FactorialView } from "./FactorialView";
import { SynonymsView } from "./SynonymsView";
import { WordsView } from "./WordsView";

export type ConceptualTab = "map" | "words" | "network" | "evolution" | "factorial" | "synonyms";
type TabId = ConceptualTab;

const TABS: TabDef<TabId>[] = [
  { id: "map", label: "Peta tematik & tren" },
  { id: "words", label: "Analisis kata" },
  { id: "network", label: "Jaringan ko-kemunculan" },
  { id: "evolution", label: "Evolusi tematik" },
  { id: "factorial", label: "Analisis faktorial" },
  { id: "synonyms", label: "Sinonim kata kunci" },
];

interface Props {
  version: number;
  initialTab?: TabId;
  /** Called after synonyms change, so the app reloads its analyses. */
  onDataChanged: () => void;
}

export function ConceptualScreen({ version, initialTab = "map", onDataChanged }: Props) {
  const [tab, setTab] = useState<TabId>(initialTab);
  const [params, setParams] = useState<ConceptualParams>(DEFAULT_PARAMS);
  const [selected, setSelected] = useState<number | null>(null);
  const { data, sample, error, loading } = useConceptual(version, params);

  const themes = data?.map.themes ?? [];
  const current = themes.find((t) => t.id === selected) ?? themes.find((t) => t.quadrant === "emerging") ?? themes[0];

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? "Menyusun peta tematik..." : ""}</ScreenMessage>;
  else if (data.documents === 0) main = <EmptyCollection />;
  else if (themes.length === 0) {
    main = <ScreenMessage>Belum ada tema yang memenuhi syarat. Coba turunkan frekuensi minimum tema, atau pilih field kata kunci lain.</ScreenMessage>;
  } else {
    main = (
      <div className="conceptual-main">
        {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
        <section className="pnl">
          <div className="ph">
            Peta tematik
            <span className="ph-aside">{`${formatInt(themes.length)} tema dari ${formatInt(data.wordsUsed)} ${data.field.toLowerCase()} · klik tema untuk detail`}</span>
          </div>
          <div className="pnl-body"><ThematicMap themes={themes} selected={current?.id ?? null}
            collectionShare={data.map.collectionRecentShare} onSelect={setSelected} /></div>
        </section>
        <section className="pnl">
          <div className="ph">
            Tren topik
            <span className="ph-aside">rentang Q1-Q3 tahun terbit, titik = median, ukuran = frekuensi</span>
          </div>
          <div className="pnl-body"><TrendTopics trends={data.trends} /></div>
        </section>
      </div>
    );
  }

  const tabs = <Tabs label="Struktur konseptual" tabs={TABS} active={tab} onChange={setTab} />;
  if (tab !== "map") {
    return (
      <>
        {tabs}
        {tab === "words" ? <WordsView version={version} />
          : tab === "network" ? <NetworkView version={version} unit="keywords" />
          : tab === "evolution" ? <EvolutionView version={version} />
          : tab === "synonyms" ? <SynonymsView version={version} onDataChanged={onDataChanged} /> : <FactorialView version={version} />}
      </>
    );
  }

  return (
    <>
      {tabs}
      <div className="screen">
        <section className="screen-main">{main}</section>
        <aside className="inspector" aria-label="Parameter dan detail tema">
          <div className="ph">Parameter</div>
          <ParamsForm applied={params} busy={loading} onApply={(p) => { setSelected(null); setParams(p); }} />
          <div className="ph">Tema dipilih</div>
          {current && data ? (
            <ThemeDetail theme={current} collectionShare={data.map.collectionRecentShare}
              recentFrom={data.map.recentFrom} recentTo={data.map.recentTo} />
          ) : (
            <p className="muted" style={{ padding: "10px 12px", margin: 0 }}>Belum ada tema.</p>
          )}
        </aside>
      </div>
    </>
  );
}
