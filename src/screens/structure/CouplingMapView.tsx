import { useState, type ReactNode } from "react";
import { ChartExport } from "../../components/ChartExport";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useCouplingMap } from "../../hooks/useCouplingMap";
import { COUPLING_QUADRANT } from "../../lib/couplingMapTypes";
import { formatDecimal, formatInt } from "../../lib/format";
import { clusterColor } from "../../lib/structureTypes";
import { CouplingMapChart } from "./CouplingMapChart";
import { t } from "../../i18n";

const SIZES = [100, 200, 300, 500];

/** Clusters of coupled documents on a centrality x impact map, with the selected cluster's details. */
export function CouplingMapView({ version, extra }: { version: number; extra?: ReactNode }) {
  const [documents, setDocuments] = useState(200);
  const [selected, setSelected] = useState<number | null>(null);
  const { data, sample, error, loading } = useCouplingMap(version, documents);
  const clusters = data?.clusters ?? [];
  const current = clusters.find((c) => c.id === selected) ?? clusters[0];

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? t("Menyusun peta kopling...") : ""}</ScreenMessage>;
  else if (data.coupled === 0) {
    main = <ScreenMessage>{t("Belum ada dua dokumen yang merujuk karya yang sama. Lengkapi rujukan di tab Historiograf, atau impor ekspor Scopus/WoS.")}</ScreenMessage>;
  } else if (clusters.length === 0) main = <EmptyCollection />;
  else {
    main = (
      <div className="conceptual-main">
        {sample ? <div className="notice">{t("Pratinjau browser: peta di halaman ini adalah data contoh.")}</div> : null}
        <section className="pnl">
          <div className="ph">
            {t("Peta kopling dokumen")}
            <span className="ph-aside">{t("{n} klaster dari {docs} dokumen · klik klaster", { n: formatInt(clusters.length), docs: formatInt(data.used) })}</span>
            <ChartExport name={t("Peta kopling dokumen")} />
          </div>
          <div className="pnl-body"><CouplingMapChart clusters={clusters} selected={current?.id ?? null} onSelect={setSelected} /></div>
        </section>
      </div>
    );
  }

  return (
    <div className="screen">
      <section className="screen-main">{main}</section>
      <aside className="inspector" aria-label={t("Parameter dan klaster terpilih")}>
        <div className="ph">{t("Parameter")}</div>
        <div className="params">
          {extra}
          <div className="param-row">
            <label htmlFor="cm-docs">{t("Jumlah dokumen")}</label>
            <select id="cm-docs" className="input" value={documents} onChange={(e) => { setDocuments(Number(e.target.value)); setSelected(null); }}>
              {SIZES.map((n) => <option key={n} value={n}>{n}</option>)}
            </select>
          </div>
          <div className="param-row"><span>{t("Clustering")}</span><span className="muted">{t("Louvain")}</span></div>
        </div>
        <div className="ph">{t("Klaster terpilih")}</div>
        {current ? (
          <div className="cm-detail">
            <div className="cm-title">
              <span className="theme-swatch" style={{ background: clusterColor(current.id) }} />
              <strong>{current.keywords.join(", ") || t("Klaster {n}", { n: current.id + 1 })}</strong>
            </div>
            <dl className="words-facts">
              <dt>{t("Posisi")}</dt><dd>{t(COUPLING_QUADRANT[current.quadrant])}</dd>
              <dt>{t("Dokumen")}</dt><dd className="num">{formatInt(current.size)}</dd>
              <dt>{t("Dampak (1,0 = rata-rata)")}</dt><dd className="num">{formatDecimal(current.impact, 2)}</dd>
              <dt>{t("Sentralitas")}</dt><dd className="num">{formatDecimal(current.centrality, 1)}</dd>
            </dl>
            <ol className="cm-docs">
              {current.documents.map((d) => (
                <li key={d.id}><span>{d.title}</span><span className="num muted">{t("{year} · {n} sitasi", { year: d.year ?? t("t.t."), n: formatInt(d.citedBy) })}</span></li>
              ))}
            </ol>
          </div>
        ) : <p className="muted words-hint">{t("Belum ada klaster.")}</p>}
        <p className="muted words-hint">
          {t("Dokumen yang merujuk karya yang sama membentuk front riset. Klaster di kiri bawah (pinggiran) kurang terhubung dan jarang disitasi: bisa topik yang baru muncul atau ditinggalkan. Niche berdampak di kiri atas adalah topik kuat yang belum banyak dihubungkan dengan front lain, sering menjadi peluang celah riset.")}
        </p>
      </aside>
    </div>
  );
}
