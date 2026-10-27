// Sample author name suggestions for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so they are left out of production builds. Made-up data.
import type { AuthorNames } from "../lib/entityTypes";

const v = (key: string, name: string, documents: number) => ({ key, name, documents });

export const SAMPLE_AUTHOR_NAMES: AuthorNames = {
  suggestions: [
    { variants: [v("sameer contoh", "Sameer Contoh", 6), v("contoh sameer", "Contoh, Sameer", 1)], sharedCoauthors: 2, sharedSources: 0, reordered: true },
    { variants: [v("rahman m", "Rahman M", 4), v("muhammad rahman", "Muhammad Rahman", 2)], sharedCoauthors: 3, sharedSources: 1, reordered: false },
    { variants: [v("andrew y contoh", "Andrew Y. Contoh", 2), v("a y contoh", "A.Y. Contoh", 1)], sharedCoauthors: 5, sharedSources: 0, reordered: false },
    { variants: [v("ziqi lee", "Ziqi Lee", 2), v("lee z m", "Lee, Z-M", 1)], sharedCoauthors: 0, sharedSources: 0, reordered: false },
  ],
  merges: [{ fromKey: "chen l", fromName: "Chen L", toKey: "lin chen", toName: "Lin Chen" }],
};
