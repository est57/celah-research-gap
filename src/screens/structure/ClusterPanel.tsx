import { formatDecimal, formatInt } from "../../lib/format";
import { clusterColor, type Bridge, type NetCluster } from "../../lib/structureTypes";
import { t } from "../../i18n";

interface Props {
  clusters: NetCluster[];
  bridges: Bridge[];
  focus: number | null;
  unitLabel: string;
  onFocus: (cluster: number | null) => void;
}

/** A cluster pair linked at under half the expected strength; the graph draws it as a dashed line. */
export function isWeakBridge(b: Bridge): boolean {
  return b.ratio < 0.5;
}

export function ClusterPanel({ clusters, bridges, focus, unitLabel, onFocus }: Props) {
  const name = (id: number) => clusters.find((c) => c.id === id)?.top[0] ?? t("Cluster {n}", { n: id + 1 });
  const weak = bridges.filter(isWeakBridge);

  return (
    <>
      <div className="ph">{t("Cluster")}</div>
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
      <div className="ph">{t("Sinyal celah struktural")}</div>
      <div className="bridge-box">
        {weak.length === 0 ? (
          <p className="muted">{t("Tidak ada pasangan cluster yang hubungannya jauh di bawah perkiraan.")}</p>
        ) : (
          weak.map((b) => (
            <div key={`${b.a}-${b.b}`} className="notice bridge">
              <strong>{t("{a} dan {b}", { a: name(b.a), b: name(b.b) })}</strong>
              <span>
                {t("{links} tautan, hanya {pct}% dari yang diharapkan menurut ukuran kedua cluster.", {
                  links: formatInt(b.links), pct: formatDecimal(b.ratio * 100, 0) })}
              </span>
            </div>
          ))
        )}
        <p className="muted bridge-note">
          {t("Cluster {unit} yang jarang terhubung bisa menandai dua bidang yang belum saling bertemu. Garis oranye putus-putus di grafik menandai pasangan terlemah.", { unit: unitLabel.toLowerCase() })}
        </p>
      </div>
    </>
  );
}
