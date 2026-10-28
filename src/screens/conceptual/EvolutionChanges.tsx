import { endedThemes, newThemes, periodLabel, QUADRANT_COLOR, type Evolution, type ThemeRef } from "../../lib/evolutionTypes";
import { t } from "../../i18n";

interface Props {
  data: Evolution;
  minInclusion: number;
  onSelect: (t: ThemeRef) => void;
}

/** Themes that appear without a predecessor or stop without a successor, per period. */
export function EvolutionChanges({ data, minInclusion, onSelect }: Props) {
  const rows = data.periods.map((p, i) => ({ label: periodLabel(p), fresh: newThemes(data, i, minInclusion), ended: endedThemes(data, i, minInclusion) }));
  const last = rows.length - 1;
  const chip = (period: number, th: { id: number; name: string; quadrant: keyof typeof QUADRANT_COLOR }) => (
    <button key={th.id} type="button" className="evo-chip" onClick={() => onSelect({ period, id: th.id })}>
      <span className="theme-swatch" style={{ background: QUADRANT_COLOR[th.quadrant] }} />
      {th.name}
    </button>
  );
  return (
    <section className="pnl">
      <div className="ph">
        {t("Perubahan tema")}
        <span className="ph-aside">{t("pada inklusi minimum {pct}%", { pct: Math.round(minInclusion * 100) })}</span>
      </div>
      <div className="pnl-body evo-changes">
        {last > 0 && rows[last].fresh.length ? (
          <div className="notice">
            <strong>{t("Sinyal celah: tema baru di {period}", { period: rows[last].label })}</strong>
            <span>
              {t("Tema ini tidak berasal dari tema periode sebelumnya. Arah baru seperti ini belum banyak dikaitkan dengan literatur yang lebih mapan.")}
            </span>
          </div>
        ) : null}
        {rows.map((r, i) => (
          <div key={r.label} className="evo-change-row">
            <span className="num evo-change-period">{r.label}</span>
            <div className="evo-change-lists">
              {i > 0 ? <div><span className="opt-label">{t("Baru")}</span>{r.fresh.length ? r.fresh.map((th) => chip(i, th)) : <span className="muted">{t("tidak ada")}</span>}</div> : null}
              {i < last ? <div><span className="opt-label">{t("Tidak berlanjut")}</span>{r.ended.length ? r.ended.map((th) => chip(i, th)) : <span className="muted">{t("tidak ada")}</span>}</div> : null}
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
