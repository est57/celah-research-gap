import { formatPct, quality as grade } from "../../lib/format";
import type { MetadataQuality } from "../../lib/qualityTypes";
import { t } from "../../i18n";

/** Severity bucket for the cell tint, from the share of missing documents. */
function level(missing: number, total: number): number {
  if (total === 0 || missing === 0) return 0;
  const share = missing / total;
  return share < 0.1 ? 1 : share < 0.2 ? 2 : share < 0.5 ? 3 : 4;
}

export function FieldMatrix({ quality }: { quality: MetadataQuality }) {
  const totals = quality.fields.map((_, i) => quality.origins.reduce((s, o) => s + o.missing[i], 0));
  return (
    <section className="pnl">
      <div className="ph">
        {t("Field kosong per sumber impor")}
        <span className="ph-aside">{t("persentase dokumen tanpa field tersebut")}</span>
      </div>
      <div className="fmx-scroll">
        <table className="fmx">
          <thead>
            <tr>
              <th scope="col">{t("Field")}</th>
              <th scope="col" className="fmx-total">{t("Semua")}</th>
              {quality.origins.map((o) => (
                <th scope="col" key={o.origin}>
                  {o.origin}
                  <span className="num muted fmx-n">{o.count}</span>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {quality.fields.map((f, i) => (
              <tr key={f.tag}>
                <th scope="row">
                  <span className="num fmx-tag">{f.tag}</span>
                  {f.label}
                </th>
                <td className="fmx-total" data-level={level(totals[i], quality.total)}
                  title={grade(totals[i], quality.total).label}>
                  {formatPct(totals[i], quality.total)}
                </td>
                {quality.origins.map((o) => (
                  <td key={o.origin} data-level={level(o.missing[i], o.count)}>{formatPct(o.missing[i], o.count)}</td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}
