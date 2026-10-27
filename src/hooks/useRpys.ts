import { rpys } from "../lib/apiEntities";
import type { RpysParams } from "../lib/rpysTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleRpys").then((m) => m.SAMPLE_RPYS) : null;

export function useRpys(version: number, p: RpysParams) {
  return useDesktopData(() => rpys(p), sample, `${version}|${p.from}|${p.to}`);
}
