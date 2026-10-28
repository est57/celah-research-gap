import { useState } from "react";
import type { NameSuggestion } from "../../lib/entityTypes";
import { formatInt } from "../../lib/format";
import { t } from "../../i18n";

interface Props {
  s: NameSuggestion;
  busy: boolean;
  onMerge: (keys: string[], target: string) => void;
}

function evidence(s: NameSuggestion): string {
  const parts = [
    s.reordered ? t("nama sama, urutan berbeda") : null,
    s.sharedCoauthors ? t("{n} rekan penulis sama", { n: formatInt(s.sharedCoauthors) }) : null,
    s.sharedSources ? t("{n} sumber sama", { n: formatInt(s.sharedSources) }) : null,
  ].filter(Boolean);
  return parts.length ? parts.join(" · ") : t("Belum ada bukti: periksa dulu sebelum menyatukan");
}

/** One group of possibly identical names: tick the ones that are one person and pick the name to keep. */
export function NameSuggestionCard({ s, busy, onMerge }: Props) {
  const [picked, setPicked] = useState<Set<string>>(new Set(s.variants.map((v) => v.key)));
  const [target, setTarget] = useState(s.variants[0].key);
  const strong = s.reordered || s.sharedCoauthors > 0;
  const toggle = (key: string) => setPicked((p) => {
    const next = new Set(p);
    if (next.has(key)) next.delete(key); else next.add(key);
    return next;
  });
  const ok = picked.size >= 2 && picked.has(target);

  return (
    <li className="name-card">
      <ul className="name-variants">
        {s.variants.map((v) => (
          <li key={v.key}>
            <label className="name-pick">
              <input type="checkbox" checked={picked.has(v.key)} onChange={() => toggle(v.key)} />
              <span>{v.name}</span>
              <span className="num muted">{t("{n} dok", { n: formatInt(v.documents) })}</span>
            </label>
            <label className="name-target">
              <input type="radio" name={`target-${s.variants[0].key}`} checked={target === v.key} onChange={() => setTarget(v.key)} />
              <span className="muted">{t("jadi nama utama")}</span>
            </label>
          </li>
        ))}
      </ul>
      <div className="name-foot">
        <span className={strong ? "name-evidence" : "name-evidence name-weak"}>{evidence(s)}</span>
        <button type="button" className="btn" disabled={busy || !ok} onClick={() => onMerge([...picked], target)}>{t("Satukan")}</button>
      </div>
    </li>
  );
}
