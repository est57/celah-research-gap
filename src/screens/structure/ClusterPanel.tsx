import { formatDecimal, formatInt } from "../../lib/format";
import { clusterColor, type Bridge, type NetCluster } from "../../lib/structureTypes";

interface Props {
  clusters: NetCluster[];
  bridges: Bridge[];
  focus: number | null;
  unitLabel: string;
  onFocus: (cluster: number | null) => void;
}

export function ClusterPanel({ clusters, bridges, focus, unitLabel, onFocus }: Props) {
  const name = (id: number) => clusters.find((c) => c.id === id)?.top[0] ?? `Cluster ${id + 1}`;
  const weak = bridges.filter((b) => b.ratio < 0.5);

  return (
    <>
      <div className="ph">Cluster</div>
      <ul className="cluster-list">
        {clusters.map((c) => (
          <li key={c.id}>
            <button type="button" aria-pressed={focus === c.id} onClick={() => onFocus(focus === c.id ? null : c.id)}>
              <span className="swatch-dot" style={{ background: clusterColor(c.id) }} />
              <span className="cluster-name">{c.top.join(", ")}</span>
              <span className="num muted">{formatInt(c.size)}</span>
            </button>
          </li>
        ))}
      </ul>
      <div className="ph">Sinyal celah struktural</div>
      <div className="bridge-box">
        {weak.length === 0 ? (
          <p className="muted">Tidak ada pasangan cluster yang hubungannya jauh di bawah perkiraan.</p>
        ) : (
          weak.map((b) => (
            <div key={`${b.a}-${b.b}`} className="notice bridge">
              <strong>{name(b.a)} dan {name(b.b)}</strong>
              <span>
                {formatInt(b.links)} tautan, hanya {formatDecimal(b.ratio * 100, 0)}% dari yang diharapkan menurut ukuran
                kedua cluster.
              </span>
            </div>
          ))
        )}
        <p className="muted bridge-note">
          Cluster {unitLabel.toLowerCase()} yang jarang terhubung bisa menandai dua bidang yang belum saling bertemu.
          Garis oranye putus-putus di grafik menandai pasangan terlemah.
        </p>
      </div>
    </>
  );
}
