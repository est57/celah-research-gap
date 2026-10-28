import { useMemo } from "react";
import { useWidth } from "../../hooks/useWidth";
import { ALLUVIAL_NODE_W, layoutAlluvial } from "../../lib/alluvial";
import { QUADRANT_COLOR, periodLabel, type Evolution, type ThemeLink, type ThemeRef } from "../../lib/evolutionTypes";
import { t } from "../../i18n";

const HEAD = 34;
const CHAR_W = 6.1;

interface Props {
  data: Evolution;
  links: ThemeLink[];
  selected: ThemeRef | null;
  onSelect: (t: ThemeRef | null) => void;
}

function clip(text: string, px: number): string {
  const max = Math.max(4, Math.floor(px / CHAR_W));
  return text.length > max ? `${text.slice(0, max - 1)}...` : text;
}

export function EvolutionChart({ data, links, selected, onSelect }: Props) {
  const { ref, width } = useWidth<HTMLDivElement>(900);
  const labelW = Math.min(230, Math.max(130, width * 0.22));
  const height = Math.round(Math.min(560, Math.max(360, width * 0.5)));
  const layout = useMemo(() => layoutAlluvial(data, links, width, height, labelW), [data, links, width, height, labelW]);
  const step = data.periods.length > 1 ? (width - labelW - ALLUVIAL_NODE_W) / (data.periods.length - 1) : width;
  const theme = (p: number, id: number) => data.periods[p].themes.find((t) => t.id === id);
  const isSel = (p: number, id: number) => selected?.period === p && selected.id === id;
  const touches = (l: ThemeLink) => selected !== null
    && ((l.period === selected.period && l.from === selected.id) || (l.period + 1 === selected.period && l.to === selected.id));

  return (
    <div ref={ref} className="evo-chart">
      <svg width={width} height={height + HEAD} role="img" aria-label={t("Aliran tema antar periode")} onClick={() => onSelect(null)}>
        {data.periods.map((p, i) => (
          <text key={i} x={layout.nodes.find((n) => n.period === i)?.x ?? 0} y={14} className="evo-period">
            <tspan fontWeight={600}>{periodLabel(p)}</tspan>
            <tspan dx={6} className="evo-period-n">{t("{n} dok", { n: p.documents })}</tspan>
          </text>
        ))}
        <g transform={`translate(0 ${HEAD})`}>
          {layout.ribbons.map((r, i) => {
            const src = theme(r.link.period, r.link.from);
            const lit = selected === null || touches(r.link);
            return (
              <path key={i} d={r.d} fill={src ? QUADRANT_COLOR[src.quadrant] : "#8a929c"}
                fillOpacity={lit ? 0.18 + 0.3 * r.link.inclusion : 0.05}>
                <title>{t("{pct}% inklusi: {words}", { pct: Math.round(r.link.inclusion * 100), words: r.link.shared.join(", ") })}</title>
              </path>
            );
          })}
          {layout.nodes.map((n) => {
            const th = theme(n.period, n.id);
            if (!th) return null;
            const dim = selected !== null && !isSel(n.period, n.id);
            return (
              <g key={`${n.period}:${n.id}`} className="evo-node" opacity={dim ? 0.55 : 1}
                onClick={(ev) => { ev.stopPropagation(); onSelect({ period: n.period, id: n.id }); }}>
                <rect x={n.x} y={n.y} width={ALLUVIAL_NODE_W} height={n.h} rx={1.5} fill={QUADRANT_COLOR[th.quadrant]}
                  style={{ stroke: isSel(n.period, n.id) ? "var(--ink)" : "none" }} strokeWidth={1.5} />
                {n.h >= 9 ? (
                  <text x={n.x + ALLUVIAL_NODE_W + 5} y={n.y + Math.min(n.h / 2, 9) + 4} className="evo-label">
                    {clip(th.name, (n.period === data.periods.length - 1 ? labelW : step) - 24)}
                  </text>
                ) : null}
                <title>{t("{name} ({n} kemunculan kata kunci)", { name: th.name, n: th.frequency })}</title>
              </g>
            );
          })}
        </g>
      </svg>
    </div>
  );
}
