import { useMemo, useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useEvolution } from "../../hooks/useEvolution";
import { QUADRANT_LABEL, type Quadrant } from "../../lib/conceptualTypes";
import { DEFAULT_EVOLUTION, QUADRANT_COLOR, type EvolutionParams, type ThemeRef } from "../../lib/evolutionTypes";
import { EvolutionChanges } from "./EvolutionChanges";
import { EvolutionChart } from "./EvolutionChart";
import { EvolutionDetail } from "./EvolutionDetail";
import { EvolutionForm } from "./EvolutionForm";
import { ChartExport } from "../../components/ChartExport";
import { t } from "../../i18n";

const QUADRANTS: Quadrant[] = ["motor", "basic", "niche", "emerging"];

export function EvolutionView({ version }: { version: number }) {
  const [params, setParams] = useState<EvolutionParams>(DEFAULT_EVOLUTION);
  const [minInclusion, setMinInclusion] = useState(0.3);
  const [selected, setSelected] = useState<ThemeRef | null>(null);
  const { data, sample, error, loading } = useEvolution(version, params);
  const links = useMemo(() => (data?.links ?? []).filter((l) => l.inclusion >= minInclusion), [data, minInclusion]);
  const themes = data?.periods.reduce((s, p) => s + p.themes.length, 0) ?? 0;

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? t("Menyusun peta tematik tiap periode...") : ""}</ScreenMessage>;
  else if (data.periods.length === 0) main = <EmptyCollection />;
  else if (themes === 0) main = <ScreenMessage>{t("Belum ada tema di periode mana pun. Tambahkan dokumen yang punya kata kunci.")}</ScreenMessage>;
  else {
    main = (
      <div className="conceptual-main">
        {sample ? <div className="notice">{t("Pratinjau browser: aliran tema di halaman ini adalah data contoh.")}</div> : null}
        <section className="pnl">
          <div className="ph">
            {t("Evolusi tematik")}
            <span className="ph-aside">{t("{n} periode · {field} · klik tema untuk detail", { n: data.periods.length, field: data.field.toLowerCase() })}</span>
            <ChartExport name={t("Evolusi tematik")} />
          </div>
          <div className="pnl-body">
            <EvolutionChart data={data} links={links} selected={selected} onSelect={setSelected} />
            <div className="evo-legend">
              {QUADRANTS.map((q) => (
                <span key={q}><span className="theme-swatch" style={{ background: QUADRANT_COLOR[q] }} />{t(QUADRANT_LABEL[q])}</span>
              ))}
              <span className="muted">{t("Tinggi = kemunculan kata kunci. Tebal aliran = kata kunci yang sama.")}</span>
            </div>
          </div>
        </section>
        <EvolutionChanges data={data} minInclusion={minInclusion} onSelect={setSelected} />
      </div>
    );
  }

  return (
    <div className="screen">
      <section className="screen-main">{main}</section>
      <aside className="inspector" aria-label={t("Parameter dan detail tema")}>
        <div className="ph">{t("Parameter")}</div>
        <EvolutionForm applied={params} usedCuts={data?.cuts ?? []} busy={loading} minInclusion={minInclusion}
          onMinInclusion={setMinInclusion} onApply={(p) => { setSelected(null); setParams(p); }} />
        <div className="ph">{t("Tema dipilih")}</div>
        {selected && data ? (
          <EvolutionDetail data={data} links={links} selected={selected} />
        ) : (
          <p className="muted evo-empty">{t("Klik tema di grafik untuk melihat asal dan kelanjutannya.")}</p>
        )}
      </aside>
    </div>
  );
}
