import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@fontsource/ibm-plex-sans/400.css";
import "@fontsource/ibm-plex-sans/500.css";
import "@fontsource/ibm-plex-sans/600.css";
import "@fontsource/ibm-plex-mono/400.css";
import "@fontsource/ibm-plex-mono/500.css";
import "@fontsource/ibm-plex-serif/400.css";
import "@fontsource/ibm-plex-serif/400-italic.css";
import "./styles/tokens.css";
import "./styles/themeDark.css";
import "./styles/base.css";
import "./styles/shell.css";
import "./styles/command.css";
import "./styles/projects.css";
import "./styles/tabs.css";
import "./styles/layout.css";
import "./styles/collection.css";
import "./styles/remove.css";
import "./styles/quality.css";
import "./styles/fieldMatrix.css";
import "./styles/duplicates.css";
import "./styles/filter.css";
import "./styles/dupRecord.css";
import "./styles/search.css";
import "./styles/table.css";
import "./styles/settings.css";
import "./styles/about.css";
import "./styles/overview.css";
import "./styles/charts.css";
import "./styles/chartExport.css";
import "./styles/cited.css";
import "./styles/years.css";
import "./styles/workflow.css";
import "./styles/welcome.css";
import "./styles/entities.css";
import "./styles/names.css";
import "./styles/sources.css";
import "./styles/sourceCharts.css";
import "./styles/timeline.css";
import "./styles/conceptual.css";
import "./styles/themes.css";
import "./styles/evolution.css";
import "./styles/factorial.css";
import "./styles/words.css";
import "./styles/structure.css";
import "./styles/countries.css";
import "./styles/historiograph.css";
import "./styles/rpys.css";
import "./styles/topRefs.css";
import "./styles/couplingMap.css";
import "./styles/gaps.css";
import "./styles/opportunity.css";
import "./styles/gapsResponsive.css";
import "./styles/gapDetail.css";
import "./styles/review.css";
import "./styles/verify.css";
import "./styles/history.css";
import "./styles/idea.css";
import "./styles/ideaSave.css";
import "./styles/report.css";
import "./styles/paper.css";
import { App } from "./App";
import { applyDisplay, loadDisplay } from "./lib/display";

// Applied before the first render so the chosen theme shows without a flash.
applyDisplay(loadDisplay());

const root = document.getElementById("root");
if (!root) throw new Error("Elemen root tidak ditemukan");

createRoot(root).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
