import { formatInt } from "../../lib/format";
import { clusterColor } from "../../lib/structureTypes";
import type { Historiograph } from "../../lib/histTypes";

export function HistDetail({ data, index, onSelect }: { data: Historiograph; index: number; onSelect: (i: number) => void }) {
  const n = data.nodes[index];
  if (!n) return null;
  const cites = data.edges.filter((e) => e[0] === index).map((e) => e[1]);
  const citedBy = data.edges.filter((e) => e[1] === index).map((e) => e[0]);
  const list = (title: string, ids: number[]) => (
    <div className="evo-flow">
      <span className="opt-label">{`${title} (${ids.length})`}</span>
      {ids.length === 0 ? <span className="muted">Tidak ada di antara dokumen yang ditampilkan.</span> : null}
      {ids.map((i) => (
        <button key={i} type="button" className="evo-chip" onClick={() => onSelect(i)}>
          <span className="theme-swatch" style={{ background: clusterColor(data.nodes[i].cluster) }} />
          {data.nodes[i].label}
        </button>
      ))}
    </div>
  );
  return (
    <div className="theme-detail">
      <div className="theme-head">
        <span className="theme-swatch" style={{ background: clusterColor(n.cluster) }} />
        <strong>{n.label}</strong>
      </div>
      <p className="hist-detail-title">{n.title}</p>
      <div className="muted theme-quadrant">{[n.source, n.doi].filter(Boolean).join(" · ")}</div>
      <div className="theme-stats">
        <div><div className="num">{formatInt(n.lcs)}</div><div className="muted">sitasi lokal</div></div>
        <div><div className="num">{formatInt(n.gcs)}</div><div className="muted">sitasi global</div></div>
        <div><div className="num">{n.gcs > 0 ? `${Math.round((n.lcs / n.gcs) * 100)}%` : "-"}</div><div className="muted">lokal/global</div></div>
      </div>
      {list("Menyitasi", cites)}
      {list("Disitasi oleh", citedBy)}
    </div>
  );
}
