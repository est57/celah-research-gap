import { useState } from "react";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { formatInt } from "../../lib/format";
import { BradfordChart } from "./BradfordChart";
import { Panel, SaOverview } from "./SaPanel";
import { SourcesTab } from "./SourcesTab";
import { TopSources } from "./TopSources";
import { QuartilesTab } from "./QuartilesTab";

export type SourcesScreenTab = "overview" | "sources" | "quartiles";

const TABS: TabDef<SourcesScreenTab>[] = [
  { id: "overview", label: "Ikhtisar sumber" },
  { id: "sources", label: "Tabel sumber" },
  { id: "quartiles", label: "Kuartil (SJR)" },
];

/** Journals and other publication sources: the most relevant ones, Bradford zones and the source table. */
interface Props {
  version: number;
  initialTab?: SourcesScreenTab;
  initialQuery?: string;
  onDataChanged: () => void;
}

export function SourcesScreen({ version, initialTab = "overview", initialQuery, onDataChanged }: Props) {
  const [tab, setTab] = useState<SourcesScreenTab>(initialTab);
  return (
    <>
      <Tabs label="Sumber" tabs={TABS} active={tab} onChange={setTab} />
      {tab === "sources" ? <SourcesTab version={version} query={initialQuery} />
        : tab === "quartiles" ? <QuartilesTab version={version} onDataChanged={onDataChanged} /> : (
        <SaOverview version={version} stack content={(d) => (
          <>
            <Panel title="Sumber paling relevan" aside={`${formatInt(d.totalSources)} sumber`}><TopSources items={d.topSources} wide /></Panel>
            <Panel title="Hukum Bradford" aside={`${d.bradford.ranked.length} sumber teratas`}>
              <BradfordChart ranked={d.bradford.ranked} zones={d.bradford.zones} />
            </Panel>
          </>
        )} />
      )}
    </>
  );
}
