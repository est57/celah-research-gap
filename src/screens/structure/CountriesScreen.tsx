import { useState } from "react";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { CountriesTab } from "../sources/CountriesTab";
import { CountriesView } from "./CountriesView";
import { t, tk } from "../../i18n";

export type CountriesScreenTab = "production" | "collaboration";

const TABS: TabDef<CountriesScreenTab>[] = [
  { id: "production", label: tk("Produksi negara") },
  { id: "collaboration", label: tk("Kolaborasi negara") },
];

/** Countries: production and impact per country, and the collaboration map and network. */
export function CountriesScreen({ version, initialTab = "production" }: { version: number; initialTab?: CountriesScreenTab }) {
  const [tab, setTab] = useState<CountriesScreenTab>(initialTab);
  return (
    <>
      <Tabs label={t("Negara dan kolaborasi")} tabs={TABS.map((x) => ({ ...x, label: t(x.label) }))} active={tab} onChange={setTab} />
      {tab === "production" ? <CountriesTab version={version} /> : <CountriesView version={version} />}
    </>
  );
}
