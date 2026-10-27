import { useState, type ReactNode } from "react";
import { ChartExport } from "../../components/ChartExport";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useWords } from "../../hooks/useWords";
import { formatInt } from "../../lib/format";
import { DEFAULT_WORDS, type WordParams, type Words } from "../../lib/wordTypes";
import { DynamicsChart } from "../sources/DynamicsChart";
import { TopSources } from "../sources/TopSources";
import { WordsSide } from "./WordsSide";

type Mode = "cumulative" | "annual";

/** Per-year counts, or their running total, in the shape DynamicsChart draws. */
function lines(data: Words, mode: Mode) {
  const dynamics = data.series.map((s) => {
    let sum = 0;
    return { label: s.label, cumulative: mode === "annual" ? s.perYear : s.perYear.map((v) => (sum += v)) };
  });
  return { years: data.years, dynamics };
}

export function WordsView({ version }: { version: number }) {
  const [params, setParams] = useState<WordParams>(DEFAULT_WORDS);
  const [mode, setMode] = useState<Mode>("cumulative");
  const { data, sample, error, loading } = useWords(version, params);

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? "Menghitung frekuensi kata..." : ""}</ScreenMessage>;
  else if (data.documents === 0) main = <EmptyCollection />;
  else if (data.words.length === 0) {
    main = <ScreenMessage>Belum ada dokumen dengan {data.field.toLowerCase()}. Pilih bidang lain, atau lengkapi metadata di Koleksi.</ScreenMessage>;
  } else {
    const unit = data.ngram > 1 ? "frasa" : "kata";
    main = (
      <div className="conceptual-main">
        {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
        <section className="pnl">
          <div className="ph">
            {`${data.field}: ${unit} paling sering`}
            <span className="ph-aside">{`${formatInt(data.words.length)} teratas · angka = jumlah dokumen`}</span>
          </div>
          <div className="pnl-body words-bars">
            <TopSources plain items={data.words.map((w) => ({ label: w.label, count: w.documents }))} />
          </div>
        </section>
        {data.years.length > 1 ? (
          <section className="pnl">
            <div className="ph">
              Dinamika kata
              <span className="ph-aside">{`${data.series.length} teratas per tahun terbit`}</span>
              <div className="words-mode" role="group" aria-label="Tampilan dinamika">
                <button type="button" aria-pressed={mode === "cumulative"} onClick={() => setMode("cumulative")}>Kumulatif</button>
                <button type="button" aria-pressed={mode === "annual"} onClick={() => setMode("annual")}>Per tahun</button>
              </div>
              <ChartExport name="Dinamika kata" />
            </div>
            <div className="pnl-body">
              <DynamicsChart data={lines(data, mode)}
                aria={`Dokumen ${mode === "cumulative" ? "kumulatif" : "per tahun"} untuk ${unit} teratas`} />
            </div>
          </section>
        ) : null}
      </div>
    );
  }

  return (
    <div className="screen">
      <section className="screen-main">{main}</section>
      <WordsSide params={params} data={data} onParams={setParams} />
    </div>
  );
}
