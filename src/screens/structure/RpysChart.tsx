import { useWidth } from "../../hooks/useWidth";
import { formatInt } from "../../lib/format";
import type { Rpys } from "../../lib/rpysTypes";
import { t } from "../../i18n";

const H = 300;
const PAD = { left: 40, right: 12, top: 14, bottom: 24 };

interface Props {
  data: Rpys;
  selected: number | null;
  onSelect: (year: number) => void;
}

/** Citations per cited year (bars) and their deviation from the 5-year median (line). */
export function RpysChart({ data, selected, onSelect }: Props) {
  const { ref, width } = useWidth<HTMLDivElement>(800);
  const { years, citations, deviation } = data;
  const peaks = new Set(data.peaks.map((p) => p.year));
  const top = Math.max(1, ...citations);
  const low = Math.min(0, ...deviation);
  const plotW = Math.max(10, width - PAD.left - PAD.right);
  const plotH = H - PAD.top - PAD.bottom;
  const step = plotW / Math.max(1, years.length);
  const x = (i: number) => PAD.left + step * (i + 0.5);
  const y = (v: number) => PAD.top + ((top - v) / (top - low)) * plotH;
  const bar = Math.max(1, Math.min(18, step * 0.7));
  const every = Math.max(1, Math.ceil(years.length / Math.max(1, plotW / 44)));
  const line = deviation.map((v, i) => `${i ? "L" : "M"}${x(i).toFixed(1)} ${y(v).toFixed(1)}`).join(" ");

  return (
    <div ref={ref}>
      <svg width={width} height={H} role="img" aria-label={t("Sitasi per tahun terbit rujukan dan selisihnya dari median lima tahun")}>
        {[0.5, 1].map((f) => (
          <g key={f}>
            <line x1={PAD.left} x2={width - PAD.right} y1={y(top * f)} y2={y(top * f)} className="chart-grid" />
            <text x={PAD.left - 6} y={y(top * f)} dy="0.35em" textAnchor="end" className="chart-axis">{formatInt(Math.round(top * f))}</text>
          </g>
        ))}
        {years.map((yr, i) => (
          <rect key={yr} className="rpys-bar" data-selected={yr === selected} data-peak={peaks.has(yr)}
            x={x(i) - bar / 2} y={y(citations[i])} width={bar} height={Math.max(0, y(0) - y(citations[i]))}
            onClick={() => onSelect(yr)}>
            <title>{t("{year}: {n} sitasi, selisih {dev}", { year: yr, n: formatInt(citations[i]), dev: `${deviation[i] > 0 ? "+" : ""}${formatInt(deviation[i])}` })}</title>
          </rect>
        ))}
        <line x1={PAD.left} x2={width - PAD.right} y1={y(0)} y2={y(0)} className="chart-baseline" />
        <path d={line} className="rpys-line" />
        {years.map((yr, i) => (peaks.has(yr) ? (
          <circle key={yr} cx={x(i)} cy={y(deviation[i])} r={yr === selected ? 5 : 3.5} className="rpys-peak" onClick={() => onSelect(yr)}>
            <title>{t("Puncak {year}: selisih +{dev}", { year: yr, dev: formatInt(deviation[i]) })}</title>
          </circle>
        ) : null))}
        {years.map((yr, i) => (i % every === 0 ? <text key={yr} x={x(i)} y={H - 6} textAnchor="middle" className="chart-axis">{yr}</text> : null))}
      </svg>
      <div className="dyn-legend">
        <span><span className="dyn-swatch rpys-swatch-bar" />{t("Sitasi per tahun terbit rujukan")}</span>
        <span><span className="dyn-swatch rpys-swatch-line" />{t("Selisih dari median 5 tahun")}</span>
        <span><span className="dyn-swatch rpys-swatch-peak" />{t("Puncak")}</span>
      </div>
    </div>
  );
}
