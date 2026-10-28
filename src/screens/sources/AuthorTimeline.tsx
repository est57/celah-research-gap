import { formatDecimal, formatInt } from "../../lib/format";
import type { Timeline } from "../../lib/sourcesTypes";
import { t } from "../../i18n";

/** Light to dark accent, by citations per year relative to the busiest cell. */
function shade(t: number): string {
  const mix = (a: number, b: number) => Math.round(a + (b - a) * t);
  return `rgb(${mix(201, 22)}, ${mix(215, 62)}, ${mix(238, 126)})`;
}

export function AuthorTimeline({ rows, currentYear }: { rows: Timeline[]; currentYear: number }) {
  const years = rows.flatMap((r) => r.years.map((c) => c.year));
  if (years.length === 0) return <div className="chart-empty">{t("Belum ada data tahun untuk penulis teratas.")}</div>;
  const start = Math.min(...years);
  const end = Math.max(Math.max(...years), Math.min(currentYear, start + 1));
  const span = Math.max(end - start, 1);
  const pos = (y: number) => `${3 + ((y - start) / span) * 94}%`;
  const maxTc = Math.max(...rows.flatMap((r) => r.years.map((c) => c.tcPerYear)), 1);
  const step = Math.max(1, Math.ceil(span / 10));
  const ticks = Array.from({ length: Math.floor(span / step) + 1 }, (_, i) => start + i * step);

  return (
    <div className="timeline">
      {rows.map((r) => {
        const ys = r.years.map((c) => c.year);
        const first = Math.min(...ys);
        const last = Math.max(...ys);
        return (
          <div key={r.label} className="timeline-row">
            <span className="timeline-name" title={t("{name}: {n} dokumen", { name: r.label, n: formatInt(r.documents) })}>{r.label}</span>
            <div className="timeline-track">
              <div className="timeline-line" style={{ left: pos(first), width: `calc(${pos(last)} - ${pos(first)})` }} />
              {r.years.map((c) => {
                const size = Math.min(6 + c.documents * 4, 28);
                return (
                  <div
                    key={c.year}
                    className="timeline-dot"
                    style={{ left: pos(c.year), width: size, height: size, background: shade(c.tcPerYear / maxTc) }}
                    title={t("{name}, {year}: {n} dokumen, {tc} sitasi per tahun", { name: r.label, year: c.year, n: formatInt(c.documents), tc: formatDecimal(c.tcPerYear) })}
                  />
                );
              })}
            </div>
          </div>
        );
      })}
      <div className="timeline-row timeline-axis">
        <span />
        <div className="timeline-track">
          {ticks.map((y) => (
            <span key={y} className="num muted" style={{ left: pos(y) }}>{y}</span>
          ))}
        </div>
      </div>
    </div>
  );
}
