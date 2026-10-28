// App-level facts for the About tab (kept apart from the project calls).
import { call } from "./api";

export interface AppInfo {
  version: string;
  os: string;
  arch: string;
  sqlite: string;
  dataDir: string;
  database: string;
  databaseBytes: number;
  documents: number;
}

export function appInfo(): Promise<AppInfo> {
  return call<AppInfo>("app_info");
}

export interface UpdateInfo {
  version: string;
  current: string;
  notes: string | null;
}

/** The newer release, or null when this version is current. */
export function checkUpdate(): Promise<UpdateInfo | null> {
  return call<UpdateInfo | null>("check_update");
}

/** Downloads and installs the newer release; the app restarts when it is done. */
export function installUpdate(): Promise<void> {
  return call<void>("install_update");
}
