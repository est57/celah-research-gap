import { useState } from "react";
import { isDesktop } from "../lib/api";
import { checkIdea, testIdea } from "../lib/apiEntities";
import type { Idea, IdeaCheck } from "../lib/ideaTypes";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleIdea") : null;

/** Runs an idea test on demand, then optionally the recent-literature check. */
export function useIdea() {
  const [idea, setIdea] = useState<Idea | null>(null);
  const [check, setCheck] = useState<IdeaCheck | null>(null);
  const [busy, setBusy] = useState<"test" | "check" | null>(null);
  const [error, setError] = useState<string | null>(null);

  const run = async <T,>(kind: "test" | "check", task: () => Promise<T>, done: (r: T) => void) => {
    setBusy(kind);
    setError(null);
    try {
      done(await task());
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(null);
    }
  };

  const test = (a: string, b: string) => {
    setCheck(null);
    const task = isDesktop() || !sample ? () => testIdea(a, b) : () => sample().then((m) => m.sampleIdea(a, b));
    return run("test", task, setIdea);
  };
  const verify = (a: string, b: string) => run("check", () => checkIdea(a, b), setCheck);

  return { idea, check, busy, error, test, verify };
}
