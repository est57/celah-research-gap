import { t } from "../../i18n";
import { formatInt } from "../../lib/format";
import type { YearPoint } from "../../lib/overviewTypes";

/** Rounds the axis top up to a 1, 2 or 5 step so gridlines land on readable values. */
function niceMax(max: number): number {
  if (max <= 0) return 1;
  const pow = 10 ** Math.floor(Math.log10(max));
  const step = [1, 2, 5, 10].find((s) => s * pow >= max) ?? 10;
  return step * pow;
}

export function AnnualChart({ points, height = 150 }: { points: YearPoint[]; height?: number }) {
  const BAR_AREA = height;
  const top = niceMax(Math.max(...points.map((p) => p.documents), 0));
  const grid = [0.25, 0.5, 0.75, 1].map((f) => Math.round(top * f));
  const dense = points.length > 16;

  return (
    <div className="annual">
      <div className="annual-plot" style={{ height: BAR_AREA + 18 }}>
        {grid.map((g) => (
          <div key={g} className="gridline num" style={{ bottom: (g / top) * BAR_AREA }}>
            {formatInt(g)}
          </div>
        ))}
        {points.map((p) => (
          <div key={p.year} className="annual-col" title={t("{year}: {n} dokumen", { year: p.year, n: formatInt(p.documents) })}>
            {!dense ? <span className="num annual-value">{formatInt(p.documents)}</span> : null}
            <div
              className="annual-bar"
              data-partial={p.partial}
              style={{ height: Math.max((p.documents / top) * BAR_AREA, p.documents > 0 ? 2 : 0) }}
            />
          </div>
        ))}
      </div>
      <div className="annual-years">
        {points.map((p, i) => (
          <span key={p.year} className="num">
            {!dense || i % Math.ceil(points.length / 12) === 0 ? `'${String(p.year).slice(2)}` : ""}
          </span>
        ))}
      </div>
      {points.some((p) => p.partial) ? (
        <div className="muted chart-note">{t("Tahun berjalan berarsir: datanya belum satu tahun penuh.")}</div>
      ) : null}
    </div>
  );
}
