import { formatDecimal } from "../../lib/format";
import { KIND_INFO, STATUS_LABEL, type Gap, type GapKind } from "../../lib/gapTypes";

export type KindFilter = GapKind | "all";
export type SortBy = "score" | "unreviewed";

interface Props {
  gaps: Gap[];
  all: Gap[];
  selected: string | null;
  kind: KindFilter;
  sort: SortBy;
  onKind: (k: KindFilter) => void;
  onSort: (s: SortBy) => void;
  onSelect: (key: string) => void;
}

export function GapListPane({ gaps, all, selected, kind, sort, onKind, onSort, onSelect }: Props) {
  const count = (k: GapKind) => all.filter((g) => g.kind === k).length;
  const accepted = all.filter((g) => g.status === "accepted").length;
  const rejected = all.filter((g) => g.status === "rejected").length;

  return (
    <section className="gap-list" aria-label="Daftar kandidat celah">
      <div className="gap-filters">
        <label className="sr-only" htmlFor="gap-kind">Jenis celah</label>
        <select id="gap-kind" className="input" value={kind} onChange={(e) => onKind(e.target.value as KindFilter)}>
          <option value="all">Semua jenis ({all.length})</option>
          {(Object.keys(KIND_INFO) as GapKind[]).map((k) => (
            <option key={k} value={k}>{`${KIND_INFO[k].label} (${count(k)})`}</option>
          ))}
        </select>
        <label className="sr-only" htmlFor="gap-sort">Urutan</label>
        <select id="gap-sort" className="input" value={sort} onChange={(e) => onSort(e.target.value as SortBy)}>
          <option value="score">Skor tertinggi</option>
          <option value="unreviewed">Belum direview dulu</option>
        </select>
      </div>
      <div className="gap-rows">
        {gaps.map((g) => (
          <button key={g.key} type="button" className="gap-row" aria-pressed={g.key === selected} onClick={() => onSelect(g.key)}>
            <span className="kind-badge" style={{ background: KIND_INFO[g.kind].color }} title={KIND_INFO[g.kind].label}>
              {KIND_INFO[g.kind].letter}
            </span>
            <span className="gap-row-body">
              <span className="gap-row-title">{g.title}</span>
              <span className="gap-row-meta muted">
                <span className="num">{formatDecimal(g.score, 2)}</span>
                <span className="mini-bar"><span style={{ width: `${g.score * 100}%` }} /></span>
                <span>{g.confidence}</span>
                {g.status ? <span className="status-pill" data-status={g.status}>{STATUS_LABEL[g.status]}</span> : null}
              </span>
            </span>
          </button>
        ))}
        {gaps.length === 0 ? <p className="muted gap-none">Tidak ada kandidat untuk filter ini.</p> : null}
      </div>
      <div className="gap-list-foot muted">
        {gaps.length} dari {all.length} ditampilkan · {accepted} diterima · {rejected} ditolak
      </div>
    </section>
  );
}
