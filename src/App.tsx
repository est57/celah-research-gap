import { useEffect, useState } from "react";
import { TitleBar } from "./components/shell/TitleBar";
import { NavSidebar } from "./components/shell/NavSidebar";
import { StatusBar } from "./components/shell/StatusBar";
import { useCollectionStats } from "./hooks/useCollectionStats";
import { useApiKey } from "./hooks/useApiKey";
import { useDisplay } from "./hooks/useDisplay";
import type { Jump } from "./lib/commands";
import type { ScreenId } from "./nav";
import { Routes, type JumpState } from "./Routes";

export function App() {
  const [screen, setScreen] = useState<ScreenId>("workflow");
  const [jump, setJump] = useState<JumpState>({ n: 0 });
  const [navOpen, setNavOpen] = useState(false);
  const { stats, error, loading, refresh } = useCollectionStats();
  const openalexKey = useApiKey("openalex");
  const s2Key = useApiKey("semantic_scholar");
  const ncbiKey = useApiKey("ncbi");
  const display = useDisplay();

  // Bumped on every stats refresh (imports, merges, filter changes) so analyses reload;
  // 0 means no document passes into the analyses.
  const [rev, setRev] = useState(1);
  useEffect(() => setRev((r) => r + 1), [stats]);
  const version = stats.analyzed > 0 ? rev : 0;

  /** Every navigation goes through here: sidebar, filter chip and title-bar search. */
  const go = (j: Jump) => {
    setScreen(j.screen);
    setJump((prev) => ({ tab: j.tab, query: j.query, n: prev.n + 1 }));
    setNavOpen(false);
  };
  const select = (id: ScreenId) => go({ screen: id });

  return (
    <div className="app">
      <TitleBar
        navOpen={navOpen}
        onToggleNav={() => setNavOpen((v) => !v)}
        onRefresh={() => void refresh()}
        busy={loading}
        onExport={() => select("report")}
        filter={stats.filtered ? { analyzed: stats.analyzed, total: stats.total } : null}
        onFilter={() => go({ screen: "collection", tab: "filter" })}
        onJump={go}
        onProjectSwitched={() => {
          void refresh();
          go({ screen: "workflow" });
        }}
      />
      <div className="body">
        <NavSidebar
          active={screen}
          total={stats.total}
          open={navOpen}
          onSelect={select}
          onClose={() => setNavOpen(false)}
        />
        <main className="main">
          <Routes screen={screen} jump={jump} version={version} stats={stats} statsError={error} refresh={refresh}
            select={select} go={go} display={display} keys={{ openalex: openalexKey, semanticScholar: s2Key, ncbi: ncbiKey }} />
        </main>
      </div>
      <StatusBar
        total={stats.total}
        analyzed={stats.filtered ? stats.analyzed : null}
        sources={stats.origins.map((o) => o.origin)}
        keyConfigured={openalexKey.status.configured}
      />
    </div>
  );
}
