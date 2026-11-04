// Calls for screening decisions and the PRISMA flow (kept apart to keep api.ts short).
import { call } from "./api";
import type { Decision, Prisma, Reason, ScreenPage, ScreenStatus } from "./screeningTypes";

export function screeningPage(status: ScreenStatus, query: string, limit: number, offset: number): Promise<ScreenPage> {
  return call<ScreenPage>("screening_page", { input: { status, query: query.trim() || null, limit, offset } });
}

/** Sets the decision of the given documents; `decision` null clears it. Resolves to the number changed. */
export function setScreening(ids: number[], decision: Decision | null, reason: Reason | "" = "", note = ""): Promise<number> {
  return call<number>("set_screening", { input: { ids, decision, reason, note } });
}

export function prismaFlow(): Promise<Prisma> {
  return call<Prisma>("prisma_flow");
}
