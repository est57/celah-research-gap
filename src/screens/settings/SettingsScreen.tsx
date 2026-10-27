import { useState } from "react";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { isDesktop } from "../../lib/api";
import type { ApiKeyState } from "../../hooks/useApiKey";
import { ApiKeyPanel } from "./ApiKeyPanel";
import { DisplayPanel } from "./DisplayPanel";
import { AboutPanel } from "./AboutPanel";
import type { DisplaySettings } from "../../lib/display";

export type SettingsTab = "sources" | "display" | "about";
type TabId = SettingsTab;

const TABS: TabDef<TabId>[] = [
  { id: "sources", label: "Sumber data" },
  { id: "display", label: "Tampilan" },
  { id: "about", label: "Tentang" },
];

interface Props {
  openalex: ApiKeyState;
  semanticScholar: ApiKeyState;
  ncbi: ApiKeyState;
  display: DisplaySettings;
  onDisplay: (patch: Partial<DisplaySettings>) => void;
  initialTab?: TabId;
}

export function SettingsScreen({ openalex, semanticScholar, ncbi, display, onDisplay, initialTab = "sources" }: Props) {
  const [tab, setTab] = useState<TabId>(initialTab);
  if (tab === "display" || tab === "about") {
    return (
      <>
        <Tabs label="Pengaturan" tabs={TABS} active={tab} onChange={setTab} />
        <div className="settings-page">
          {tab === "display" ? <DisplayPanel settings={display} onChange={onDisplay} /> : <AboutPanel />}
        </div>
      </>
    );
  }
  return (
    <>
      <Tabs label="Pengaturan" tabs={TABS} active={tab} onChange={setTab} />
      <div className="settings-page">
        {!isDesktop() ? (
          <div className="notice">Pengaturan kunci hanya berfungsi di aplikasi desktop.</div>
        ) : null}
        <ApiKeyPanel
          api={openalex}
          id="openalex"
          title="Kunci API OpenAlex"
          host="api.openalex.org"
          intro={
            <>
              Tanpa kunci, semua pengguna di jaringan yang sama berbagi kuota harian gratis yang kecil. Dengan
              kunci gratis dari OpenAlex, kuota menjadi milik Anda sendiri. Buat kunci di{" "}
              <span className="num select">openalex.org/settings/api</span> lalu tempel di bawah.
            </>
          }
        />
        <ApiKeyPanel
          api={semanticScholar}
          id="s2"
          title="Kunci API Semantic Scholar"
          host="api.semanticscholar.org"
          intro={
            <>
              Tanpa kunci, Semantic Scholar sering menolak permintaan karena kuota bersama sudah penuh. Ajukan
              kunci gratis di <span className="num select">semanticscholar.org/product/api</span> lalu tempel di
              bawah.
            </>
          }
        />
        <ApiKeyPanel
          api={ncbi}
          id="ncbi"
          title="Kunci API NCBI (PubMed)"
          host="eutils.ncbi.nlm.nih.gov"
          intro={
            <>
              Tanpa kunci, PubMed membatasi 3 permintaan per detik sehingga pencarian besar lebih lambat. Dengan
              kunci gratis dari akun NCBI, batasnya naik menjadi 10. Buat kunci di halaman pengaturan akun{" "}
              <span className="num select">ncbi.nlm.nih.gov/account</span> lalu tempel di bawah.
            </>
          }
        />
        <p className="muted" style={{ margin: 0, fontSize: 12 }}>
          Crossref, Europe PMC, dan arXiv tidak memerlukan kunci API.
        </p>
      </div>
    </>
  );
}
