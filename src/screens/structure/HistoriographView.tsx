import { useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useHistoriograph } from "../../hooks/useHistoriograph";
import { formatInt } from "../../lib/format";
import { HistChart } from "./HistChart";
import { HistDetail } from "./HistDetail";
import { CompleteRefs } from "./CompleteRefs";
import { ChartExport } from "../../components/ChartExport";
import { t } from "../../i18n";

const NODE_OPTIONS = [20, 30, 50, 80];

export function HistoriographView({ version, onDataChanged }: { version: number; onDataChanged: () => void }) {
  const [nodes, setNodes] = useState(30);
  const [selected, setSelected] = useState<number | null>(null);
  const { data, sample, error, loading } = useHistoriograph(version, nodes);

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? t("Mencocokkan referensi dengan dokumen koleksi...") : ""}</ScreenMessage>;
  else if (data.documents === 0) main = <EmptyCollection />;
  else if (data.nodes.length === 0) {
    main = (
      <ScreenMessage>
        <strong>{t("Belum ada sitasi antar dokumen koleksi")}</strong>
        <span className="muted">
          {t("Historiograf butuh daftar rujukan. Lengkapi rujukan dari OpenAlex untuk dokumen yang sudah ada, atau impor ekspor Scopus/Web of Science yang menyertakan referensi.")}
        </span>
        <CompleteRefs version={version} onDone={onDataChanged} />
      </ScreenMessage>
    );
  } else {
    main = (
      <div className="structure-main">
        {sample ? <div className="notice">{t("Pratinjau browser: historiograf di halaman ini adalah data contoh.")}</div> : null}
        <section className="pnl">
          <div className="ph">
            {t("Historiograf")}
            <span className="ph-aside">{t("{n} dokumen dengan sitasi lokal tertinggi · {links} sitasi langsung", { n: formatInt(data.nodes.length), links: formatInt(data.edges.length) })}</span>
            <ChartExport name={t("Historiograf")} />
          </div>
          <div className="pnl-body">
            <HistChart data={data} selected={selected} onSelect={setSelected} />
            <p className="muted net-caption">
              {t("Panah menunjuk dari dokumen yang menyitasi ke dokumen yang disitasi. Ukuran titik = sitasi lokal (dari dokumen di koleksi). Warna = klaster Louvain. Klik dokumen untuk melihat rantai sitasinya.")}
            </p>
          </div>
        </section>
      </div>
    );
  }

  return (
    <div className="screen">
      <section className="screen-main">{main}</section>
      <aside className="inspector" aria-label={t("Parameter dan detail dokumen")}>
        <div className="ph">{t("Parameter")}</div>
        <div className="params">
          <div className="param-row">
            <label htmlFor="hist-nodes">{t("Jumlah dokumen")}</label>
            <select id="hist-nodes" className="input" value={nodes} onChange={(e) => { setNodes(Number(e.target.value)); setSelected(null); }}>
              {NODE_OPTIONS.map((n) => <option key={n} value={n}>{n}</option>)}
            </select>
          </div>
          {data ? (
            <>
              <div className="param-row"><span>{t("Sitasi lokal ditemukan")}</span><span className="num">{formatInt(data.links)}</span></div>
              <div className="param-row"><span>{t("lewat ID sumber")}</span><span className="num muted">{formatInt(data.byId)}</span></div>
              <div className="param-row"><span>{t("lewat DOI")}</span><span className="num muted">{formatInt(data.byDoi)}</span></div>
              <div className="param-row"><span>{t("lewat penulis + tahun")}</span><span className="num muted">{formatInt(data.byName)}</span></div>
            </>
          ) : null}
        </div>
        {data && data.nodes.length > 0 ? <CompleteRefs version={version} onDone={onDataChanged} /> : null}
        <div className="ph">{t("Dokumen dipilih")}</div>
        {data && selected !== null ? (
          <HistDetail data={data} index={selected} onSelect={setSelected} />
        ) : (
          <p className="muted evo-empty">{t("Klik dokumen di grafik untuk melihat apa yang disitasinya dan siapa yang menyitasinya.")}</p>
        )}
      </aside>
    </div>
  );
}
