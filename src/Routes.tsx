import type { DisplaySettings } from "./lib/display";
import type { CollectionStats } from "./lib/types";
import type { ApiKeyState } from "./hooks/useApiKey";
import type { ScreenId } from "./nav";
import { CollectionScreen, type CollectionTab } from "./screens/collection/CollectionScreen";
import { ConceptualScreen, type ConceptualTab } from "./screens/conceptual/ConceptualScreen";
import { GapsScreen, type GapsTab } from "./screens/gaps/GapsScreen";
import { OverviewScreen, type OverviewTab } from "./screens/overview/OverviewScreen";
import { Placeholder } from "./screens/Placeholder";
import { ReportScreen, type ReportTab } from "./screens/report/ReportScreen";
import { SettingsScreen, type SettingsTab } from "./screens/settings/SettingsScreen";
import { SourcesAuthorsScreen, type SourcesTabId } from "./screens/sources/SourcesAuthorsScreen";
import { StructureScreen, type StructureTab } from "./screens/structure/StructureScreen";
import { WorkflowScreen } from "./screens/workflow/WorkflowScreen";
import type { Jump } from "./lib/commands";

/** The tab and prefilled search the last navigation asked for; `n` remounts the screen. */
export interface JumpState {
  tab?: string;
  query?: string;
  n: number;
}

interface Props {
  screen: ScreenId;
  jump: JumpState;
  version: number;
  stats: CollectionStats;
  statsError: string | null;
  refresh: () => Promise<void>;
  select: (id: ScreenId) => void;
  go: (j: Jump) => void;
  display: { settings: DisplaySettings; update: (patch: Partial<DisplaySettings>) => void };
  keys: { openalex: ApiKeyState; semanticScholar: ApiKeyState; ncbi: ApiKeyState };
}

export function Routes({ screen, jump, version, stats, statsError, refresh, select, go, display, keys }: Props) {
  // Tab ids come from lib/commands.ts and match each screen's own tab type.
  const tab = <T extends string>(fallback: T): T => (jump.tab as T | undefined) ?? fallback;
  const key = `${screen}-${jump.n}`;
  const changed = () => void refresh();
  switch (screen) {
    case "workflow":
      return <WorkflowScreen key={key} version={version} onJump={go} />;
    case "collection":
      return <CollectionScreen key={key} initialTab={tab<CollectionTab>("collection")} initialQuery={jump.query} stats={stats} statsError={statsError} onChanged={refresh} />;
    case "sources":
    case "authors":
      return <SourcesAuthorsScreen key={key} version={version} initialTab={tab<SourcesTabId>(screen === "authors" ? "authors" : "overview")}
        initialQuery={jump.query} onDataChanged={changed} />;
    case "conceptual":
    case "thematic":
      return <ConceptualScreen key={key} version={version} initialTab={tab<ConceptualTab>("map")} onDataChanged={changed} />;
    case "intellectual":
    case "social":
    case "countries":
      return <StructureScreen key={key} version={version} onDataChanged={changed}
        initialTab={tab<StructureTab>(screen === "social" ? "authors" : screen === "countries" ? "countries" : "references")} />;
    case "gaps":
    case "matrix":
    case "verify":
      return <GapsScreen key={key} version={version} initialKind={screen === "matrix" ? "matrix" : "all"}
        initialTab={tab<GapsTab>(screen === "verify" ? "verify" : "explorer")} onNavigate={(s, t) => go({ screen: s, tab: t })} />;
    case "report":
      return <ReportScreen key={key} version={version} onNavigate={select} initialTab={tab<ReportTab>("draft")}
        defaults={{ style: display.settings.reportStyle, language: display.settings.reportLanguage }} />;
    case "overview":
      return <OverviewScreen key={key} version={version} initialTab={tab<OverviewTab>("main")} />;
    case "settings":
      return <SettingsScreen key={key} openalex={keys.openalex} semanticScholar={keys.semanticScholar} ncbi={keys.ncbi}
        display={display.settings} onDisplay={display.update} initialTab={tab<SettingsTab>("sources")} />;
    default:
      return <Placeholder id={screen} />;
  }
}
