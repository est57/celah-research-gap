import { describe, expect, it } from "vitest";
import type { Quadrant, Theme } from "../conceptualTypes";
import { opportunities } from "../opportunity";

const theme = (id: number, quadrant: Quadrant, recentShare: number, x: number, documents: number): Theme => ({
  id, name: `T${id}`, words: [], frequency: documents, documents, centrality: x, density: 0.5, x, y: 0.3, quadrant, recentShare,
});

describe("opportunities", () => {
  it("keeps only emerging and niche themes, and ranks a rising isolated theme first", () => {
    const list = opportunities([
      theme(1, "motor", 0.9, 0.9, 50),
      theme(2, "emerging", 0.9, 0.1, 6),
      theme(3, "emerging", 0.1, 0.1, 6),
      theme(4, "niche", 0.4, 0.3, 8),
    ], 0.4);
    expect(list.map((o) => o.theme.id)).toEqual([2, 4, 3]);
    expect(list.map((o) => o.status)).toEqual(["rising", "niche", "declining"]);
  });

  it("halves the score of a declining theme", () => {
    const [rising] = opportunities([theme(1, "emerging", 0.8, 0.2, 5)], 0.4);
    const [falling] = opportunities([theme(1, "emerging", 0.2, 0.2, 5)], 0.4);
    expect(falling.score).toBeLessThan(rising.score / 2 + 0.01);
  });

  it("does not let a theme with a single document top the list", () => {
    const list = opportunities([theme(1, "niche", 0.4, 0.0, 1), theme(2, "emerging", 0.6, 0.3, 6)], 0.4);
    expect(list[0].theme.id).toBe(2);
  });
});
