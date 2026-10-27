import { useState } from "react";
import { formatDecimal, formatInt } from "../../lib/format";
import type { Overview } from "../../lib/overviewTypes";
import { CitationChart, type CitationMetric } from "./CitationChart";
import { YearTable } from "./YearTable";
import { ChartExport } from "../../components/ChartExport";

const HELP: Record<CitationMetric, string> = {
  perYear: "Rata-rata sitasi per artikel dibagi jumlah tahun artikel itu bisa disitasi, sehingga tahun lama dan baru sebanding.",
  perArticle: "Rata-rata total sitasi artikel yang terbit pada tahun itu. Tahun lama cenderung tinggi karena sudah lebih lama beredar.",
};

export function CitationsView({ data, sample }: { data: Overview; sample: boolean }) {
  const [metric, setMetric] = useState<CitationMetric>("perYear");
  const points = data.annual;
  return (
    <div className="year-page">
      {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
      <section className="pnl">
        <div className="ph">
          Rata-rata sitasi
          <div className="opt-seg year-toggle" role="group" aria-label="Ukuran">
            <button type="button" aria-pressed={metric === "perYear"} onClick={() => setMetric("perYear")}>Per tahun</button>
            <button type="button" aria-pressed={metric === "perArticle"} onClick={() => setMetric("perArticle")}>Per artikel</button>
          </div>
          <ChartExport name={metric === "perYear" ? "Sitasi per tahun" : "Sitasi per artikel"} />
        </div>
        <div className="pnl-body">
          <CitationChart points={points} metric={metric} height={280} />
          <p className="muted year-note">{HELP[metric]}</p>
        </div>
      </section>
      <section className="pnl">
        <div className="ph">Rincian per tahun</div>
        <YearTable label="Sitasi per tahun" years={points.map((p) => p.year)} columns={[
          { label: "Dokumen", cell: (i) => formatInt(points[i].documents) },
          { label: "Sitasi per artikel", cell: (i) => formatDecimal(points[i].meanTc, 2) },
          { label: "Tahun dapat disitasi", cell: (i) => formatInt(points[i].citableYears), wide: true },
          { label: "Sitasi per tahun", cell: (i) => formatDecimal(points[i].meanTcPerYear, 2) },
        ]} />
      </section>
    </div>
  );
}
