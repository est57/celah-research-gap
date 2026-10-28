import { clusterColor } from "../../lib/structureTypes";
import { DEFAULT_FACTORIAL, type Factorial, type FactorialParams } from "../../lib/factorialTypes";
import { t } from "../../i18n";

interface Props {
  params: FactorialParams;
  data: Factorial | null;
  focus: number | null;
  onParams: (p: FactorialParams) => void;
  onFocus: (cluster: number | null) => void;
}

const WORDS = [30, 50, 75, 100];
const CLUSTERS = [0, 2, 3, 4, 5, 6, 8];

export function FactorialSide({ params, data, focus, onParams, onFocus }: Props) {
  return (
    <aside className="inspector" aria-label={t("Parameter dan klaster")}>
      <div className="ph">{t("Parameter")}</div>
      <div className="params">
        <div className="param-row">
          <label htmlFor="fa-field">{t("Field")}</label>
          <select id="fa-field" className="input" value={params.field} onChange={(e) => onParams({ ...params, field: e.target.value as FactorialParams["field"] })}>
            <option value="auto">{t("Otomatis")}</option>
            <option value="author">{t("Kata kunci penulis")}</option>
            <option value="plus">{t("Keywords Plus")}</option>
          </select>
        </div>
        <div className="param-row">
          <label htmlFor="fa-words">{t("Jumlah kata")}</label>
          <select id="fa-words" className="input" value={params.words} onChange={(e) => onParams({ ...params, words: Number(e.target.value) })}>
            {WORDS.map((n) => <option key={n} value={n}>{n}</option>)}
          </select>
        </div>
        <div className="param-row">
          <label htmlFor="fa-k">{t("Jumlah klaster")}</label>
          <select id="fa-k" className="input" value={params.clusters} onChange={(e) => onParams({ ...params, clusters: Number(e.target.value) })}>
            {CLUSTERS.map((n) => <option key={n} value={n}>{n === 0 ? t("Otomatis") : n}</option>)}
          </select>
        </div>
        <div className="param-row"><span>{t("Metode")}</span><span className="muted">{t("MCA, klaster Ward")}</span></div>
        <div className="param-actions">
          <button type="button" className="btn" onClick={() => onParams(DEFAULT_FACTORIAL)}>{t("Atur ulang")}</button>
        </div>
      </div>
      <div className="ph">{t("Klaster")}</div>
      <ul className="fa-clusters">
        {(data?.clusters ?? []).map((c) => (
          <li key={c.id}>
            <button type="button" aria-pressed={focus === c.id} onClick={() => onFocus(focus === c.id ? null : c.id)}>
              <span className="theme-swatch" style={{ background: clusterColor(c.id) }} />
              <span className="fa-cluster-top">{c.top.join(", ")}</span>
              <span className="num muted">{c.size}</span>
            </button>
          </li>
        ))}
      </ul>
      <p className="muted fa-hint">
        {t("Kata yang berdekatan sering muncul di dokumen yang sama. Klaster yang berjauhan adalah aliran riset yang jarang bersinggungan; kata kunci di antara dua klaster bisa menjadi jembatan atau celah riset.")}
      </p>
    </aside>
  );
}
