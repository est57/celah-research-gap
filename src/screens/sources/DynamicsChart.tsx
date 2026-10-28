import { useWidth } from "../../hooks/useWidth";
import { formatInt } from "../../lib/format";
import { clusterColor } from "../../lib/structureTypes";
import type { SourcesDetail } from "../../lib/entityTypes";
import { t } from "../../i18n";

type Dynamics = Pick<SourcesDetail, "years" | "dynamics">;

const H = 260;
const PAD = { left: 36, right: 12, top: 12, bottom: 24 };

/** Cumulative documents of the top sources (or institutions) per year; `label` is already translated. */
export function DynamicsChart({ data, label = t("sumber"), aria }: { data: Dynamics; label?: string; aria?: string }) {
  const { ref, width } = useWidth<HTMLDivElement>(700);
  const { years, dynamics } = data;
  const top = Math.max(1, ...dynamics.flatMap((s) => s.cumulative));
  const plotW = Math.max(10, width - PAD.left - PAD.right);
  const plotH = H - PAD.top - PAD.bottom;
  const x = (i: number) => PAD.left + (years.length > 1 ? (i / (years.length - 1)) * plotW : plotW / 2);
  const y = (v: number) => PAD.top + plotH - (v / top) * plotH;
  const every = Math.max(1, Math.ceil(years.length / Math.max(1, plotW / 44)));

  return (
    <div ref={ref}>
      <svg width={width} height={H} role="img" aria-label={aria ?? t("Produksi kumulatif {label} teratas per tahun", { label })}>
        {[0.5, 1].map((f) => (
          <g key={f}>
            <line x1={PAD.left} x2={width - PAD.right} y1={y(top * f)} y2={y(top * f)} className="chart-grid" />
            <text x={PAD.left - 6} y={y(top * f)} dy="0.35em" textAnchor="end" className="chart-axis">{formatInt(Math.round(top * f))}</text>
          </g>
        ))}
        <line x1={PAD.left} x2={width - PAD.right} y1={y(0)} y2={y(0)} className="chart-baseline" />
        {years.map((yr, i) => (i % every === 0 ? <text key={yr} x={x(i)} y={H - 6} textAnchor="middle" className="chart-axis">{yr}</text> : null))}
        {dynamics.map((s, k) => (
          <path key={s.label} d={s.cumulative.map((v, i) => `${i ? "L" : "M"}${x(i).toFixed(1)} ${y(v).toFixed(1)}`).join(" ")}
            fill="none" stroke={clusterColor(k)} strokeWidth={2}>
            <title>{t("{name}: {n} dokumen", { name: s.label, n: formatInt(s.cumulative[s.cumulative.length - 1] ?? 0) })}</title>
          </path>
        ))}
      </svg>
      <div className="dyn-legend">
        {dynamics.map((s, k) => (
          <span key={s.label}><span className="dyn-swatch" style={{ background: clusterColor(k) }} />{s.label}</span>
        ))}
      </div>
    </div>
  );
}
