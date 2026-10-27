import { useState } from "react";
import { formatInt } from "../../lib/format";
import type { KeywordSuggestion } from "../../lib/synonymTypes";

const KIND: Record<KeywordSuggestion["kind"], string> = { ejaan: "Ejaan atau jamak", singkatan: "Singkatan" };

/** One suggested group: tick the keywords that mean the same and pick the one to keep. */
export function SynonymCard({ s, busy, onMerge }: { s: KeywordSuggestion; busy: boolean; onMerge: (labels: string[], target: string) => void }) {
  const [picked, setPicked] = useState<Set<string>>(new Set(s.terms.map((t) => t.label)));
  const [target, setTarget] = useState(s.terms[0].label);
  const id = s.terms.map((t) => t.label).join("|");
  const toggle = (label: string) => setPicked((p) => {
    const next = new Set(p);
    if (next.has(label)) next.delete(label); else next.add(label);
    return next;
  });

  return (
    <li className="name-card">
      <span className="syn-kind" data-kind={s.kind}>{KIND[s.kind]}</span>
      <ul className="name-variants">
        {s.terms.map((t) => (
          <li key={t.label}>
            <label className="name-pick">
              <input type="checkbox" checked={picked.has(t.label)} onChange={() => toggle(t.label)} />
              <span>{t.label}</span>
              <span className="num muted">{`${formatInt(t.documents)} dok`}</span>
            </label>
            <label className="name-target">
              <input type="radio" name={`syn-${id}`} checked={target === t.label} onChange={() => setTarget(t.label)} />
              <span className="muted">dipakai</span>
            </label>
          </li>
        ))}
      </ul>
      <div className="name-foot">
        <span className="muted name-evidence-plain">{s.kind === "singkatan" ? "Periksa bahwa singkatan ini memang berarti istilah itu di bidang Anda." : "Bentuk tulisan berbeda dari istilah yang sama."}</span>
        <button type="button" className="btn" disabled={busy || picked.size < 2 || !picked.has(target)} onClick={() => onMerge([...picked], target)}>Satukan</button>
      </div>
    </li>
  );
}
