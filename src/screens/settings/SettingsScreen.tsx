import { useState } from "react";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { isDesktop } from "../../lib/api";
import type { ApiKeyState } from "../../hooks/useApiKey";
import { ApiKeyPanel } from "./ApiKeyPanel";
import { DisplayPanel } from "./DisplayPanel";
import { AboutPanel } from "./AboutPanel";
import { UpdatePanel } from "./UpdatePanel";
import type { DisplaySettings } from "../../lib/display";
import { fill } from "../../components/fill";
import { t, tk } from "../../i18n";

export type SettingsTab = "sources" | "display" | "about";
type TabId = SettingsTab;

const TABS: TabDef<TabId>[] = [
  { id: "sources", label: tk("Sumber data") },
  { id: "display", label: tk("Tampilan") },
  { id: "about", label: tk("Tentang") },
];

const link = (url: string) => <span className="num select">{url}</span>;

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
        <Tabs label={t("Pengaturan")} tabs={TABS} active={tab} onChange={setTab} />
        <div className="settings-page">
          {tab === "display" ? <DisplayPanel settings={display} onChange={onDisplay} /> : <><AboutPanel /><UpdatePanel /></>}
        </div>
      </>
    );
  }
  return (
    <>
      <Tabs label={t("Pengaturan")} tabs={TABS} active={tab} onChange={setTab} />
      <div className="settings-page">
        {!isDesktop() ? (
          <div className="notice">{t("Pengaturan kunci hanya berfungsi di aplikasi desktop.")}</div>
        ) : null}
        <ApiKeyPanel
          api={openalex}
          id="openalex"
          title={t("Kunci API OpenAlex")}
          host="api.openalex.org"
          intro={fill(t("Tanpa kunci, semua pengguna di jaringan yang sama berbagi kuota harian gratis yang kecil. Dengan kunci gratis dari OpenAlex, kuota menjadi milik Anda sendiri. Buat kunci di {link} lalu tempel di bawah."),
            { link: link("openalex.org/settings/api") })}
        />
        <ApiKeyPanel
          api={semanticScholar}
          id="s2"
          title={t("Kunci API Semantic Scholar")}
          host="api.semanticscholar.org"
          intro={fill(t("Tanpa kunci, Semantic Scholar sering menolak permintaan karena kuota bersama sudah penuh. Ajukan kunci gratis di {link} lalu tempel di bawah."),
            { link: link("semanticscholar.org/product/api") })}
        />
        <ApiKeyPanel
          api={ncbi}
          id="ncbi"
          title={t("Kunci API NCBI (PubMed)")}
          host="eutils.ncbi.nlm.nih.gov"
          intro={fill(t("Tanpa kunci, PubMed membatasi 3 permintaan per detik sehingga pencarian besar lebih lambat. Dengan kunci gratis dari akun NCBI, batasnya naik menjadi 10. Buat kunci di halaman pengaturan akun {link} lalu tempel di bawah."),
            { link: link("ncbi.nlm.nih.gov/account") })}
        />
        <p className="muted" style={{ margin: 0, fontSize: 12 }}>
          {t("Crossref, Europe PMC, dan arXiv tidak memerlukan kunci API.")}
        </p>
      </div>
    </>
  );
}
