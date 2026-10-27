import { useEffect, useState } from "react";
import { isDesktop } from "../lib/api";
import { removeIdea, saveIdea, savedIdeas, type SavedIdea } from "../lib/apiEntities";

/** Ideas saved to Gap Explorer; saving or removing one reloads the list and the gap candidates. */
export function useSavedIdeas(onChanged: () => void) {
  const [ideas, setIdeas] = useState<SavedIdea[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [rev, setRev] = useState(0);

  useEffect(() => {
    if (!isDesktop()) return;
    savedIdeas().then(setIdeas).catch((err: unknown) => setError(err instanceof Error ? err.message : String(err)));
  }, [rev]);

  const run = async (task: () => Promise<unknown>) => {
    setError(null);
    try {
      await task();
      setRev((r) => r + 1);
      onChanged();
      return true;
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      return false;
    }
  };

  return {
    ideas,
    error,
    save: (a: string, b: string) => run(() => saveIdea(a, b)),
    remove: (key: string) => run(() => removeIdea(key)),
  };
}
