import { useMemo, useState } from "react";
import { formatDecimal, formatInt, formatPct } from "../../lib/format";
import type { Overview } from "../../lib/overviewTypes";
import { AnnualChart } from "./AnnualChart";
import { YearTable } from "./YearTable";

export function AnnualView({ data, sample }: { data: Overview; sample: boolean }) {
  const [cumulative, setCumulative] = useState(false);
  const points = data.annual;
  const running = useMemo(() => points.reduce<number[]>((acc, p) => [...acc, (acc[acc.length - 1] ?? 0) + p.documents], []), [points]);
  const shown = cumulative ? points.map((p, i) => ({ ...p, documents: running[i] })) : points;
  const total = running[running.length - 1] ?? 0;
  const change = (i: number) => {
    if (points[i].partial) return "berjalan";
    const prev = points[i - 1]?.documents ?? 0;
    if (i === 0 || prev === 0) return "-";
    const pct = ((points[i].documents - prev) / prev) * 100;
    return `${pct > 0 ? "+" : ""}${formatDecimal(pct, 0)}%`;
  };

  return (
    <div className="year-page">
      {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
      <section className="pnl">
        <div className="ph">
          Produksi ilmiah tahunan
          <div className="opt-seg year-toggle" role="group" aria-label="Tampilan">
            <button type="button" aria-pressed={!cumulative} onClick={() => setCumulative(false)}>Per tahun</button>
            <button type="button" aria-pressed={cumulative} onClick={() => setCumulative(true)}>Kumulatif</button>
          </div>
          {data.info.growthRate !== null ? (
            <span className="ph-aside">Laju pertumbuhan tahunan <b className="num">{formatDecimal(data.info.growthRate)}%</b></span>
          ) : null}
        </div>
        <AnnualChart points={shown} height={260} />
        <p className="muted year-note">Laju pertumbuhan dihitung dari tahun lengkap pertama dan terakhir; tahun berjalan tidak ikut dihitung.</p>
      </section>
      <section className="pnl">
        <div className="ph">Rincian per tahun</div>
        <YearTable label="Produksi per tahun" years={points.map((p) => p.year)} columns={[
          { label: "Dokumen", cell: (i) => formatInt(points[i].documents) },
          { label: "Kumulatif", cell: (i) => formatInt(running[i]) },
          { label: "Porsi", cell: (i) => formatPct(points[i].documents, total), wide: true },
          { label: "Perubahan", cell: change },
        ]} />
      </section>
    </div>
  );
}
