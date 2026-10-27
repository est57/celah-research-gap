// Greedy label placement: the most important labels first, each kept only when its
// box does not overlap a label already placed or leave the plot.

export interface LabelCandidate {
  index: number;
  /** Anchor point and node radius in screen pixels. */
  x: number;
  y: number;
  r: number;
  text: string;
  /** Higher goes first. */
  weight: number;
}

export interface PlacedLabel {
  index: number;
  x: number;
  y: number;
  anchor: "start" | "end";
}

const CHAR_W = 6.4;
const LINE_H = 14;
const GAP = 3;

interface Box { x0: number; x1: number; y0: number; y1: number }

const overlaps = (a: Box, b: Box) => a.x0 < b.x1 && b.x0 < a.x1 && a.y0 < b.y1 && b.y0 < a.y1;

/** Tries the right side, then the left; returns at most `limit` labels inside `width` x `height`. */
export function placeLabels(items: LabelCandidate[], width: number, height: number, limit: number): PlacedLabel[] {
  const placed: PlacedLabel[] = [];
  const boxes: Box[] = [];
  for (const c of [...items].sort((a, b) => b.weight - a.weight)) {
    if (placed.length >= limit) break;
    if (c.x < 0 || c.x > width || c.y < 0 || c.y > height) continue;
    const w = c.text.length * CHAR_W;
    const sides: ["start" | "end", number][] = [["start", c.x + c.r + GAP], ["end", c.x - c.r - GAP]];
    for (const [anchor, x] of sides) {
      const box = anchor === "start" ? { x0: x, x1: x + w } : { x0: x - w, x1: x };
      const full = { ...box, y0: c.y - LINE_H / 2, y1: c.y + LINE_H / 2 };
      if (full.x0 < 0 || full.x1 > width || full.y0 < 0 || full.y1 > height) continue;
      if (boxes.some((b) => overlaps(b, full))) continue;
      boxes.push(full);
      placed.push({ index: c.index, x, y: c.y, anchor });
      break;
    }
  }
  return placed;
}

/** Labels centered above their nodes, kept only when they do not overlap one already shown. */
export function placeAbove(items: LabelCandidate[], width: number, height: number): Set<number> {
  const shown = new Set<number>();
  const boxes: Box[] = [];
  for (const c of [...items].sort((a, b) => b.weight - a.weight)) {
    const w = c.text.length * CHAR_W;
    const y = c.y - c.r - 4;
    const box = { x0: c.x - w / 2, x1: c.x + w / 2, y0: y - LINE_H + 3, y1: y + 3 };
    if (box.x1 < 0 || box.x0 > width || box.y1 < 0 || box.y0 > height) continue;
    if (boxes.some((b) => overlaps(b, box))) continue;
    boxes.push(box);
    shown.add(c.index);
  }
  return shown;
}
