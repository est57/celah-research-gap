import { t } from "../../i18n";
import { useWidth } from "../../hooks/useWidth";
import { formatDecimal } from "../../lib/format";
import type { YearPoint } from "../../lib/overviewTypes";

const PAD = { left: 30, right: 8, top: 10, bottom: 22 };

export type CitationMetric = "perYear" | "perArticle";

interface Props {
  points: YearPoint[];
  metric?: CitationMetric;
  height?: number;
}

export function CitationChart({ points, metric = "perYear", height = 170 }: Props) {
  const H = height;
  const val = (p: YearPoint) => (metric === "perArticle" ? p.meanTc : p.meanTcPerYear);
  const unit = metric === "perArticle" ? t("sitasi per artikel") : t("sitasi per tahun");
  const { ref, width } = useWidth<HTMLDivElement>(340);
  const max = Math.max(...points.map((p) => val(p)), 0);
  const top = max > 0 ? Math.ceil(max) : 1;
  const plotW = Math.max(width - PAD.left - PAD.right, 10);
  const plotH = H - PAD.top - PAD.bottom;
  const x = (i: number) => PAD.left + (points.length > 1 ? (i / (points.length - 1)) * plotW : plotW / 2);
  const y = (v: number) => PAD.top + plotH - (v / top) * plotH;
  const line = points.map((p, i) => `${i ? "L" : "M"}${x(i).toFixed(1)} ${y(val(p)).toFixed(1)}`).join(" ");
  const area = points.length ? `${line} L${x(points.length - 1).toFixed(1)} ${y(0)} L${x(0).toFixed(1)} ${y(0)} Z` : "";
  const ticks = [0.5, 1].map((f) => Math.round(top * f * 10) / 10);
  const labelEvery = Math.max(1, Math.ceil(points.length / Math.max(plotW / 34, 1)));
  const peak = points.reduce<YearPoint | null>((best, p) => (!best || val(p) > val(best) ? p : best), null);

  return (
    <div ref={ref}>
      <svg width={width} height={H} role="img" aria-label={t("Rata-rata {unit}", { unit })}>
        {ticks.map((v) => (
          <g key={v}>
            <line x1={PAD.left} x2={width - PAD.right} y1={y(v)} y2={y(v)} className="chart-grid" />
            <text x={PAD.left - 6} y={y(v)} dy="0.35em" textAnchor="end" className="chart-axis">{formatDecimal(v)}</text>
          </g>
        ))}
        <line x1={PAD.left} x2={width - PAD.right} y1={y(0)} y2={y(0)} className="chart-baseline" />
        <path d={area} className="chart-area" />
        <path d={line} className="chart-line" />
        {points.map((p, i) => (
          <g key={p.year}>
            <circle cx={x(i)} cy={y(val(p))} r={2.8} className="chart-dot">
              <title>{`${p.year}: ${formatDecimal(val(p), 2)} ${unit}`}</title>
            </circle>
            {i % labelEvery === 0 ? (
              <text x={x(i)} y={H - 6} textAnchor="middle" className="chart-axis">{`'${String(p.year).slice(2)}`}</text>
            ) : null}
          </g>
        ))}
      </svg>
      {peak && val(peak) > 0 ? (
        <div className="muted chart-note">
          {t("Puncak {year} ({value}). Dokumen terbaru wajar masih rendah karena belum banyak waktu untuk disitasi.", { year: peak.year, value: formatDecimal(val(peak)) })}
        </div>
      ) : null}
    </div>
  );
}
