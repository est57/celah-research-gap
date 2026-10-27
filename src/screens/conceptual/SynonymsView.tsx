import { useState, type FormEvent } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useSynonyms } from "../../hooks/useSynonyms";
import { formatInt } from "../../lib/format";
import { SynonymCard } from "./SynonymCard";

const SHOWN = 9;

export function SynonymsView({ version, onDataChanged }: { version: number; onDataChanged: () => void }) {
  const { data, sample, error, loading, busy, actionError, merge, remove } = useSynonyms(version, onDataChanged);
  const [all, setAll] = useState(false);
  const [from, setFrom] = useState("");
  const [to, setTo] = useState("");
  if (error) return <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  if (!data) return <ScreenMessage>{loading ? "Mencari kata kunci yang bermakna sama..." : ""}</ScreenMessage>;
  if (data.terms.length === 0) return <EmptyCollection />;
  const manual = (e: FormEvent) => {
    e.preventDefault();
    if (from.trim() && to.trim()) void merge([from.trim(), to.trim()], to.trim()).then(() => { setFrom(""); setTo(""); });
  };
  const shown = all ? data.suggestions : data.suggestions.slice(0, SHOWN);

  return (
    <div className="year-page">
      {sample ? <div className="notice">Pratinjau browser: kata kunci di halaman ini adalah data contoh.</div> : null}
      <section className="pnl">
        <div className="ph">Sinonim kata kunci<span className="ph-aside">{`${formatInt(data.suggestions.length)} saran`}</span></div>
        <p className="muted ent-note">
          Kata kunci yang disatukan dihitung sebagai satu istilah di peta tematik, jaringan, evolusi, analisis faktorial,
          plot tiga bidang, dan pencarian celah, sehingga celah palsu antara dua sinonim hilang. Teks kata kunci asli
          tidak diubah.
        </p>
        {actionError ? <p className="delete-error ent-note" role="alert">{actionError}</p> : null}
        <ul className="name-list">
          {shown.map((s) => <SynonymCard key={s.terms.map((t) => t.label).join("|")} s={s} busy={busy} onMerge={(l, t) => void merge(l, t)} />)}
        </ul>
        {data.suggestions.length > SHOWN ? (
          <button type="button" className="btn name-more" onClick={() => setAll(!all)}>
            {all ? "Tampilkan lebih sedikit" : `Tampilkan semua (${formatInt(data.suggestions.length)})`}
          </button>
        ) : null}
        <form className="syn-manual" onSubmit={manual}>
          <span className="opt-label">Satukan manual</span>
          <label htmlFor="syn-from" className="sr-only">Kata kunci</label>
          <input id="syn-from" className="input" list="syn-terms" maxLength={120} placeholder="Kata kunci, misalnya TB" value={from} onChange={(e) => setFrom(e.target.value)} />
          <span className="muted">dihitung sebagai</span>
          <label htmlFor="syn-to" className="sr-only">Kata kunci tujuan</label>
          <input id="syn-to" className="input" list="syn-terms" maxLength={120} placeholder="misalnya tuberculosis" value={to} onChange={(e) => setTo(e.target.value)} />
          <button type="submit" className="btn" disabled={busy || !from.trim() || !to.trim()}>Satukan</button>
          <datalist id="syn-terms">{data.terms.map((t) => <option key={t.label} value={t.label} />)}</datalist>
        </form>
        {data.synonyms.length ? (
          <div className="name-merges">
            <span className="opt-label">{`Sinonim aktif (${formatInt(data.synonyms.length)})`}</span>
            {data.synonyms.map((m) => (
              <div key={m.fromKey} className="name-merge">
                <span>{`${m.fromLabel} dihitung sebagai ${m.toLabel}`}</span>
                <button type="button" className="btn" disabled={busy} onClick={() => void remove(m.fromKey)}>Hapus</button>
              </div>
            ))}
          </div>
        ) : null}
      </section>
    </div>
  );
}
