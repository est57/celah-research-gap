// Fruchterman-Reingold force-directed layout in a unit box (0..1).
// Seeded, so the same network always gets the same picture.

export interface Point {
  x: number;
  y: number;
}

interface LinkLike {
  from: number;
  to: number;
  weight: number;
}

function mulberry32(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

export function forceLayout(n: number, links: LinkLike[], iterations = 300, seed = 7): Point[] {
  const rand = mulberry32(seed);
  const pos: Point[] = Array.from({ length: n }, () => ({ x: rand(), y: rand() }));
  if (n <= 1) return pos.map(() => ({ x: 0.5, y: 0.5 }));
  const k = Math.sqrt(1 / n);
  let temp = 0.1;
  const cool = temp / iterations;

  for (let it = 0; it < iterations; it++) {
    const disp: Point[] = pos.map(() => ({ x: 0, y: 0 }));
    for (let i = 0; i < n; i++) {
      for (let j = i + 1; j < n; j++) {
        const dx = pos[i].x - pos[j].x;
        const dy = pos[i].y - pos[j].y;
        const d = Math.max(Math.hypot(dx, dy), 1e-4);
        const f = (k * k) / d;
        disp[i].x += (dx / d) * f;
        disp[i].y += (dy / d) * f;
        disp[j].x -= (dx / d) * f;
        disp[j].y -= (dy / d) * f;
      }
    }
    for (const l of links) {
      const a = pos[l.from];
      const b = pos[l.to];
      const dx = a.x - b.x;
      const dy = a.y - b.y;
      const d = Math.max(Math.hypot(dx, dy), 1e-4);
      const f = ((d * d) / k) * (0.2 + l.weight);
      disp[l.from].x -= (dx / d) * f;
      disp[l.from].y -= (dy / d) * f;
      disp[l.to].x += (dx / d) * f;
      disp[l.to].y += (dy / d) * f;
    }
    for (let i = 0; i < n; i++) {
      // Mild gravity keeps disconnected groups in view.
      disp[i].x += (0.5 - pos[i].x) * k * 0.5;
      disp[i].y += (0.5 - pos[i].y) * k * 0.5;
      const len = Math.max(Math.hypot(disp[i].x, disp[i].y), 1e-9);
      const step = Math.min(len, temp);
      pos[i].x = Math.min(1, Math.max(0, pos[i].x + (disp[i].x / len) * step));
      pos[i].y = Math.min(1, Math.max(0, pos[i].y + (disp[i].y / len) * step));
    }
    temp -= cool;
  }
  return normalize(pos);
}

/** Stretches the result to fill the unit box. */
function normalize(pos: Point[]): Point[] {
  const xs = pos.map((p) => p.x);
  const ys = pos.map((p) => p.y);
  const [minX, maxX, minY, maxY] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
  const w = Math.max(maxX - minX, 1e-6);
  const h = Math.max(maxY - minY, 1e-6);
  return pos.map((p) => ({ x: (p.x - minX) / w, y: (p.y - minY) / h }));
}
