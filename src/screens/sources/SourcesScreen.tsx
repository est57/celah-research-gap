import { useState } from "react";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { formatInt } from "../../lib/format";
import { BradfordChart } from "./BradfordChart";
import { Panel, SaOverview } from "./SaPanel";
import { SourcesTab } from "./SourcesTab";
import { TopSources } from "./TopSources";
import { QuartilesTab } from "./QuartilesTab";
import { t, tk } from "../../i18n";

export type SourcesScreenTab = "overview" | "sources" | "quartiles";

const TABS: TabDef<SourcesScreenTab>[] = [
  { id: "overview", label: tk("Ikhtisar sumber") },
  { id: "sources", label: tk("Tabel sumber") },
  { id: "quartiles", label: tk("Kuartil (SJR)") },
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
      <Tabs label={t("Sumber")} tabs={TABS.map((x) => ({ ...x, label: t(x.label) }))} active={tab} onChange={setTab} />
      {tab === "sources" ? <SourcesTab version={version} query={initialQuery} />
        : tab === "quartiles" ? <QuartilesTab version={version} onDataChanged={onDataChanged} /> : (
        <SaOverview version={version} stack content={(d) => (
          <>
            <Panel title={t("Sumber paling relevan")} aside={t("{n} sumber", { n: formatInt(d.totalSources) })}><TopSources items={d.topSources} wide /></Panel>
            <Panel title={t("Hukum Bradford")} aside={t("{n} sumber teratas", { n: d.bradford.ranked.length })}>
              <BradfordChart ranked={d.bradford.ranked} zones={d.bradford.zones} />
            </Panel>
          </>
        )} />
      )}
    </>
  );
}
