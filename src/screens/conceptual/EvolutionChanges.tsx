import { endedThemes, newThemes, periodLabel, QUADRANT_COLOR, type Evolution, type ThemeRef } from "../../lib/evolutionTypes";

interface Props {
  data: Evolution;
  minInclusion: number;
  onSelect: (t: ThemeRef) => void;
}

/** Themes that appear without a predecessor or stop without a successor, per period. */
export function EvolutionChanges({ data, minInclusion, onSelect }: Props) {
  const rows = data.periods.map((p, i) => ({ label: periodLabel(p), fresh: newThemes(data, i, minInclusion), ended: endedThemes(data, i, minInclusion) }));
  const last = rows.length - 1;
  const chip = (period: number, t: { id: number; name: string; quadrant: keyof typeof QUADRANT_COLOR }) => (
    <button key={t.id} type="button" className="evo-chip" onClick={() => onSelect({ period, id: t.id })}>
      <span className="theme-swatch" style={{ background: QUADRANT_COLOR[t.quadrant] }} />
      {t.name}
    </button>
  );
  return (
    <section className="pnl">
      <div className="ph">
        Perubahan tema
        <span className="ph-aside">{`pada inklusi minimum ${Math.round(minInclusion * 100)}%`}</span>
      </div>
      <div className="pnl-body evo-changes">
        {last > 0 && rows[last].fresh.length ? (
          <div className="notice">
            <strong>Sinyal celah: tema baru di {rows[last].label}</strong>
            <span>
              Tema ini tidak berasal dari tema periode sebelumnya. Arah baru seperti ini belum banyak dikaitkan dengan
              literatur yang lebih mapan.
            </span>
          </div>
        ) : null}
        {rows.map((r, i) => (
          <div key={r.label} className="evo-change-row">
            <span className="num evo-change-period">{r.label}</span>
            <div className="evo-change-lists">
              {i > 0 ? <div><span className="opt-label">Baru</span>{r.fresh.length ? r.fresh.map((t) => chip(i, t)) : <span className="muted">tidak ada</span>}</div> : null}
              {i < last ? <div><span className="opt-label">Tidak berlanjut</span>{r.ended.length ? r.ended.map((t) => chip(i, t)) : <span className="muted">tidak ada</span>}</div> : null}
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
