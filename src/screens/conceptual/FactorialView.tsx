import { useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useFactorial } from "../../hooks/useFactorial";
import { formatInt } from "../../lib/format";
import { DEFAULT_FACTORIAL, type FactorialParams } from "../../lib/factorialTypes";
import { clusterColor } from "../../lib/structureTypes";
import { FactorialMap } from "./FactorialMap";
import { FactorialSide } from "./FactorialSide";
import { ChartExport } from "../../components/ChartExport";

export function FactorialView({ version }: { version: number }) {
  const [params, setParams] = useState<FactorialParams>(DEFAULT_FACTORIAL);
  const [focus, setFocus] = useState<number | null>(null);
  const { data, sample, error, loading } = useFactorial(version, params);

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? "Menghitung analisis korespondensi berganda..." : ""}</ScreenMessage>;
  else if (data.documents === 0) main = <EmptyCollection />;
  else if (data.words.length === 0) main = <ScreenMessage>Kata kunci terlalu sedikit untuk peta faktorial. Tambahkan dokumen yang punya kata kunci.</ScreenMessage>;
  else {
    main = (
      <div className="conceptual-main">
        {sample ? <div className="notice">Pratinjau browser: peta di halaman ini adalah data contoh.</div> : null}
        <section className="pnl">
          <div className="ph">
            Peta struktur konseptual
            <span className="ph-aside">{`MCA · ${formatInt(data.words.length)} kata · ${formatInt(data.documents)} dokumen · klik klaster untuk menyorot`}</span>
            <ChartExport name="Peta struktur konseptual" />
          </div>
          <div className="pnl-body">
            <FactorialMap data={data} focus={focus} onFocus={setFocus} />
            <p className="muted fa-caption">Persentase dimensi memakai koreksi Benzecri. Ukuran titik = jumlah dokumen dengan kata kunci tersebut.</p>
          </div>
        </section>
        <section className="pnl">
          <div className="ph">Dokumen paling berkontribusi<span className="ph-aside">jarak terjauh dari pusat peta</span></div>
          <ol className="fa-docs">
            {data.topDocuments.map((d) => (
              <li key={d.id}>
                <span className="theme-swatch" style={{ background: clusterColor(d.cluster) }} />
                <span className="fa-doc-title">{d.title}</span>
                <span className="num muted">{d.year ?? "t.t."}</span>
              </li>
            ))}
          </ol>
        </section>
      </div>
    );
  }

  return (
    <div className="screen">
      <section className="screen-main">{main}</section>
      <FactorialSide params={params} data={data} focus={focus}
        onParams={(p) => { setFocus(null); setParams(p); }} onFocus={setFocus} />
    </div>
  );
}
