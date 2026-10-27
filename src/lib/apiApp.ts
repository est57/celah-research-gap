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
