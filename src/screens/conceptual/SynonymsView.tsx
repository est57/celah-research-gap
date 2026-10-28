import { useState, type FormEvent } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useSynonyms } from "../../hooks/useSynonyms";
import { formatInt } from "../../lib/format";
import { SynonymCard } from "./SynonymCard";
import { t } from "../../i18n";

const SHOWN = 9;

export function SynonymsView({ version, onDataChanged }: { version: number; onDataChanged: () => void }) {
  const { data, sample, error, loading, busy, actionError, merge, remove } = useSynonyms(version, onDataChanged);
  const [all, setAll] = useState(false);
  const [from, setFrom] = useState("");
  const [to, setTo] = useState("");
  if (error) return <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  if (!data) return <ScreenMessage>{loading ? t("Mencari kata kunci yang bermakna sama...") : ""}</ScreenMessage>;
  if (data.terms.length === 0) return <EmptyCollection />;
  const manual = (e: FormEvent) => {
    e.preventDefault();
    if (from.trim() && to.trim()) void merge([from.trim(), to.trim()], to.trim()).then(() => { setFrom(""); setTo(""); });
  };
  const shown = all ? data.suggestions : data.suggestions.slice(0, SHOWN);

  return (
    <div className="year-page">
      {sample ? <div className="notice">{t("Pratinjau browser: kata kunci di halaman ini adalah data contoh.")}</div> : null}
      <section className="pnl">
        <div className="ph">{t("Sinonim kata kunci")}<span className="ph-aside">{t("{n} saran", { n: formatInt(data.suggestions.length) })}</span></div>
        <p className="muted ent-note">
          {t("Kata kunci yang disatukan dihitung sebagai satu istilah di peta tematik, jaringan, evolusi, analisis faktorial, plot tiga bidang, dan pencarian celah, sehingga celah palsu antara dua sinonim hilang. Teks kata kunci asli tidak diubah.")}
        </p>
        {actionError ? <p className="delete-error ent-note" role="alert">{actionError}</p> : null}
        <ul className="name-list">
          {shown.map((s) => <SynonymCard key={s.terms.map((x) => x.label).join("|")} s={s} busy={busy} onMerge={(l, target) => void merge(l, target)} />)}
        </ul>
        {data.suggestions.length > SHOWN ? (
          <button type="button" className="btn name-more" onClick={() => setAll(!all)}>
            {all ? t("Tampilkan lebih sedikit") : t("Tampilkan semua ({n})", { n: formatInt(data.suggestions.length) })}
          </button>
        ) : null}
        <form className="syn-manual" onSubmit={manual}>
          <span className="opt-label">{t("Satukan manual")}</span>
          <label htmlFor="syn-from" className="sr-only">{t("Kata kunci")}</label>
          <input id="syn-from" className="input" list="syn-terms" maxLength={120} placeholder={t("Kata kunci, misalnya TB")} value={from} onChange={(e) => setFrom(e.target.value)} />
          <span className="muted">{t("dihitung sebagai")}</span>
          <label htmlFor="syn-to" className="sr-only">{t("Kata kunci tujuan")}</label>
          <input id="syn-to" className="input" list="syn-terms" maxLength={120} placeholder={t("misalnya tuberculosis")} value={to} onChange={(e) => setTo(e.target.value)} />
          <button type="submit" className="btn" disabled={busy || !from.trim() || !to.trim()}>{t("Satukan")}</button>
          <datalist id="syn-terms">{data.terms.map((x) => <option key={x.label} value={x.label} />)}</datalist>
        </form>
        {data.synonyms.length ? (
          <div className="name-merges">
            <span className="opt-label">{t("Sinonim aktif ({n})", { n: formatInt(data.synonyms.length) })}</span>
            {data.synonyms.map((m) => (
              <div key={m.fromKey} className="name-merge">
                <span>{t("{from} dihitung sebagai {to}", { from: m.fromLabel, to: m.toLabel })}</span>
                <button type="button" className="btn" disabled={busy} onClick={() => void remove(m.fromKey)}>{t("Hapus")}</button>
              </div>
            ))}
          </div>
        ) : null}
      </section>
    </div>
  );
}
