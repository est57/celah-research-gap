import type { Evolution, ThemeLink } from "./evolutionTypes";

export interface AlluvialNode {
  period: number;
  id: number;
  x: number;
  y: number;
  h: number;
}

export interface AlluvialRibbon {
  link: ThemeLink;
  d: string;
}

export const ALLUVIAL_NODE_W = 12;
const GAP = 10;
const MIN_H = 3;

const f = (v: number) => v.toFixed(1);

/** Band from (x0, y0, h0) to (x1, y1, h1); the two ends may differ in height. */
function band(x0: number, y0: number, h0: number, x1: number, y1: number, h1: number): string {
  const xm = (x0 + x1) / 2;
  return `M${f(x0)} ${f(y0)} C${f(xm)} ${f(y0)} ${f(xm)} ${f(y1)} ${f(x1)} ${f(y1)} L${f(x1)} ${f(y1 + h1)} C${f(xm)} ${f(y1 + h1)} ${f(xm)} ${f(y0 + h0)} ${f(x0)} ${f(y0 + h0)} Z`;
}

/** One column per period, node height by theme frequency on a shared scale. */
export function layoutAlluvial(e: Evolution, links: ThemeLink[], width: number, height: number, labelW: number) {
  const cols = e.periods.length;
  const usable = width - labelW - ALLUVIAL_NODE_W;
  const xOf = (p: number) => (cols === 1 ? 0 : (usable * p) / (cols - 1));
  const scale = Math.min(...e.periods.map((p) => {
    const total = p.themes.reduce((s, t) => s + t.frequency, 0);
    return total > 0 ? (height - GAP * (p.themes.length - 1)) / total : Infinity;
  }));
  if (!Number.isFinite(scale) || scale <= 0) return { nodes: [], ribbons: [] };

  const nodes: AlluvialNode[] = [];
  const at = new Map<string, AlluvialNode>();
  e.periods.forEach((p, pi) => {
    let y = 0;
    for (const t of p.themes) {
      const node = { period: pi, id: t.id, x: xOf(pi), y, h: Math.max(MIN_H, t.frequency * scale) };
      nodes.push(node);
      at.set(`${pi}:${t.id}`, node);
      y += node.h + GAP;
    }
  });

  // Stack ribbons inside each node in the order of the node they connect to, so bands do not cross needlessly.
  const outY = new Map(nodes.map((n) => [`${n.period}:${n.id}`, n.y]));
  const inY = new Map(outY);
  const ribbons: AlluvialRibbon[] = [];
  const sorted = [...links].sort((a, b) => a.period - b.period || a.from - b.from || a.to - b.to);
  for (const l of sorted) {
    const a = at.get(`${l.period}:${l.from}`);
    const b = at.get(`${l.period + 1}:${l.to}`);
    if (!a || !b) continue;
    const h0 = Math.min(a.h, l.outValue * scale);
    const h1 = Math.min(b.h, l.inValue * scale);
    const y0 = outY.get(`${l.period}:${l.from}`) ?? a.y;
    const y1 = inY.get(`${l.period + 1}:${l.to}`) ?? b.y;
    ribbons.push({ link: l, d: band(a.x + ALLUVIAL_NODE_W, y0, h0, b.x, y1, h1) });
    outY.set(`${l.period}:${l.from}`, y0 + h0);
    inY.set(`${l.period + 1}:${l.to}`, y1 + h1);
  }
  return { nodes, ribbons };
}
