import { describe, expect, it } from "vitest";
import { steps, type WorkflowStatus } from "../workflow";

const base: WorkflowStatus = {
  documents: 0, origins: 0, withAbstract: 0, withKeywords: 0, duplicates: 0, synonymSuggestions: 0, synonyms: 0,
  nameSuggestions: 0, nameMerges: 0, gaps: 0, coreListed: 0, coreMissing: 0, reviewed: 0, accepted: 0, verified: 0,
};

describe("workflow steps", () => {
  it("starts with everything to do on an empty project", () => {
    expect(steps(base).map((s) => s.state)).toEqual(["todo", "todo", "todo", "todo", "todo", "todo"]);
  });

  it("asks for cleaning while duplicates remain and marks verification incomplete", () => {
    const s = steps({ ...base, documents: 300, origins: 2, duplicates: 4, gaps: 10, reviewed: 3, accepted: 2, verified: 1, coreListed: 50, coreMissing: 12 });
    expect(s[0].state).toBe("done");
    expect(s[1].state).toBe("attention");
    expect(s[2].facts[0]).toContain("12 dari 50");
    expect(s[4].state).toBe("attention");
    expect(s[5].state).toBe("ready");
  });

  it("calls a small collection out", () => {
    expect(steps({ ...base, documents: 20, origins: 1 })[0].state).toBe("attention");
  });
});
