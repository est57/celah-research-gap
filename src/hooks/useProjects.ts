import { useEffect, useState } from "react";
import { isDesktop } from "../lib/api";
import { createProject, deleteProject, listProjects, renameProject, switchProject, type ProjectInfo } from "../lib/apiProjects";

const PREVIEW: ProjectInfo[] = [{ id: "utama", name: "Proyek utama", documents: 0, current: true }];

/** Project list and actions; `onSwitched` runs after another project was opened. */
export function useProjects(onSwitched: () => void) {
  const [projects, setProjects] = useState<ProjectInfo[]>(PREVIEW);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (isDesktop()) listProjects().then(setProjects).catch(() => undefined);
  }, []);

  const run = async (task: () => Promise<ProjectInfo[]>, after?: () => void) => {
    if (!isDesktop()) return false;
    setBusy(true);
    setError(null);
    try {
      setProjects(await task());
      after?.();
      return true;
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      return false;
    } finally {
      setBusy(false);
    }
  };

  return {
    projects,
    current: projects.find((p) => p.current) ?? projects[0],
    busy,
    error,
    refresh: () => run(listProjects),
    create: (name: string) => run(() => createProject(name)),
    rename: (id: string, name: string) => run(() => renameProject(id, name)),
    remove: (id: string) => run(() => deleteProject(id)),
    open: (id: string) => run(() => switchProject(id), onSwitched),
  };
}
