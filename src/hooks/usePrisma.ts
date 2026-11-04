import { prismaFlow } from "../lib/apiScreening";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleScreening").then((m) => m.SAMPLE_PRISMA) : null;

export function usePrisma(version: number) {
  return useDesktopData(prismaFlow, sample, version);
}
