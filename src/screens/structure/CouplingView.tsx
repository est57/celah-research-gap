import { useState } from "react";
import { CouplingMapView } from "./CouplingMapView";
import { NetworkView } from "./NetworkView";
import { t } from "../../i18n";

type View = "network" | "map";

/** Bibliographic coupling as a network, or as a map of document clusters. */
export function CouplingView({ version }: { version: number }) {
  const [view, setView] = useState<View>("network");
  const select = (
    <div className="param-row">
      <label htmlFor="cp-view">{t("Tampilan")}</label>
      <select id="cp-view" className="input" value={view} onChange={(e) => setView(e.target.value as View)}>
        <option value="network">{t("Jaringan")}</option>
        <option value="map">{t("Peta klaster dokumen")}</option>
      </select>
    </div>
  );
  return view === "map" ? <CouplingMapView version={version} extra={select} /> : <NetworkView version={version} unit="coupling" extra={select} />;
}
