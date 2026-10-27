import { useMemo } from "react";
import { useWidth } from "../../hooks/useWidth";
import { useZoomPan } from "../../hooks/useZoomPan";
import { forceLayout } from "../../lib/forceLayout";
import { formatInt } from "../../lib/format";
import { placeLabels } from "../../lib/labelPlace";
import { clusterColor, type Bridge, type NetEdge, type NetNode } from "../../lib/structureTypes";

const PAD = 28;
/** Labels shown at 1x; more appear as the view is zoomed in. */
const LABELS = 14;
const MAX_LABELS = 60;

interface Props {
  nodes: NetNode[];
  edges: NetEdge[];
  bridge: Bridge | null;
  focus: number | null;
  onFocus: (cluster: number | null) => void;
}

export function NetworkGraph({ nodes, edges: rawEdges, bridge, focus, onFocus }: Props) {
  const { ref: box, width } = useWidth<HTMLDivElement>(800);
  // Defensive: drop links that point outside the node list instead of crashing the screen.
  const edges = useMemo(
    () => rawEdges.filter((e) => e.from >= 0 && e.to >= 0 && e.from < nodes.length && e.to < nodes.length),
    [rawEdges, nodes.length],
  );
  const height = Math.round(Math.min(600, Math.max(380, width * 0.62)));
  const zoom = useZoomPan(width, height);
  const { k, tx, ty } = zoom.view;
  const layout = useMemo(() => forceLayout(nodes.length, edges), [nodes, edges]);
  const x = (i: number) => tx + k * (PAD + layout[i].x * (width - PAD * 2));
  const y = (i: number) => ty + k * (PAD + layout[i].y * (height - PAD * 2));
  const maxCount = Math.max(...nodes.map((n) => n.count), 1);
  const radius = (n: NetNode) => (4 + 11 * Math.sqrt(n.count / maxCount)) * k ** 0.3;
  const dim = (cluster: number) => focus !== null && focus !== cluster;
  const labels = placeLabels(
    nodes.flatMap((n, i) => (dim(n.cluster) ? [] : [{ index: i, x: x(i), y: y(i), r: radius(n), text: n.label, weight: n.count + (focus === n.cluster ? maxCount : 0) }])),
    width, height, Math.min(MAX_LABELS, Math.round(LABELS * k)),
  );

  const centroid = (cluster: number) => {
    const ids = nodes.map((n, i) => (n.cluster === cluster ? i : -1)).filter((i) => i >= 0);
    return ids.length ? { cx: ids.reduce((a, i) => a + x(i), 0) / ids.length, cy: ids.reduce((a, i) => a + y(i), 0) / ids.length } : null;
  };
  const ends = bridge ? [centroid(bridge.a), centroid(bridge.b)] : [];

  return (
    <div ref={box} className="netgraph">
      <div className="net-zoom" role="group" aria-label="Perbesar jaringan">
        <span className="muted">Roda mouse untuk zoom, seret untuk menggeser</span>
        <button type="button" className="btn" aria-label="Perbesar" onClick={zoom.zoomIn}>+</button>
        <button type="button" className="btn" aria-label="Perkecil" disabled={k <= 1} onClick={zoom.zoomOut}>-</button>
        <button type="button" className="btn" disabled={k === 1} onClick={zoom.reset}>Atur ulang</button>
      </div>
      <svg ref={zoom.ref} width={width} height={height} role="img" aria-label="Jaringan, warna menunjukkan cluster; roda mouse untuk zoom, seret untuk menggeser"
        data-zoomed={k > 1} {...zoom.handlers} onClick={() => { if (!zoom.wasDrag()) onFocus(null); }}>
        {edges.map((e, i) => {
          const same = nodes[e.from].cluster === nodes[e.to].cluster;
          const faded = dim(nodes[e.from].cluster) && dim(nodes[e.to].cluster);
          return (
            <line key={i} x1={x(e.from)} y1={y(e.from)} x2={x(e.to)} y2={y(e.to)}
              stroke={same ? clusterColor(nodes[e.from].cluster) : "#8A929C"}
              strokeOpacity={faded ? 0.04 : 0.12 + 0.45 * e.weight} strokeWidth={0.6 + 2 * e.weight} />
          );
        })}
        {ends[0] && ends[1] ? (
          <line x1={ends[0].cx} y1={ends[0].cy} x2={ends[1].cx} y2={ends[1].cy} className="net-bridge" />
        ) : null}
        {nodes.map((n, i) => (
          <circle key={i} cx={x(i)} cy={y(i)} r={radius(n)} fill={clusterColor(n.cluster)}
            fillOpacity={dim(n.cluster) ? 0.2 : 0.9} style={{ stroke: "var(--panel)" }} strokeWidth={1.5} className="net-node"
            onClick={(ev) => { ev.stopPropagation(); if (!zoom.wasDrag()) onFocus(n.cluster); }}>
            <title>{`${n.label}: ${formatInt(n.count)}`}</title>
          </circle>
        ))}
        {labels.map((l) => (
          <text key={l.index} x={l.x} y={l.y} dy="0.35em" textAnchor={l.anchor} className="net-label">{nodes[l.index].label}</text>
        ))}
      </svg>
    </div>
  );
}
