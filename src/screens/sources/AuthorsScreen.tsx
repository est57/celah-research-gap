import { useState } from "react";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { AffiliationsTab } from "./AffiliationsTab";
import { AuthorsTab } from "./AuthorsTab";
import { AuthorTimeline } from "./AuthorTimeline";
import { ImpactTable } from "./ImpactTable";
import { LotkaChart } from "./LotkaChart";
import { Panel, SaOverview } from "./SaPanel";
import { t, tk } from "../../i18n";

export type AuthorsScreenTab = "overview" | "authors" | "affiliations";

const TABS: TabDef<AuthorsScreenTab>[] = [
  { id: "overview", label: tk("Ikhtisar penulis") },
  { id: "authors", label: tk("Tabel penulis") },
  { id: "affiliations", label: tk("Afiliasi") },
];

interface Props {
  version: number;
  initialTab?: AuthorsScreenTab;
  initialQuery?: string;
  onDataChanged: () => void;
}

/** Authors and their institutions: productivity (Lotka), production over time, local impact, tables. */
export function AuthorsScreen({ version, initialTab = "overview", initialQuery, onDataChanged }: Props) {
  const [tab, setTab] = useState<AuthorsScreenTab>(initialTab);
  let body;
  if (tab === "authors") body = <AuthorsTab version={version} query={initialQuery} onDataChanged={onDataChanged} />;
  else if (tab === "affiliations") body = <AffiliationsTab version={version} onDataChanged={onDataChanged} />;
  else {
    body = (
      <SaOverview version={version} stack content={(d) => (
        <>
          <Panel title={t("Hukum Lotka")} aside={t("produktivitas penulis")}>
            <LotkaChart rows={d.lotka.rows} totalAuthors={d.lotka.totalAuthors} beta={d.lotka.beta} />
          </Panel>
          <Panel title={t("Dampak lokal penulis")} aside={t("urut indeks h")}><ImpactTable rows={d.impact} /></Panel>
          <Panel title={t("Produksi penulis dari waktu ke waktu")} aside={t("ukuran = dokumen, warna = sitasi per tahun")}>
            <AuthorTimeline rows={d.production} currentYear={d.currentYear} />
          </Panel>
        </>
      )} />
    );
  }
  return (
    <>
      <Tabs label={t("Penulis")} tabs={TABS.map((x) => ({ ...x, label: t(x.label) }))} active={tab} onChange={setTab} />
      {body}
    </>
  );
}
