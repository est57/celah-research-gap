import { useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { useConceptual } from "../../hooks/useConceptual";
import { formatInt } from "../../lib/format";
import { t, tk } from "../../i18n";
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
  { id: "map", label: tk("Peta tematik & tren") },
  { id: "words", label: tk("Analisis kata") },
  { id: "network", label: tk("Jaringan ko-kemunculan") },
  { id: "evolution", label: tk("Evolusi tematik") },
  { id: "factorial", label: tk("Analisis faktorial") },
  { id: "synonyms", label: tk("Sinonim kata kunci") },
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
  const current = themes.find((x) => x.id === selected) ?? themes.find((x) => x.quadrant === "emerging") ?? themes[0];

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? t("Menyusun peta tematik...") : ""}</ScreenMessage>;
  else if (data.documents === 0) main = <EmptyCollection />;
  else if (themes.length === 0) {
    main = <ScreenMessage>{t("Belum ada tema yang memenuhi syarat. Coba turunkan frekuensi minimum tema, atau pilih field kata kunci lain.")}</ScreenMessage>;
  } else {
    main = (
      <div className="conceptual-main">
        {sample ? <div className="notice">{t("Pratinjau browser: angka di halaman ini adalah data contoh.")}</div> : null}
        <section className="pnl">
          <div className="ph">
            {t("Peta tematik")}
            <span className="ph-aside">{t("{n} tema dari {words} {field} · klik tema untuk detail", { n: formatInt(themes.length), words: formatInt(data.wordsUsed), field: data.field.toLowerCase() })}</span>
          </div>
          <div className="pnl-body"><ThematicMap themes={themes} selected={current?.id ?? null}
            collectionShare={data.map.collectionRecentShare} onSelect={setSelected} /></div>
        </section>
        <section className="pnl">
          <div className="ph">
            {t("Tren topik")}
            <span className="ph-aside">{t("rentang Q1-Q3 tahun terbit, titik = median, ukuran = frekuensi")}</span>
          </div>
          <div className="pnl-body"><TrendTopics trends={data.trends} /></div>
        </section>
      </div>
    );
  }

  const tabs = <Tabs label={t("Struktur konseptual")} tabs={TABS.map((x) => ({ ...x, label: t(x.label) }))} active={tab} onChange={setTab} />;
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
        <aside className="inspector" aria-label={t("Parameter dan detail tema")}>
          <div className="ph">{t("Parameter")}</div>
          <ParamsForm applied={params} busy={loading} onApply={(p) => { setSelected(null); setParams(p); }} />
          <div className="ph">{t("Tema dipilih")}</div>
          {current && data ? (
            <ThemeDetail theme={current} collectionShare={data.map.collectionRecentShare}
              recentFrom={data.map.recentFrom} recentTo={data.map.recentTo} />
          ) : (
            <p className="muted" style={{ padding: "10px 12px", margin: 0 }}>{t("Belum ada tema.")}</p>
          )}
        </aside>
      </div>
    </>
  );
}
