import { useState } from "react";
import { formatInt } from "../../lib/format";
import type { KeywordSuggestion } from "../../lib/synonymTypes";
import { t, tk } from "../../i18n";

const KIND: Record<KeywordSuggestion["kind"], string> = { ejaan: tk("Ejaan atau jamak"), singkatan: tk("Singkatan") };

/** One suggested group: tick the keywords that mean the same and pick the one to keep. */
export function SynonymCard({ s, busy, onMerge }: { s: KeywordSuggestion; busy: boolean; onMerge: (labels: string[], target: string) => void }) {
  const [picked, setPicked] = useState<Set<string>>(new Set(s.terms.map((x) => x.label)));
  const [target, setTarget] = useState(s.terms[0].label);
  const id = s.terms.map((x) => x.label).join("|");
  const toggle = (label: string) => setPicked((p) => {
    const next = new Set(p);
    if (next.has(label)) next.delete(label); else next.add(label);
    return next;
  });

  return (
    <li className="name-card">
      <span className="syn-kind" data-kind={s.kind}>{t(KIND[s.kind])}</span>
      <ul className="name-variants">
        {s.terms.map((term) => (
          <li key={term.label}>
            <label className="name-pick">
              <input type="checkbox" checked={picked.has(term.label)} onChange={() => toggle(term.label)} />
              <span>{term.label}</span>
              <span className="num muted">{t("{n} dok", { n: formatInt(term.documents) })}</span>
            </label>
            <label className="name-target">
              <input type="radio" name={`syn-${id}`} checked={target === term.label} onChange={() => setTarget(term.label)} />
              <span className="muted">{t("dipakai")}</span>
            </label>
          </li>
        ))}
      </ul>
      <div className="name-foot">
        <span className="muted name-evidence-plain">{s.kind === "singkatan" ? t("Periksa bahwa singkatan ini memang berarti istilah itu di bidang Anda.") : t("Bentuk tulisan berbeda dari istilah yang sama.")}</span>
        <button type="button" className="btn" disabled={busy || picked.size < 2 || !picked.has(target)} onClick={() => onMerge([...picked], target)}>{t("Satukan")}</button>
      </div>
    </li>
  );
}
