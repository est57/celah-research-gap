import { useEffect, useState, type FormEvent } from "react";
import { DEFAULT_PARAMS, type ConceptualParams, type FieldChoice } from "../../lib/conceptualTypes";

interface Props {
  applied: ConceptualParams;
  busy: boolean;
  onApply: (p: ConceptualParams) => void;
}

const clamp = (v: number, min: number, max: number) => Math.min(max, Math.max(min, Math.round(v) || min));

export function ParamsForm({ applied, busy, onApply }: Props) {
  const [draft, setDraft] = useState(applied);
  useEffect(() => setDraft(applied), [applied]);
  const set = <K extends keyof ConceptualParams>(k: K, v: ConceptualParams[K]) => setDraft((d) => ({ ...d, [k]: v }));

  const submit = (e: FormEvent) => {
    e.preventDefault();
    onApply({
      field: draft.field,
      words: clamp(draft.words, 20, 1000),
      minClusterFreq: clamp(draft.minClusterFreq, 0, 100),
      labels: clamp(draft.labels, 1, 5),
      trendMinFreq: clamp(draft.trendMinFreq, 1, 100),
    });
  };

  const num = (id: string, label: string, k: "words" | "minClusterFreq" | "labels" | "trendMinFreq", min: number, max: number) => (
    <div className="param-row">
      <label htmlFor={id}>{label}</label>
      <input id={id} className="input num" type="number" min={min} max={max} value={draft[k]}
        onChange={(e) => set(k, Number(e.target.value))} />
    </div>
  );

  return (
    <form className="params" onSubmit={submit}>
      <div className="param-row">
        <label htmlFor="p-field">Field</label>
        <select id="p-field" className="input" value={draft.field} onChange={(e) => set("field", e.target.value as FieldChoice)}>
          <option value="auto">Otomatis</option>
          <option value="author">Kata kunci penulis</option>
          <option value="plus">Keywords Plus</option>
        </select>
      </div>
      {num("p-words", "Jumlah kata", "words", 20, 1000)}
      {num("p-min", "Frek. min. tema (per 1.000 dok)", "minClusterFreq", 0, 100)}
      {num("p-labels", "Label per tema", "labels", 1, 5)}
      {num("p-trend", "Frek. min. tren", "trendMinFreq", 1, 100)}
      <div className="param-row">
        <span>Algoritma</span>
        <span className="muted">Louvain</span>
      </div>
      <div className="param-actions">
        <button type="button" className="btn" onClick={() => setDraft(DEFAULT_PARAMS)}>Atur ulang</button>
        <button type="submit" className="btn btn-primary" disabled={busy}>Terapkan</button>
      </div>
    </form>
  );
}
