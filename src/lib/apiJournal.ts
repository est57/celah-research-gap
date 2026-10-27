// Calls for journal quartiles from the Scimago (SJR) list (kept apart to keep api.ts short).
import { call } from "./api";
import type { Quartiles, SjrStatus } from "./quartileTypes";

/** Opens the native file picker; resolves to the new status, or null when cancelled. */
export function importSjr(): Promise<SjrStatus | null> {
  return call<SjrStatus | null>("import_sjr");
}

export function clearSjr(): Promise<SjrStatus> {
  return call<SjrStatus>("clear_sjr");
}

export function quartiles(): Promise<Quartiles> {
  return call<Quartiles>("quartiles");
}
