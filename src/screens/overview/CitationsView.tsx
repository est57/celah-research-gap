import { useState } from "react";
import { formatDecimal, formatInt } from "../../lib/format";
import type { Overview } from "../../lib/overviewTypes";
import { CitationChart, type CitationMetric } from "./CitationChart";
import { YearTable } from "./YearTable";
import { ChartExport } from "../../components/ChartExport";
import { t, tk } from "../../i18n";

const HELP: Record<CitationMetric, string> = {
  perYear: tk("Rata-rata sitasi per artikel dibagi jumlah tahun artikel itu bisa disitasi, sehingga tahun lama dan baru sebanding."),
  perArticle: tk("Rata-rata total sitasi artikel yang terbit pada tahun itu. Tahun lama cenderung tinggi karena sudah lebih lama beredar."),
};

export function CitationsView({ data, sample }: { data: Overview; sample: boolean }) {
  const [metric, setMetric] = useState<CitationMetric>("perYear");
  const points = data.annual;
  return (
    <div className="year-page">
      {sample ? <div className="notice">{t("Pratinjau browser: angka di halaman ini adalah data contoh.")}</div> : null}
      <section className="pnl">
        <div className="ph">
          {t("Rata-rata sitasi")}
          <div className="opt-seg year-toggle" role="group" aria-label={t("Ukuran")}>
            <button type="button" aria-pressed={metric === "perYear"} onClick={() => setMetric("perYear")}>{t("Per tahun")}</button>
            <button type="button" aria-pressed={metric === "perArticle"} onClick={() => setMetric("perArticle")}>{t("Per artikel")}</button>
          </div>
          <ChartExport name={metric === "perYear" ? t("Sitasi per tahun") : t("Sitasi per artikel")} />
        </div>
        <div className="pnl-body">
          <CitationChart points={points} metric={metric} height={280} />
          <p className="muted year-note">{t(HELP[metric])}</p>
        </div>
      </section>
      <section className="pnl">
        <div className="ph">{t("Rincian per tahun")}</div>
        <YearTable label={t("Sitasi per tahun")} years={points.map((p) => p.year)} columns={[
          { label: t("Dokumen"), cell: (i) => formatInt(points[i].documents) },
          { label: t("Sitasi per artikel"), cell: (i) => formatDecimal(points[i].meanTc, 2) },
          { label: t("Tahun dapat disitasi"), cell: (i) => formatInt(points[i].citableYears), wide: true },
          { label: t("Sitasi per tahun"), cell: (i) => formatDecimal(points[i].meanTcPerYear, 2) },
        ]} />
      </section>
    </div>
  );
}
