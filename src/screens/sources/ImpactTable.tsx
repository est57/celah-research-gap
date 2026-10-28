import { formatDecimal, formatInt } from "../../lib/format";
import type { Impact } from "../../lib/sourcesTypes";
import { t } from "../../i18n";

export function ImpactTable({ rows }: { rows: Impact[] }) {
  return (
    <div className="impact-wrap">
      <table className="impact">
        <thead>
          <tr>
            <th scope="col">{t("Penulis")}</th>
            <th scope="col" title={t("Indeks h")}>h</th>
            <th scope="col" title={t("Indeks g")}>g</th>
            <th scope="col" title={t("Indeks m: h dibagi tahun sejak publikasi pertama")}>m</th>
            <th scope="col" title={t("Total sitasi")}>TC</th>
            <th scope="col" title={t("Jumlah dokumen di koleksi")}>{"NP"}</th>
            <th scope="col" title={t("Tahun publikasi pertama")}>{t("Mulai")}</th>
          </tr>
        </thead>
        <tbody>
          {rows.map((r) => (
            <tr key={r.label}>
              <th scope="row" title={r.label}>{r.label}</th>
              <td className="num strong">{r.h}</td>
              <td className="num">{r.g}</td>
              <td className="num">{r.m === null ? "-" : formatDecimal(r.m, 2)}</td>
              <td className="num">{formatInt(r.citations)}</td>
              <td className="num">{formatInt(r.documents)}</td>
              <td className="num">{r.firstYear ?? "-"}</td>
            </tr>
          ))}
        </tbody>
      </table>
      <p className="muted chart-note">{t("TC = total sitasi, NP = jumlah dokumen dalam koleksi.")}</p>
    </div>
  );
}
