import { useEffect, useState, type FormEvent } from "react";
import { DEFAULT_EVOLUTION, type EvolutionParams } from "../../lib/evolutionTypes";
import { t } from "../../i18n";

interface Props {
  applied: EvolutionParams;
  usedCuts: number[];
  busy: boolean;
  minInclusion: number;
  onMinInclusion: (v: number) => void;
  onApply: (p: EvolutionParams) => void;
}

const INCLUSION = [0.1, 0.2, 0.3, 0.5];
const clamp = (v: number, min: number, max: number) => Math.min(max, Math.max(min, Math.round(v) || min));

/** "2019, 2022" -> [2019, 2022]; null when the text is not 0-3 ascending years. */
function parseCuts(text: string): number[] | null {
  const parts = text.split(/[,\s]+/).filter(Boolean);
  const years = parts.map(Number);
  const ok = years.length <= 3 && years.every((y, i) => Number.isInteger(y) && y >= 1900 && y <= 2100 && (i === 0 || y > years[i - 1]));
  return ok ? years : null;
}

export function EvolutionForm({ applied, usedCuts, busy, minInclusion, onMinInclusion, onApply }: Props) {
  const [cuts, setCuts] = useState(applied.cuts.join(", "));
  const [words, setWords] = useState(applied.words);
  const [labels, setLabels] = useState(applied.labels);
  const [error, setError] = useState<string | null>(null);
  useEffect(() => setCuts(applied.cuts.join(", ")), [applied.cuts]);

  const submit = (e: FormEvent) => {
    e.preventDefault();
    const parsed = parseCuts(cuts);
    if (!parsed) {
      setError(t("Isi 1 sampai 3 tahun berurutan, misalnya 2019, 2022. Kosongkan untuk otomatis."));
      return;
    }
    setError(null);
    onApply({ ...applied, cuts: parsed, words: clamp(words, 20, 1000), labels: clamp(labels, 1, 5) });
  };

  return (
    <form className="params" onSubmit={submit}>
      <div className="param-row">
        <label htmlFor="evo-cuts">{t("Tahun potong")}</label>
        <input id="evo-cuts" className="input" maxLength={20} value={cuts} placeholder={usedCuts.join(", ") || t("otomatis")}
          onChange={(e) => setCuts(e.target.value)} />
      </div>
      <p className="muted evo-hint">
        {applied.cuts.length ? t("Tahun potong ikut periode sebelumnya.") : t("Otomatis: {cuts}, agar jumlah dokumen tiap periode seimbang.", { cuts: usedCuts.join(", ") || t("satu periode") })}
      </p>
      {error ? <p className="evo-error" role="alert">{error}</p> : null}
      <div className="param-row">
        <label htmlFor="evo-words">{t("Kata per periode")}</label>
        <input id="evo-words" className="input num" type="number" min={20} max={1000} value={words} onChange={(e) => setWords(Number(e.target.value))} />
      </div>
      <div className="param-row">
        <label htmlFor="evo-labels">{t("Label per tema")}</label>
        <input id="evo-labels" className="input num" type="number" min={1} max={5} value={labels} onChange={(e) => setLabels(Number(e.target.value))} />
      </div>
      <div className="param-row">
        <label htmlFor="evo-inc">{t("Inklusi minimum")}</label>
        <select id="evo-inc" className="input" value={minInclusion} onChange={(e) => onMinInclusion(Number(e.target.value))}>
          {INCLUSION.map((v) => <option key={v} value={v}>{`${Math.round(v * 100)}%`}</option>)}
        </select>
      </div>
      <div className="param-actions">
        <button type="button" className="btn" onClick={() => { setCuts(""); setWords(DEFAULT_EVOLUTION.words); setLabels(DEFAULT_EVOLUTION.labels); }}>{t("Atur ulang")}</button>
        <button type="submit" className="btn btn-primary" disabled={busy}>{t("Terapkan")}</button>
      </div>
    </form>
  );
}
