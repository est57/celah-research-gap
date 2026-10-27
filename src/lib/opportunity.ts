import type { Theme } from "./conceptualTypes";

export type OpportunityStatus = "rising" | "declining" | "niche";

export const STATUS_LABEL: Record<OpportunityStatus, string> = {
  rising: "Sedang naik",
  declining: "Menurun",
  niche: "Niche matang",
};

export interface Opportunity {
  theme: Theme;
  status: OpportunityStatus;
  /** Share of recent documents relative to the whole collection (1 = same pace). */
  pace: number;
  /** 0..1; higher is more promising. */
  score: number;
}

/**
 * Themes worth considering as a research direction: the emerging/declining and niche quadrants
 * of the thematic map. Recency, isolation (low centrality) and evidence (documents) make the score;
 * a declining theme keeps half of it, since it may be fading rather than opening up. A theme backed
 * by fewer than three documents is scaled down in proportion, so one paper never tops the list.
 */
export function opportunities(themes: Theme[], collectionShare: number): Opportunity[] {
  return themes
    .filter((t) => t.quadrant === "emerging" || t.quadrant === "niche")
    .map((t) => {
      const pace = t.recentShare / Math.max(collectionShare, 0.05);
      const status: OpportunityStatus = t.quadrant === "niche" ? "niche" : pace > 1 ? "rising" : "declining";
      const raw = 0.45 * Math.min(pace, 2) / 2 + 0.35 * (1 - t.x) + 0.2 * Math.min(t.documents / 10, 1);
      const thin = Math.min(1, t.documents / 3);
      return { theme: t, status, pace, score: Math.round((status === "declining" ? raw / 2 : raw) * thin * 100) / 100 };
    })
    .sort((a, b) => b.score - a.score || b.theme.documents - a.theme.documents);
}
