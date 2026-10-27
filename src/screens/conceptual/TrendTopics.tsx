import { formatInt } from "../../lib/format";
import type { Trend } from "../../lib/conceptualTypes";

export function TrendTopics({ trends }: { trends: Trend[] }) {
  if (trends.length === 0) {
    return <div className="chart-empty">Belum ada kata kunci yang cukup sering untuk dihitung trennya.</div>;
  }
  const start = Math.min(...trends.map((t) => t.q1));
  const end = Math.max(...trends.map((t) => t.q3), start + 1);
  const span = end - start;
  const pos = (y: number) => 3 + ((y - start) / span) * 94;
  const maxFreq = Math.max(...trends.map((t) => t.freq), 1);
  const step = Math.max(1, Math.ceil(span / 10));
  const ticks = Array.from({ length: Math.floor(span / step) + 1 }, (_, i) => start + i * step);

  return (
    <div className="trends">
      {trends.map((t) => {
        const size = 6 + 10 * Math.sqrt(t.freq / maxFreq);
        return (
          <div key={t.label} className="trend-row" title={`${t.label}: Q1 ${t.q1}, median ${t.median}, Q3 ${t.q3}, ${formatInt(t.freq)} dokumen`}>
            <span className="trend-label">{t.label}</span>
            <div className="trend-track">
              <div className="trend-range" style={{ left: `${pos(t.q1)}%`, width: `${Math.max(pos(t.q3) - pos(t.q1), 0.8)}%` }} />
              <div className="trend-dot" style={{ left: `${pos(t.median)}%`, width: size, height: size }} />
            </div>
            <span className="num muted trend-freq">{formatInt(t.freq)}</span>
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
