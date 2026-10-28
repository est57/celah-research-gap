import { formatInt } from "../../lib/format";
import type { Trend } from "../../lib/conceptualTypes";
import { t } from "../../i18n";

export function TrendTopics({ trends }: { trends: Trend[] }) {
  if (trends.length === 0) {
    return <div className="chart-empty">{t("Belum ada kata kunci yang cukup sering untuk dihitung trennya.")}</div>;
  }
  const start = Math.min(...trends.map((x) => x.q1));
  const end = Math.max(...trends.map((x) => x.q3), start + 1);
  const span = end - start;
  const pos = (y: number) => 3 + ((y - start) / span) * 94;
  const maxFreq = Math.max(...trends.map((x) => x.freq), 1);
  const step = Math.max(1, Math.ceil(span / 10));
  const ticks = Array.from({ length: Math.floor(span / step) + 1 }, (_, i) => start + i * step);

  return (
    <div className="trends">
      {trends.map((tr) => {
        const size = 6 + 10 * Math.sqrt(tr.freq / maxFreq);
        return (
          <div key={tr.label} className="trend-row" title={t("{label}: Q1 {q1}, median {median}, Q3 {q3}, {n} dokumen", { label: tr.label, q1: tr.q1, median: tr.median, q3: tr.q3, n: formatInt(tr.freq) })}>
            <span className="trend-label">{tr.label}</span>
            <div className="trend-track">
              <div className="trend-range" style={{ left: `${pos(tr.q1)}%`, width: `${Math.max(pos(tr.q3) - pos(tr.q1), 0.8)}%` }} />
              <div className="trend-dot" style={{ left: `${pos(tr.median)}%`, width: size, height: size }} />
            </div>
            <span className="num muted trend-freq">{formatInt(tr.freq)}</span>
          </div>
        );
      })}
      <div className="trend-row trend-axis">
        <span />
        <div className="trend-track">
          {ticks.map((y) => (
            <span key={y} className="num muted" style={{ left: `${pos(y)}%` }}>{y}</span>
          ))}
        </div>
        <span />
      </div>
    </div>
  );
}
