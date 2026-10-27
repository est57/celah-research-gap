import type { Rpys } from "../lib/rpysTypes";

const FROM = 1970;
const TO = 2024;
const years = Array.from({ length: TO - FROM + 1 }, (_, i) => FROM + i);
const SPIKES: Record<number, number> = { 1982: 9, 1998: 14, 2006: 11, 2012: 22, 2016: 30 };
const citations = years.map((y) => Math.round(Math.exp((y - FROM) / 9)) + (SPIKES[y] ?? 0));

function deviation(c: number[]): number[] {
  return c.map((v, i) => {
    const w = c.slice(Math.max(0, i - 2), Math.min(c.length, i + 3)).sort((a, b) => a - b);
    const m = w.length % 2 ? w[(w.length - 1) / 2] : (w[w.length / 2 - 1] + w[w.length / 2]) / 2;
    return Math.round(v - m);
  });
}

const dev = deviation(citations);
const ref = (label: string, example: string, documents: number) => ({ label, example, documents });

export const SAMPLE_RPYS: Rpys = {
  documents: 495,
  citing: 312,
  dated: 9840,
  undated: 1210,
  from: FROM,
  to: TO,
  years,
  citations,
  deviation: dev,
  peaks: [2016, 2012, 1998, 2006, 1982].map((y) => ({
    year: y,
    citations: citations[y - FROM],
    deviation: dev[y - FROM],
    refs: [
      ref(`Penulis ${y}`, `PENULIS A, ${y}, JURNAL CONTOH, V1, P1`, 40 - (y % 7)),
      ref(`Peneliti ${y}`, `Peneliti B., Judul contoh rujukan, (${y}) Jurnal Lain`, 12),
    ],
  })),
};
