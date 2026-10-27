// Sample workflow progress for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so it is left out of production builds. Made-up data.
import type { WorkflowStatus } from "../lib/workflow";

export const SAMPLE_WORKFLOW: WorkflowStatus = {
  documents: 553, origins: 3, withAbstract: 512, withKeywords: 430,
  duplicates: 2, synonymSuggestions: 14, synonyms: 1, nameSuggestions: 9, nameMerges: 0,
  gaps: 12, reviewed: 5, accepted: 2, verified: 1,
  coreListed: 50,
  coreMissing: 37,
};
