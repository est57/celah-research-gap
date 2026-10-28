import { useMemo } from "react";
import { useWidth } from "../../hooks/useWidth";
import { formatDecimal } from "../../lib/format";
import { convexHull, type Pt } from "../../lib/hull";
import { placeLabels } from "../../lib/labelPlace";
import { clusterColor } from "../../lib/structureTypes";
import type { Factorial } from "../../lib/factorialTypes";
import { t } from "../../i18n";

const PAD = 34;
const LABELS = 20;

interface Props {
  data: Factorial;
  focus: number | null;
  onFocus: (cluster: number | null) => void;
}

export function FactorialMap({ data, focus, onFocus }: Props) {
  const { ref, width } = useWidth<HTMLDivElement>(800);
  const height = Math.round(Math.min(580, Math.max(360, width * 0.62)));
  const { sx, sy } = useMemo(() => {
    const xs = data.words.map((w) => w.x).concat(0);
    const ys = data.words.map((w) => w.y).concat(0);
    const span = (v: number[]) => [Math.min(...v), Math.max(...v)] as const;
    const [x0, x1] = span(xs);
    const [y0, y1] = span(ys);
    return {
      sx: (x: number) => PAD + ((x - x0) / (x1 - x0 || 1)) * (width - PAD * 2),
      sy: (y: number) => height - PAD - ((y - y0) / (y1 - y0 || 1)) * (height - PAD * 2),
    };
  }, [data.words, width, height]);
  const maxFreq = Math.max(...data.words.map((w) => w.freq), 1);
  const dim = (c: number) => focus !== null && focus !== c;
  const radius = (freq: number) => 2.5 + 5 * Math.sqrt(freq / maxFreq);
  // Most frequent words first; a label that would cover another one is left out (the point keeps its tooltip).
  const labels = new Map(
    placeLabels(
      data.words.flatMap((w, i) => (dim(w.cluster) ? [] : [{ index: i, x: sx(w.x), y: sy(w.y), r: radius(w.freq), text: w.label, weight: w.freq + (focus === w.cluster ? maxFreq : 0) }])),
      width, height, focus === null ? LABELS : LABELS * 2,
    ).map((l) => [l.index, l]),
  );

  return (
    <div ref={ref} className="fmap">
      <svg width={width} height={height} role="img" aria-label={t("Peta konseptual kata kunci hasil MCA")} onClick={() => onFocus(null)}>
        <line x1={sx(0)} x2={sx(0)} y1={PAD / 2} y2={height - PAD / 2} className="fmap-axis" />
        <line x1={PAD / 2} x2={width - PAD / 2} y1={sy(0)} y2={sy(0)} className="fmap-axis" />
        <text x={width - PAD / 2} y={height - 6} textAnchor="end" className="fmap-dim">{`Dim 1 (${formatDecimal(data.explained[0] * 100, 1)}%)`}</text>
        <text x={sx(0) + 6} y={PAD / 2 + 10} className="fmap-dim">{`Dim 2 (${formatDecimal(data.explained[1] * 100, 1)}%)`}</text>
        {data.clusters.map((c) => {
          const hull = convexHull(data.words.filter((w) => w.cluster === c.id).map((w): Pt => [sx(w.x), sy(w.y)]));
          if (hull.length < 3) return null;
          return (
            <polygon key={c.id} points={hull.map((p) => p.join(",")).join(" ")} fill={clusterColor(c.id)} stroke={clusterColor(c.id)}
              fillOpacity={dim(c.id) ? 0.02 : 0.08} strokeOpacity={dim(c.id) ? 0.1 : 0.35} strokeLinejoin="round" strokeWidth={14}
              className="fmap-hull" onClick={(e) => { e.stopPropagation(); onFocus(c.id); }} />
          );
        })}
        {data.words.map((w, i) => {
          const l = labels.get(i);
          return (
            <g key={w.label} opacity={dim(w.cluster) ? 0.2 : 1}>
              <circle cx={sx(w.x)} cy={sy(w.y)} r={radius(w.freq)} fill={clusterColor(w.cluster)}>
                <title>{t("{label} ({n} dokumen)", { label: w.label, n: w.freq })}</title>
              </circle>
              {l ? <text x={l.x} y={l.y} dy="0.35em" textAnchor={l.anchor} className="fmap-label">{w.label}</text> : null}
            </g>
          );
        })}
      </svg>
    </div>
  );
}
