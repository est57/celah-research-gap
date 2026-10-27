import { useState, type ReactNode } from "react";
import { ChartExport } from "../../components/ChartExport";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useRpys } from "../../hooks/useRpys";
import { formatInt } from "../../lib/format";
import { DEFAULT_RPYS, type RpysParams } from "../../lib/rpysTypes";
import { RpysChart } from "./RpysChart";
import { RpysPeaks } from "./RpysPeaks";
import { RpysSide } from "./RpysSide";

/** Reference Publication Year Spectroscopy: the historical roots of the collection. */
export function RpysView({ version }: { version: number }) {
  const [params, setParams] = useState<RpysParams>(DEFAULT_RPYS);
  const [selected, setSelected] = useState<number | null>(null);
  const { data, sample, error, loading } = useRpys(version, params);

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? "Menghitung tahun terbit rujukan..." : ""}</ScreenMessage>;
  else if (data.documents === 0) main = <EmptyCollection />;
  else if (data.dated === 0) {
    main = (
      <ScreenMessage>
        Belum ada rujukan bertahun. RPYS membaca daftar rujukan dalam bentuk teks, misalnya dari impor Scopus atau Web of
        Science, atau dari pencarian Crossref. Rujukan dari OpenAlex hanya berupa ID tanpa tahun terbit.
      </ScreenMessage>
    );
  } else {
    main = (
      <div className="conceptual-main">
        {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
        <section className="pnl">
          <div className="ph">
            Spektroskopi tahun rujukan
            <span className="ph-aside">{`${data.from}-${data.to} · ${formatInt(data.dated)} sitasi · klik batang atau puncak`}</span>
            <ChartExport name="Spektroskopi tahun rujukan" />
          </div>
          <div className="pnl-body"><RpysChart data={data} selected={selected} onSelect={setSelected} /></div>
        </section>
        <section className="pnl">
          <div className="ph">
            Tahun puncak dan karya yang paling dirujuk
            <span className="ph-aside">{data.peaks.length ? "urut tahun" : ""}</span>
          </div>
          {data.peaks.length ? (
            <RpysPeaks peaks={data.peaks} selected={selected} onSelect={setSelected} />
          ) : (
            <p className="muted rpys-empty">Belum ada tahun yang menonjol di rentang ini. Coba perlebar rentang tahun.</p>
          )}
        </section>
      </div>
    );
  }

  return (
    <div className="screen">
      <section className="screen-main">{main}</section>
      <RpysSide params={params} data={data} busy={loading} onParams={(p) => { setSelected(null); setParams(p); }} />
    </div>
  );
}
