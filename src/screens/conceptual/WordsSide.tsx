import { formatInt } from "../../lib/format";
import { DEFAULT_WORDS, isFreeText, WORD_FIELDS, type WordField, type WordParams, type Words } from "../../lib/wordTypes";
import { t, tk } from "../../i18n";

interface Props {
  params: WordParams;
  data: Words | null;
  onParams: (p: WordParams) => void;
}

const TOPS = [20, 30, 50, 100];
const NGRAMS = [
  { n: 1, label: tk("Satu kata") },
  { n: 2, label: tk("Frasa dua kata") },
  { n: 3, label: tk("Frasa tiga kata") },
];

export function WordsSide({ params, data, onParams }: Props) {
  const free = isFreeText(params.field);
  return (
    <aside className="inspector" aria-label={t("Parameter analisis kata")}>
      <div className="ph">{t("Parameter")}</div>
      <div className="params">
        <div className="param-row">
          <label htmlFor="wd-field">{t("Bidang")}</label>
          <select id="wd-field" className="input" value={params.field}
            onChange={(e) => onParams({ ...params, field: e.target.value as WordField })}>
            {WORD_FIELDS.map((f) => <option key={f.id} value={f.id}>{t(f.label)}</option>)}
          </select>
        </div>
        <div className="param-row">
          <label htmlFor="wd-ngram">{t("Satuan")}</label>
          <select id="wd-ngram" className="input" value={free ? params.ngram : 1} disabled={!free}
            onChange={(e) => onParams({ ...params, ngram: Number(e.target.value) })}>
            {NGRAMS.map((g) => <option key={g.n} value={g.n}>{t(g.label)}</option>)}
          </select>
        </div>
        <div className="param-row">
          <label htmlFor="wd-top">{t("Jumlah kata")}</label>
          <select id="wd-top" className="input" value={params.top} onChange={(e) => onParams({ ...params, top: Number(e.target.value) })}>
            {TOPS.map((n) => <option key={n} value={n}>{n}</option>)}
          </select>
        </div>
        <div className="param-actions">
          <button type="button" className="btn" onClick={() => onParams(DEFAULT_WORDS)}>{t("Atur ulang")}</button>
        </div>
      </div>
      <div className="ph">{t("Ringkasan")}</div>
      {data ? (
        <dl className="words-facts">
          <dt>{t("Dokumen dengan bidang ini")}</dt>
          <dd className="num">{t("{n} dari {total}", { n: formatInt(data.withField), total: formatInt(data.documents) })}</dd>
          <dt>{free ? t("Kata atau frasa unik") : t("Kata kunci unik")}</dt>
          <dd className="num">{formatInt(data.distinct)}</dd>
        </dl>
      ) : null}
      <p className="muted words-hint">
        {free
          ? t("Kata umum (the, of, yang, dan) dan kata generik seperti study atau results diabaikan. Bentuk jamak dihitung bersama bentuk tunggalnya. Frasa tidak menyeberangi tanda baca.")
          : t("Satu dokumen dihitung sekali per kata kunci. Sinonim yang aktif sudah digabungkan.")}
      </p>
    </aside>
  );
}
