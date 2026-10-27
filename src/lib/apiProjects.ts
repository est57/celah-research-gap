// Projects: one database each. Every call resolves to the updated project list.
import { call } from "./api";

export interface ProjectInfo {
  id: string;
  name: string;
  documents: number;
  current: boolean;
}

export const listProjects = () => call<ProjectInfo[]>("list_projects");
export const createProject = (name: string) => call<ProjectInfo[]>("create_project", { name });
export const renameProject = (id: string, name: string) => call<ProjectInfo[]>("rename_project", { id, name });
export const deleteProject = (id: string) => call<ProjectInfo[]>("delete_project", { id });
/** Opens another project; every screen should reload afterwards. */
export const switchProject = (id: string) => call<ProjectInfo[]>("switch_project", { id });
