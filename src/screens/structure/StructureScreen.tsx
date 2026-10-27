import { useState } from "react";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { CountriesView } from "./CountriesView";
import { CouplingView } from "./CouplingView";
import { NetworkView } from "./NetworkView";
import { HistoriographView } from "./HistoriographView";
import { RpysView } from "./RpysView";
import { TopRefsView } from "./TopRefsView";

export type StructureTab = "references" | "toprefs" | "coupling" | "authors" | "historiograph" | "rpys" | "countries";

const TABS: TabDef<StructureTab>[] = [
  { id: "references", label: "Ko-sitasi" },
  { id: "toprefs", label: "Rujukan teratas" },
  { id: "coupling", label: "Kopling bibliografis" },
  { id: "authors", label: "Kolaborasi penulis" },
  { id: "countries", label: "Kolaborasi negara" },
  { id: "historiograph", label: "Historiograf" },
  { id: "rpys", label: "Akar sejarah" },
];

interface Props {
  version: number;
  initialTab: StructureTab;
  /** Called after an action here changed stored data, so the app reloads its counts. */
  onDataChanged: () => void;
}

export function StructureScreen({ version, initialTab, onDataChanged }: Props) {
  const [tab, setTab] = useState<StructureTab>(initialTab);
  return (
    <>
      <Tabs label="Struktur jaringan" tabs={TABS} active={tab} onChange={setTab} />
      {tab === "countries" ? (
        <CountriesView version={version} />
      ) : tab === "coupling" ? (
        <CouplingView version={version} />
      ) : tab === "references" || tab === "authors" ? (
        <NetworkView key={tab} version={version} unit={tab} />
      ) : tab === "toprefs" ? (
        <TopRefsView version={version} onDataChanged={onDataChanged} />
      ) : tab === "rpys" ? (
        <RpysView version={version} />
      ) : (
        <HistoriographView version={version} onDataChanged={onDataChanged} />
      )}
    </>
  );
}
