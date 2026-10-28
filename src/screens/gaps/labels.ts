// Labels kept as Indonesian constants in src/lib (gapTypes, ideaTypes, opportunity, marked
// with tk() there) are translated where they render.
import { t, tk } from "../../i18n";
import { KIND_INFO, STATUS_LABEL, VERDICT_LABEL, type GapKind, type ReviewStatus, type Verdict } from "../../lib/gapTypes";
import { IDEA_VERDICT, type IdeaVerdict } from "../../lib/ideaTypes";
import { STATUS_LABEL as OP_STATUS, type OpportunityStatus } from "../../lib/opportunity";

// Fixed values from the backend, kept in Indonesian there: evidence roles and gap confidence.
export const VALUE_KEYS = [
  tk("mendukung"), tk("hampir menjawab"), tk("konteks"),
  tk("Kuat"), tk("Sedang"), tk("Lemah"),
];

export const kindLabel = (k: GapKind): string => t(KIND_INFO[k].label);
export const statusLabel = (s: Exclude<ReviewStatus, "">): string => t(STATUS_LABEL[s]);
export const verdictLabel = (v: Verdict): string => t(VERDICT_LABEL[v]);
export const opStatusLabel = (s: OpportunityStatus): string => t(OP_STATUS[s]);

export function ideaVerdict(v: IdeaVerdict): { title: string; text: string } {
  return { title: t(IDEA_VERDICT[v].title), text: t(IDEA_VERDICT[v].text) };
}
