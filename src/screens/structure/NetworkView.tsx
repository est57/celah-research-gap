import { useState, type ReactNode } from "react";
import { NoReferences } from "../../components/NoReferences";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useStructure } from "../../hooks/useStructure";
import { formatInt } from "../../lib/format";
import type { CouplingUnit, Unit } from "../../lib/structureTypes";
import { ClusterPanel, isWeakBridge } from "./ClusterPanel";
import { NetworkGraph } from "./NetworkGraph";
import { ChartExport } from "../../components/ChartExport";
import { t, tk } from "../../i18n";

const NODE_OPTIONS = [30, 50, 80, 120];

const TITLE: Record<Unit, string> = {
  references: tk("Jaringan ko-sitasi referensi"),
  authors: tk("Jaringan kolaborasi penulis"),
  keywords: tk("Jaringan ko-kemunculan kata kunci"),
  coupling_documents: tk("Kopling bibliografis dokumen"),
  coupling_sources: tk("Kopling bibliografis sumber"),
  coupling_authors: tk("Kopling bibliografis penulis"),
};

const LEVELS: { id: CouplingUnit; label: string }[] = [
  { id: "coupling_documents", label: tk("Dokumen") },
  { id: "coupling_sources", label: tk("Sumber") },
  { id: "coupling_authors", label: tk("Penulis") },
];

const SHARED = tk("jumlah rujukan yang juga dirujuk dokumen lain");
const SIZE: Record<Unit, string> = { references: tk("sitasi lokal"), authors: tk("jumlah dokumen"), keywords: tk("jumlah dokumen"),
  coupling_documents: SHARED, coupling_sources: SHARED, coupling_authors: SHARED };

/** Co-citation, collaboration, keyword or coupling network with its parameter and cluster panel.
 * "coupling" lets the user pick the level (documents, sources or authors). */
export function NetworkView({ version, unit: chosen, extra, onDataChanged }: { version: number; unit: Unit | "coupling"; extra?: ReactNode; onDataChanged?: () => void }) {
  const [level, setLevel] = useState<CouplingUnit>("coupling_documents");
  const unit: Unit = chosen === "coupling" ? level : chosen;
  const [nodes, setNodes] = useState(50);
  const [focus, setFocus] = useState<number | null>(null);
  const { data, sample, error, loading } = useStructure(version, { unit, nodes });
  const net = data?.network;

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data || !net) main = <ScreenMessage>{loading ? t("Menyusun jaringan...") : ""}</ScreenMessage>;
  else if (net.nodes.length === 0) {
    main = chosen === "coupling"
      ? <ScreenMessage>{t("Belum ada dua dokumen yang merujuk karya yang sama. Kopling butuh daftar rujukan: impor Scopus atau Web of Science, cari lewat Crossref, atau lengkapi rujukan OpenAlex di tab Historiograf.")}</ScreenMessage>
      : unit === "references" ? <NoReferences analysis={t("Jaringan ko-sitasi")} onDone={onDataChanged} />
      : data.documents === 0 ? <EmptyCollection /> : <ScreenMessage>{t("Belum ada data yang cukup untuk jaringan ini.")}</ScreenMessage>;
  } else {
    main = (
      <div className="structure-main">
        {sample ? <div className="notice">{t("Pratinjau browser: jaringan di halaman ini adalah data contoh.")}</div> : null}
        <section className="pnl">
          <div className="ph">
            {t(TITLE[unit])}
            <span className="ph-aside">{chosen === "coupling"
              ? t("{docs} dokumen terkopel, {nodes} node, {clusters} cluster, Louvain", { docs: formatInt(data.documents), nodes: formatInt(net.nodes.length), clusters: formatInt(net.clusters.length) })
              : t("{nodes} node, {clusters} cluster, Louvain", { nodes: formatInt(net.nodes.length), clusters: formatInt(net.clusters.length) })}</span>
            <ChartExport name={t(TITLE[unit])} />
          </div>
          <NetworkGraph nodes={net.nodes} edges={net.edges} bridge={net.bridges.find(isWeakBridge) ?? null}
            focus={focus} onFocus={setFocus} />
          <div className="muted net-caption">
            {t("Ukuran node = {size}. Tebal garis = kekuatan hubungan (normalisasi association). Klik node untuk menyorot cluster-nya.", { size: t(SIZE[unit]) })}
          </div>
        </section>
      </div>
    );
  }

  return (
    <div className="screen">
        <section className="screen-main">{main}</section>
        <aside className="inspector" aria-label={t("Parameter dan cluster")}>
          <div className="ph">{t("Parameter jaringan")}</div>
          <div className="params">
            {extra}
            {chosen === "coupling" ? (
              <div className="param-row">
                <label htmlFor="net-level">{t("Tingkat")}</label>
                <select id="net-level" className="input" value={level} onChange={(e) => { setLevel(e.target.value as CouplingUnit); setFocus(null); }}>
                  {LEVELS.map((l) => <option key={l.id} value={l.id}>{t(l.label)}</option>)}
                </select>
              </div>
            ) : null}
            <div className="param-row">
              <label htmlFor="net-nodes">{t("Jumlah node")}</label>
              <select id="net-nodes" className="input" value={nodes} onChange={(e) => { setNodes(Number(e.target.value)); setFocus(null); }}>
                {NODE_OPTIONS.map((n) => <option key={n} value={n}>{n}</option>)}
              </select>
            </div>
            <div className="param-row"><span>{t("Normalisasi")}</span><span className="muted">{t("Association")}</span></div>
            <div className="param-row"><span>{t("Clustering")}</span><span className="muted">{t("Louvain")}</span></div>
          </div>
          {net && data ? (
            <ClusterPanel clusters={net.clusters} bridges={net.bridges} focus={focus} unitLabel={data.unit} onFocus={setFocus} />
          ) : null}
        </aside>
    </div>
  );
}
