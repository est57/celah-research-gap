import { useMemo } from "react";
import { useWidth } from "../../hooks/useWidth";
import { useZoomPan } from "../../hooks/useZoomPan";
import { placeAbove } from "../../lib/labelPlace";
import { clusterColor } from "../../lib/structureTypes";
import type { Historiograph } from "../../lib/histTypes";

const PAD_X = 46;
const PAD_TOP = 30;
const AXIS = 28;
const ROW = 38;
const MIN_HEIGHT = 320;

interface Props {
  data: Historiograph;
  selected: number | null;
  onSelect: (index: number | null) => void;
}

/** One column per publication year; citations run from the citing (later) document to the cited one. */
export function HistChart({ data, selected, onSelect }: Props) {
  const { ref, width } = useWidth<HTMLDivElement>(900);
  const layout = useMemo(() => {
    const years = [...new Set(data.nodes.map((n) => n.year))].sort((a, b) => a - b);
    const perYear = years.map((y) => data.nodes.map((n, i) => ({ n, i })).filter((e) => e.n.year === y)
      .sort((a, b) => a.n.cluster - b.n.cluster || b.n.lcs - a.n.lcs));
    const rows = Math.max(1, ...perYear.map((c) => c.length));
    const height = Math.max(MIN_HEIGHT, PAD_TOP + rows * ROW + AXIS);
    const row = (height - PAD_TOP - AXIS) / rows;
    const colX = (c: number) => (years.length === 1 ? width / 2 : PAD_X + (c * (width - PAD_X * 2)) / (years.length - 1));
    const pos = new Array<{ x: number; y: number }>(data.nodes.length);
    perYear.forEach((col, c) => {
      const offset = ((rows - col.length) * row) / 2;
      col.forEach((e, r) => (pos[e.i] = { x: colX(c), y: PAD_TOP + offset + r * row + row / 2 }));
    });
    const gap = years.length > 1 ? (width - PAD_X * 2) / (years.length - 1) : width;
    return { years, colX, pos, height, gap };
  }, [data.nodes, width]);
  const zoom = useZoomPan(width, layout.height);
  const { k, tx, ty } = zoom.view;
  // Screen position of a layout point under the current zoom.
  const at = (p: { x: number; y: number }) => ({ x: tx + k * p.x, y: ty + k * p.y });
  const colX = (c: number) => tx + k * layout.colX(c);
  const maxLcs = Math.max(...data.nodes.map((n) => n.lcs), 1);
  const radius = (lcs: number) => (4 + 8 * Math.sqrt(lcs / maxLcs)) * k ** 0.3;
  // The year is on the axis, so narrow columns show the surname only.
  const short = layout.gap * k < 110;
  const text = (label: string) => (short ? label.replace(/ \d{4}$/, "") : label);
  const labelled = placeAbove(
    data.nodes.map((n, i) => ({ index: i, ...at(layout.pos[i]), r: radius(n.lcs), text: text(n.label), weight: n.lcs + (i === selected ? maxLcs * 2 : 0) })),
    width, layout.height - AXIS,
  );
  const touches = (e: [number, number]) => selected !== null && (e[0] === selected || e[1] === selected);
  const near = new Set(data.edges.filter(touches).flat());

  return (
    <div ref={ref} className="hist-chart netgraph">
      <div className="net-zoom" role="group" aria-label="Perbesar historiograf">
        <span className="muted">Roda mouse untuk zoom, seret untuk menggeser</span>
        <button type="button" className="btn" aria-label="Perbesar" onClick={zoom.zoomIn}>+</button>
        <button type="button" className="btn" aria-label="Perkecil" disabled={k <= 1} onClick={zoom.zoomOut}>-</button>
        <button type="button" className="btn" disabled={k === 1} onClick={zoom.reset}>Atur ulang</button>
      </div>
      <svg ref={zoom.ref} width={width} height={layout.height} role="img" aria-label="Historiograf sitasi langsung menurut tahun"
        {...zoom.handlers} onClick={() => { if (!zoom.wasDrag()) onSelect(null); }}>
        <defs>
          <marker id="hist-arrow" viewBox="0 0 8 8" refX="7" refY="4" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
            <path d="M0 0 L8 4 L0 8 Z" style={{ fill: "var(--axis)" }} />
          </marker>
        </defs>
        {layout.years.map((y, c) => (
          <g key={y}>
            <line x1={colX(c)} x2={colX(c)} y1={PAD_TOP} y2={layout.height - AXIS} className="hist-grid" />
            <text x={colX(c)} y={layout.height - 8} textAnchor="middle" className="hist-year">{y}</text>
          </g>
        ))}
        {data.edges.map((e, k) => {
          if (!layout.pos[e[0]] || !layout.pos[e[1]]) return null;
          const a = at(layout.pos[e[0]]);
          const b = at(layout.pos[e[1]]);
          const r = radius(data.nodes[e[1]].lcs) + 2;
          const bx = b.x + (a.x >= b.x ? r : -r);
          const dx = Math.abs(a.x - bx);
          // Arcs rise with the distance they span, so long citations pass over the nodes in between.
          const lift = Math.max(0, Math.min(Math.min(a.y, b.y) - 4, 14 + dx * 0.12));
          const c1 = `${a.x - (a.x - bx) * 0.3} ${a.y - lift}`;
          const c2 = `${bx + (a.x - bx) * 0.3} ${b.y - lift}`;
          return (
            <path key={k} d={`M${a.x} ${a.y} C${c1} ${c2} ${bx} ${b.y}`} fill="none" markerEnd="url(#hist-arrow)"
              className="hist-edge" data-state={selected === null ? "" : touches(e) ? "on" : "off"} />
          );
        })}
        {data.nodes.map((n, i) => {
          const p = at(layout.pos[i]);
          const dim = selected !== null && i !== selected && !near.has(i);
          return (
            <g key={n.id} className="hist-node" opacity={dim ? 0.3 : 1} tabIndex={0} role="button" aria-pressed={i === selected}
              aria-label={`${n.label}, ${n.lcs} sitasi lokal`}
              onClick={(ev) => { ev.stopPropagation(); if (!zoom.wasDrag()) onSelect(i); }}
              onKeyDown={(ev) => { if (ev.key === "Enter" || ev.key === " ") { ev.preventDefault(); onSelect(i); } }}>
              <circle cx={p.x} cy={p.y} r={radius(n.lcs)} fill={clusterColor(n.cluster)} style={{ stroke: i === selected ? "var(--ink)" : "var(--panel)" }} strokeWidth={1.5} />
              {labelled.has(i) ? (
                <text x={p.x} y={p.y - radius(n.lcs) - 4} textAnchor="middle" className="hist-label">{text(n.label)}</text>
              ) : null}
              <title>{`${n.title} (${n.lcs} sitasi lokal)`}</title>
            </g>
          );
        })}
      </svg>
    </div>
  );
}
