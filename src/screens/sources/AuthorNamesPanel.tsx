import { useState } from "react";
import { useAuthorNames } from "../../hooks/useAuthorNames";
import { formatInt } from "../../lib/format";
import { NameSuggestionCard } from "./NameSuggestionCard";
import { t } from "../../i18n";

const SHOWN = 8;

/** Possible duplicate author names, bulk-merge for reordered names, and existing merges to undo. */
export function AuthorNamesPanel({ version, onChanged }: { version: number; onChanged: () => void }) {
  const { data, busy, actionError, merge, mergeAll, split } = useAuthorNames(version, onChanged);
  const [all, setAll] = useState(false);
  if (!data || (data.suggestions.length === 0 && data.merges.length === 0)) return null;
  const reordered = data.suggestions.filter((s) => s.reordered);
  const shown = all ? data.suggestions : data.suggestions.slice(0, SHOWN);

  return (
    <section className="pnl">
      <div className="ph">
        {t("Satukan nama penulis")}
        <span className="ph-aside">{t("{n} kemungkinan nama ganda", { n: formatInt(data.suggestions.length) })}</span>
      </div>
      <p className="muted ent-note">
        {t("Nama yang mungkin orang yang sama: nama keluarga sama, inisial cocok, dan tidak pernah muncul di paper yang sama. Setelah disatukan, semua analisis menghitungnya sebagai satu penulis; nama di byline dan daftar pustaka tidak berubah.")}
      </p>
      {reordered.length ? (
        <div className="name-bulk">
          <span>{t("{n} kelompok hanya berbeda urutan nama (misalnya \"Antani, Sameer\" dan \"Sameer Antani\").", { n: formatInt(reordered.length) })}</span>
          <button type="button" className="btn btn-primary" disabled={busy}
            onClick={() => void mergeAll(reordered.map((s) => ({ keys: s.variants.map((v) => v.key), target: s.variants[0].key })))}>
            {t("Satukan {n} kelompok ini", { n: formatInt(reordered.length) })}
          </button>
        </div>
      ) : null}
      {actionError ? <p className="delete-error ent-note" role="alert">{actionError}</p> : null}
      <ul className="name-list">
        {shown.map((s) => (
          <NameSuggestionCard key={s.variants.map((v) => v.key).join("|")} s={s} busy={busy} onMerge={(k, target) => void merge(k, target)} />
        ))}
      </ul>
      {data.suggestions.length > SHOWN ? (
        <button type="button" className="btn name-more" onClick={() => setAll(!all)}>
          {all ? t("Tampilkan lebih sedikit") : t("Tampilkan semua ({n})", { n: formatInt(data.suggestions.length) })}
        </button>
      ) : null}
      {data.merges.length ? (
        <div className="name-merges">
          <span className="opt-label">{t("Sudah disatukan")}</span>
          {data.merges.map((m) => (
            <div key={m.fromKey} className="name-merge">
              <span>{t("{from} dihitung sebagai {to}", { from: m.fromName, to: m.toName })}</span>
              <button type="button" className="btn" disabled={busy} onClick={() => void split(m.fromKey)}>{t("Pisahkan")}</button>
            </div>
          ))}
        </div>
      ) : null}
    </section>
  );
}
