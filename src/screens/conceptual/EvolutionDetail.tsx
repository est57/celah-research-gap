import { formatInt } from "../../lib/format";
import { QUADRANT_LABEL } from "../../lib/conceptualTypes";
import { QUADRANT_COLOR, periodLabel, type Evolution, type ThemeLink, type ThemeRef } from "../../lib/evolutionTypes";
import { t, tk } from "../../i18n";

interface Props {
  data: Evolution;
  links: ThemeLink[];
  selected: ThemeRef;
}

function Flow({ title, items }: { title: string; items: { name: string; link: ThemeLink }[] }) {
  return (
    <div className="evo-flow">
      <span className="opt-label">{t(title)}</span>
      {items.length === 0 ? <span className="muted">{t("Tidak ada di atas inklusi minimum.")}</span> : null}
      {items.map(({ name, link }) => (
        <div key={`${link.period}-${link.from}-${link.to}`} className="evo-flow-item">
          <div className="evo-flow-head"><span>{name}</span><span className="num">{`${Math.round(link.inclusion * 100)}%`}</span></div>
          <div className="muted">{link.shared.join(", ")}</div>
        </div>
      ))}
    </div>
  );
}

export function EvolutionDetail({ data, links, selected }: Props) {
  const period = data.periods[selected.period];
  const theme = period?.themes.find((t) => t.id === selected.id);
  if (!theme) return null;
  const name = (p: number, id: number) => data.periods[p]?.themes.find((t) => t.id === id)?.name ?? "";
  const incoming = links.filter((l) => l.period + 1 === selected.period && l.to === theme.id).map((l) => ({ name: name(l.period, l.from), link: l }));
  const outgoing = links.filter((l) => l.period === selected.period && l.from === theme.id).map((l) => ({ name: name(l.period + 1, l.to), link: l }));
  return (
    <div className="theme-detail">
      <div className="theme-head">
        <span className="theme-swatch" style={{ background: QUADRANT_COLOR[theme.quadrant] }} />
        <strong>{theme.name}</strong>
      </div>
      <div className="muted theme-quadrant">{t("{period} · {quadrant} · {n} dokumen", { period: periodLabel(period), quadrant: t(QUADRANT_LABEL[theme.quadrant]), n: formatInt(theme.documents) })}</div>
      <ul className="theme-words">
        {theme.words.slice(0, 8).map((w) => (
          <li key={w.label}><span>{w.label}</span><span className="num muted">{formatInt(w.freq)}</span></li>
        ))}
      </ul>
      {selected.period > 0 ? <Flow title={tk("Berasal dari")} items={incoming} /> : null}
      {selected.period < data.periods.length - 1 ? <Flow title={tk("Berlanjut ke")} items={outgoing} /> : null}
    </div>
  );
}
