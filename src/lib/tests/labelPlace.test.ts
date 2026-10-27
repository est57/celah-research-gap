import { describe, expect, it } from "vitest";
import { placeAbove, placeLabels, type LabelCandidate } from "../labelPlace";

const at = (index: number, x: number, y: number, weight: number, text = "Label"): LabelCandidate => ({ index, x, y, r: 5, text, weight });

describe("placeLabels", () => {
  it("tries the other side, then drops the lightest of three crowded labels, and respects the limit", () => {
    const placed = placeLabels([at(0, 100, 100, 1), at(1, 102, 101, 9), at(2, 101, 99, 5)], 600, 400, 5);
    expect(placed.map((p) => p.index).sort()).toEqual([1, 2]);
    expect(placed.map((p) => p.anchor).sort()).toEqual(["end", "start"]);
    expect(placeLabels([at(0, 100, 100, 1), at(1, 100, 300, 2)], 600, 400, 1).map((p) => p.index)).toEqual([1]);
  });

  it("moves a label to the left side near the right edge, and drops points outside the plot", () => {
    const [p] = placeLabels([at(0, 590, 100, 1, "Long label")], 600, 400, 5);
    expect(p.anchor).toBe("end");
    expect(placeLabels([at(0, -20, 100, 1)], 600, 400, 5)).toEqual([]);
  });
});

describe("placeAbove", () => {
  it("hides a label that would cover one already shown", () => {
    const shown = placeAbove([at(0, 100, 100, 3), at(1, 110, 100, 1), at(2, 300, 100, 2)], 600, 400);
    expect([...shown].sort()).toEqual([0, 2]);
  });
});
