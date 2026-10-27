import { useState } from "react";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { CouplingView } from "./CouplingView";
import { NetworkView } from "./NetworkView";
import { HistoriographView } from "./HistoriographView";
import { RpysView } from "./RpysView";
import { TopRefsView } from "./TopRefsView";

export type StructureTab = "references" | "toprefs" | "coupling" | "historiograph" | "rpys";

const TABS: TabDef<StructureTab>[] = [
  { id: "references", label: "Ko-sitasi" },
  { id: "toprefs", label: "Rujukan teratas" },
  { id: "coupling", label: "Kopling bibliografis" },
  { id: "historiograph", label: "Historiograf" },
  { id: "rpys", label: "Akar sejarah" },
];

interface Props {
  version: number;
  initialTab: StructureTab;
  /** Called after an action here changed stored data, so the app reloads its counts. */
  onDataChanged: () => void;
}

/** Intellectual structure: what the collection cites, and how its documents relate through citations. */
export function StructureScreen({ version, initialTab, onDataChanged }: Props) {
  const [tab, setTab] = useState<StructureTab>(initialTab);
  return (
    <>
      <Tabs label="Struktur intelektual" tabs={TABS} active={tab} onChange={setTab} />
      {tab === "coupling" ? (
        <CouplingView version={version} />
      ) : tab === "references" ? (
        <NetworkView version={version} unit="references" onDataChanged={onDataChanged} />
      ) : tab === "toprefs" ? (
        <TopRefsView version={version} onDataChanged={onDataChanged} />
      ) : tab === "rpys" ? (
        <RpysView version={version} onDataChanged={onDataChanged} />
      ) : (
        <HistoriographView version={version} onDataChanged={onDataChanged} />
      )}
    </>
  );
}
