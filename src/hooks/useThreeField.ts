import { threeField } from "../lib/api";
import type { ThreeFieldParams } from "../lib/overviewTypes";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleOverview").then((m) => m.SAMPLE_OVERVIEW.threeField) : null;

export function useThreeField(version: number, p: ThreeFieldParams) {
  return useDesktopData(() => threeField(p), sample, `${version}|${p.left}|${p.middle}|${p.right}|${p.items}`);
}
