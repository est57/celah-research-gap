import { formatInt } from "../../lib/format";
import { DEFAULT_WORDS, isFreeText, WORD_FIELDS, type WordField, type WordParams, type Words } from "../../lib/wordTypes";

interface Props {
  params: WordParams;
  data: Words | null;
  onParams: (p: WordParams) => void;
}

const TOPS = [20, 30, 50, 100];
const NGRAMS = [
  { n: 1, label: "Satu kata" },
  { n: 2, label: "Frasa dua kata" },
  { n: 3, label: "Frasa tiga kata" },
];

export function WordsSide({ params, data, onParams }: Props) {
  const free = isFreeText(params.field);
  return (
    <aside className="inspector" aria-label="Parameter analisis kata">
      <div className="ph">Parameter</div>
      <div className="params">
        <div className="param-row">
          <label htmlFor="wd-field">Bidang</label>
          <select id="wd-field" className="input" value={params.field}
            onChange={(e) => onParams({ ...params, field: e.target.value as WordField })}>
            {WORD_FIELDS.map((f) => <option key={f.id} value={f.id}>{f.label}</option>)}
          </select>
        </div>
        <div className="param-row">
          <label htmlFor="wd-ngram">Satuan</label>
          <select id="wd-ngram" className="input" value={free ? params.ngram : 1} disabled={!free}
            onChange={(e) => onParams({ ...params, ngram: Number(e.target.value) })}>
            {NGRAMS.map((g) => <option key={g.n} value={g.n}>{g.label}</option>)}
          </select>
        </div>
        <div className="param-row">
          <label htmlFor="wd-top">Jumlah kata</label>
          <select id="wd-top" className="input" value={params.top} onChange={(e) => onParams({ ...params, top: Number(e.target.value) })}>
            {TOPS.map((n) => <option key={n} value={n}>{n}</option>)}
          </select>
        </div>
        <div className="param-actions">
          <button type="button" className="btn" onClick={() => onParams(DEFAULT_WORDS)}>Atur ulang</button>
        </div>
      </div>
      <div className="ph">Ringkasan</div>
      {data ? (
        <dl className="words-facts">
          <dt>Dokumen dengan bidang ini</dt>
          <dd className="num">{`${formatInt(data.withField)} dari ${formatInt(data.documents)}`}</dd>
          <dt>{free ? "Kata atau frasa unik" : "Kata kunci unik"}</dt>
          <dd className="num">{formatInt(data.distinct)}</dd>
        </dl>
      ) : null}
      <p className="muted words-hint">
        {free
          ? "Kata umum (the, of, yang, dan) dan kata generik seperti study atau results diabaikan. Bentuk jamak dihitung bersama bentuk tunggalnya. Frasa tidak menyeberangi tanda baca."
          : "Satu dokumen dihitung sekali per kata kunci. Sinonim yang aktif sudah digabungkan."}
      </p>
    </aside>
  );
}
