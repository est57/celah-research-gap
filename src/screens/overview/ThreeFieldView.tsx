import { useState, type ReactNode } from "react";
import { ScreenMessage } from "../../components/ScreenMessage";
import { useThreeField } from "../../hooks/useThreeField";
import { THREE_FIELD_LABEL, type ThreeFieldId, type ThreeFieldParams } from "../../lib/overviewTypes";
import { ThreeFieldPlot } from "./ThreeFieldPlot";
import { ChartExport } from "../../components/ChartExport";
import { t, tk } from "../../i18n";

const FIELDS = Object.keys(THREE_FIELD_LABEL) as ThreeFieldId[];
const ITEMS = [5, 8, 10, 15, 20, 30];
const DEFAULT: ThreeFieldParams = { left: "authors", middle: "author_keywords", right: "sources", items: 10 };
const COLS: ["left" | "middle" | "right", string][] = [["left", tk("Kolom kiri")], ["middle", tk("Kolom tengah")], ["right", tk("Kolom kanan")]];

export function ThreeFieldView({ version }: { version: number }) {
  const [p, setP] = useState<ThreeFieldParams>(DEFAULT);
  const { data, sample, error, loading } = useThreeField(version, p);
  // Picking a field already used elsewhere swaps the two columns, so all three stay distinct.
  const pick = (col: "left" | "middle" | "right", f: ThreeFieldId) => {
    const other = COLS.map(([c]) => c).find((c) => c !== col && p[c] === f);
    setP(other ? { ...p, [col]: f, [other]: p[col] } : { ...p, [col]: f });
  };

  let body: ReactNode;
  if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? t("Menyusun plot tiga bidang...") : ""}</ScreenMessage>;
  else body = <div className="pnl-body"><ThreeFieldPlot data={data} height={Math.max(320, p.items * 24)} /></div>;

  return (
    <div className="year-page">
      {sample ? <div className="notice">{t("Pratinjau browser: plot di halaman ini adalah data contoh dan tidak berubah mengikuti pengaturan.")}</div> : null}
      <section className="pnl">
        <div className="tf-settings">
          {COLS.map(([col, label]) => (
            <div key={col} className="tf-field">
              <label htmlFor={`tf-${col}`}>{t(label)}</label>
              <select id={`tf-${col}`} className="input" value={p[col]} onChange={(e) => pick(col, e.target.value as ThreeFieldId)}>
                {FIELDS.map((f) => <option key={f} value={f}>{t(THREE_FIELD_LABEL[f])}</option>)}
              </select>
            </div>
          ))}
          <div className="tf-field">
            <label htmlFor="tf-items">{t("Item per kolom")}</label>
            <select id="tf-items" className="input" value={p.items} onChange={(e) => setP({ ...p, items: Number(e.target.value) })}>
              {ITEMS.map((n) => <option key={n} value={n}>{n}</option>)}
            </select>
          </div>
          <button type="button" className="btn tf-reset" onClick={() => setP(DEFAULT)}>{t("Atur ulang")}</button>
        </div>
      </section>
      <section className="pnl">
        <div className="ph">
          {t("Plot tiga bidang")}
          <span className="ph-aside">{t("tebal aliran = jumlah dokumen yang memuat kedua item")}</span>
          <ChartExport name={t("Plot tiga bidang")} />
        </div>
        {body}
      </section>
    </div>
  );
}
