import { useMemo, useState } from "react";
import { formatInt } from "../../lib/format";
import type { TopRef } from "../../lib/topRefTypes";

const PAGE = 50;

interface Props {
  refs: TopRef[];
  /** Adds works by DOI. */
  onAdd: (dois: string[]) => void;
  busy: boolean;
}

/** Searchable, paged list of cited works with their collection status, copy and add actions. */
export function TopRefList({ refs, onAdd, busy }: Props) {
  const [query, setQuery] = useState("");
  const [onlyMissing, setOnlyMissing] = useState(false);
  const [page, setPage] = useState(0);
  const [copied, setCopied] = useState<string | null>(null);

  const shown = useMemo(() => {
    const q = query.trim().toLowerCase();
    return refs.filter((r) => (!onlyMissing || r.inCollection === null) && (!q || `${r.label} ${r.example}`.toLowerCase().includes(q)));
  }, [refs, query, onlyMissing]);
  const pages = Math.max(1, Math.ceil(shown.length / PAGE));
  const current = Math.min(page, pages - 1);

  const copy = async (key: string, text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopied(key);
    } catch {
      setCopied(null);
    }
  };

  return (
    <div className="tr">
      <div className="ent-bar">
        <label htmlFor="tr-q" className="sr-only">Cari rujukan</label>
        <input id="tr-q" className="input" type="search" maxLength={100} placeholder="Cari penulis, tahun, atau judul" value={query}
          onChange={(e) => { setQuery(e.target.value); setPage(0); }} />
        <label className="tr-toggle">
          <input type="checkbox" checked={onlyMissing} onChange={(e) => { setOnlyMissing(e.target.checked); setPage(0); }} />
          Hanya yang belum ada di koleksi
        </label>
        <span className="muted">{`${formatInt(shown.length)} rujukan`}</span>
      </div>
      <ol className="tr-list" start={current * PAGE + 1}>
        {shown.slice(current * PAGE, (current + 1) * PAGE).map((r, i) => {
          const key = `${r.label}-${current * PAGE + i}`;
          return (
            <li key={key}>
              <div className="tr-head">
                <span className="tr-label">{r.label}</span>
                <span className="tr-chip" data-in={r.inCollection !== null}>{r.inCollection !== null ? "Di koleksi" : "Belum ada"}</span>
                <span className="num tr-count">{`${formatInt(r.documents)} dok.`}</span>
              </div>
              {r.title ? <div className="tr-title">{r.title}</div> : null}
              <div className="tr-example select">{r.example}</div>
              <div className="tr-actions">
                {r.inCollection === null && r.doi ? (
                  <button type="button" className="btn btn-primary tr-btn" disabled={busy} onClick={() => onAdd([r.doi ?? ""])}>
                    Tambahkan ke koleksi
                  </button>
                ) : null}
                <button type="button" className="btn tr-btn" onClick={() => void copy(`${key}-r`, r.example)}>
                  {copied === `${key}-r` ? "Tersalin" : "Salin rujukan"}
                </button>
                {r.doi ? (
                  <button type="button" className="btn tr-btn" onClick={() => void copy(`${key}-d`, r.doi ?? "")}>
                    {copied === `${key}-d` ? "Tersalin" : "Salin DOI"}
                  </button>
                ) : null}
              </div>
            </li>
          );
        })}
      </ol>
      {shown.length === 0 ? <p className="muted ent-empty">Tidak ada yang cocok.</p> : null}
      {pages > 1 ? (
        <div className="pager">
          <button type="button" className="btn" disabled={current === 0} onClick={() => setPage(current - 1)}>Sebelumnya</button>
          <span className="num muted">{`${current + 1} / ${pages}`}</span>
          <button type="button" className="btn" disabled={current >= pages - 1} onClick={() => setPage(current + 1)}>Berikutnya</button>
        </div>
      ) : null}
    </div>
  );
}
