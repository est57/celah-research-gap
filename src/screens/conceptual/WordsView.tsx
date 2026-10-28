import { useState, type ReactNode } from "react";
import { ChartExport } from "../../components/ChartExport";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useWords } from "../../hooks/useWords";
import { formatInt } from "../../lib/format";
import { DEFAULT_WORDS, type WordParams, type Words } from "../../lib/wordTypes";
import { DynamicsChart } from "../sources/DynamicsChart";
import { TopSources } from "../sources/TopSources";
import { WordsSide } from "./WordsSide";
import { t } from "../../i18n";

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
  else if (!data) main = <ScreenMessage>{loading ? t("Menghitung frekuensi kata...") : ""}</ScreenMessage>;
  else if (data.documents === 0) main = <EmptyCollection />;
  else if (data.words.length === 0) {
    main = <ScreenMessage>{t("Belum ada dokumen dengan {field}. Pilih bidang lain, atau lengkapi metadata di Koleksi.", { field: data.field.toLowerCase() })}</ScreenMessage>;
  } else {
    const plural = data.ngram > 1;
    main = (
      <div className="conceptual-main">
        {sample ? <div className="notice">{t("Pratinjau browser: angka di halaman ini adalah data contoh.")}</div> : null}
        <section className="pnl">
          <div className="ph">
            {plural ? t("{field}: frasa paling sering", { field: data.field }) : t("{field}: kata paling sering", { field: data.field })}
            <span className="ph-aside">{t("{n} teratas · angka = jumlah dokumen", { n: formatInt(data.words.length) })}</span>
          </div>
          <div className="pnl-body words-bars">
            <TopSources plain wide items={data.words.map((w) => ({ label: w.label, count: w.documents }))} />
          </div>
        </section>
        {data.years.length > 1 ? (
          <section className="pnl">
            <div className="ph">
              {t("Dinamika kata")}
              <span className="ph-aside">{t("{n} teratas per tahun terbit", { n: data.series.length })}</span>
              <div className="words-mode" role="group" aria-label={t("Tampilan dinamika")}>
                <button type="button" aria-pressed={mode === "cumulative"} onClick={() => setMode("cumulative")}>{t("Kumulatif")}</button>
                <button type="button" aria-pressed={mode === "annual"} onClick={() => setMode("annual")}>{t("Per tahun")}</button>
              </div>
              <ChartExport name={t("Dinamika kata")} />
            </div>
            <div className="pnl-body">
              <DynamicsChart data={lines(data, mode)}
                aria={mode === "cumulative"
                  ? (plural ? t("Dokumen kumulatif untuk frasa teratas") : t("Dokumen kumulatif untuk kata teratas"))
                  : (plural ? t("Dokumen per tahun untuk frasa teratas") : t("Dokumen per tahun untuk kata teratas"))} />
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
