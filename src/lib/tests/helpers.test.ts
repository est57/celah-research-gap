import { describe, expect, it } from "vitest";
import { matchCommands } from "../commands";
import { forceLayout } from "../forceLayout";
import { convexHull } from "../hull";
import { formatInt, formatPct, setNumberLocale } from "../format";

describe("command search", () => {
  it("needs every word and ranks label prefixes first", () => {
    const hits = matchCommands("rujukan");
    expect(hits[0].label).toBe("Rujukan teratas");
    expect(matchCommands("rujukan tidakada")).toEqual([]);
    expect(matchCommands("   ")).toEqual([]);
  });
});

describe("force layout", () => {
  it("is deterministic and stays inside the unit box", () => {
    const links = [{ from: 0, to: 1, weight: 1 }, { from: 1, to: 2, weight: 0.5 }];
    const a = forceLayout(4, links);
    expect(forceLayout(4, links)).toEqual(a);
    expect(a.every((p) => p.x >= 0 && p.x <= 1 && p.y >= 0 && p.y <= 1)).toBe(true);
  });
});

describe("convex hull", () => {
  it("drops interior points", () => {
    const hull = convexHull([[0, 0], [2, 0], [2, 2], [0, 2], [1, 1]]);
    expect(hull).toHaveLength(4);
    expect(hull).not.toContainEqual([1, 1]);
  });
});

describe("number format", () => {
  it("follows the chosen locale", () => {
    setNumberLocale("id");
    expect(formatInt(12345)).toBe("12.345");
    expect(formatPct(1, 4)).toBe("25,0%");
    setNumberLocale("en");
    expect(formatInt(12345)).toBe("12,345");
    setNumberLocale("id");
  });
});
