import type { FieldLink, FieldNode, ThreeField } from "./overviewTypes";

export interface PlacedNode {
  label: string;
  value: number;
  column: 0 | 1 | 2;
  x: number;
  y: number;
  h: number;
}

export interface Ribbon {
  d: string;
  /** 0 = left-to-middle, 1 = middle-to-right. */
  stage: 0 | 1;
  /** Index of the source node within its column, for colouring. */
  source: number;
  value: number;
}

export interface SankeyLayout {
  nodes: PlacedNode[];
  ribbons: Ribbon[];
}

const NODE_W = 10;
const GAP = 8;

function sums(links: FieldLink[], key: "from" | "to", n: number): number[] {
  const out = new Array<number>(n).fill(0);
  for (const l of links) out[l[key]] += l.value;
  return out;
}

function ribbonPath(x0: number, y0: number, x1: number, y1: number, h: number): string {
  const xm = (x0 + x1) / 2;
  const f = (v: number) => v.toFixed(1);
  return `M${f(x0)} ${f(y0)} C${f(xm)} ${f(y0)} ${f(xm)} ${f(y1)} ${f(x1)} ${f(y1)} L${f(x1)} ${f(y1 + h)} C${f(xm)} ${f(y1 + h)} ${f(xm)} ${f(y0 + h)} ${f(x0)} ${f(y0 + h)} Z`;
}

/** Lays out a three-column Sankey diagram; `labelW` is reserved on each side for labels. */
export function layoutSankey(data: ThreeField, width: number, height: number, labelW: number): SankeyLayout {
  const cols: FieldNode[][] = [data.left, data.middle, data.right];
  const values = [
    sums(data.leftLinks, "from", data.left.length),
    data.middle.map((_, i) =>
      Math.max(sums(data.leftLinks, "to", data.middle.length)[i], sums(data.rightLinks, "from", data.middle.length)[i]),
    ),
    sums(data.rightLinks, "to", data.right.length),
  ];
  const xs = [labelW, (labelW + width - labelW - NODE_W) / 2, width - labelW - NODE_W];

  // One scale for all columns so equal values have equal heights.
  const scale = Math.min(
    ...values.map((v) => {
      const shown = v.filter((x) => x > 0);
      const total = shown.reduce((a, b) => a + b, 0);
      return total > 0 ? (height - GAP * (shown.length - 1)) / total : Infinity;
    }),
  );
  if (!Number.isFinite(scale) || scale <= 0) return { nodes: [], ribbons: [] };

  const tops: number[][] = values.map((v) => {
    let y = 0;
    return v.map((val) => {
      const top = y;
      if (val > 0) y += val * scale + GAP;
      return top;
    });
  });
  const nodes: PlacedNode[] = [];
  cols.forEach((col, c) =>
    col.forEach((n, i) => {
      if (values[c][i] > 0) {
        nodes.push({ label: n.label, value: values[c][i], column: c as 0 | 1 | 2, x: xs[c], y: tops[c][i], h: values[c][i] * scale });
      }
    }),
  );

  const ribbons: Ribbon[] = [];
  const stages: [FieldLink[], number][] = [[data.leftLinks, 0], [data.rightLinks, 1]];
  for (const [links, s] of stages) {
    const out = [...tops[s]];
    const inn = [...tops[s + 1]];
    for (const l of links) {
      const h = l.value * scale;
      ribbons.push({ d: ribbonPath(xs[s] + NODE_W, out[l.from], xs[s + 1], inn[l.to], h), stage: s as 0 | 1, source: l.from, value: l.value });
      out[l.from] += h;
      inn[l.to] += h;
    }
  }
  return { nodes, ribbons };
}

export const SANKEY_NODE_W = NODE_W;
