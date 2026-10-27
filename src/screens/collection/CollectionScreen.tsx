import { useState } from "react";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { usePapers } from "../../hooks/usePapers";
import { formatInt } from "../../lib/format";
import { isDesktop } from "../../lib/api";
import type { CollectionStats } from "../../lib/types";
import { SearchForm } from "./SearchForm";
import { FileImport } from "./FileImport";
import { OriginSummary } from "./OriginSummary";
import { DocumentTable } from "./DocumentTable";
import { MetadataPanel } from "./MetadataPanel";
import { QualityView } from "./QualityView";
import { DuplicatesView } from "./DuplicatesView";
import { FilterView } from "./FilterView";
import { DeleteBar } from "./DeleteBar";
import { OriginRemove } from "./OriginRemove";
import { ExportView } from "./ExportView";

export type CollectionTab = "collection" | "quality" | "duplicates" | "filter" | "export";
type TabId = CollectionTab;

const TABS: TabDef<TabId>[] = [
  { id: "collection", label: "Koleksi" },
  { id: "quality", label: "Kualitas metadata" },
  { id: "duplicates", label: "Duplikat" },
  { id: "filter", label: "Filter analisis" },
  { id: "export", label: "Ekspor & cadangan" },
];

interface Props {
  stats: CollectionStats;
  statsError: string | null;
  onChanged: () => Promise<void>;
  initialTab?: CollectionTab;
  /** Prefilled document filter, from the title-bar search. */
  initialQuery?: string;
}

export function CollectionScreen({ stats, statsError, onChanged, initialTab = "collection", initialQuery = "" }: Props) {
  const [tab, setTab] = useState<TabId>(initialTab);
  const [filter, setFilter] = useState(initialQuery);
  const [page, setPage] = useState(0);
  const [version, setVersion] = useState(0);
  const papers = usePapers(filter, page, version);

  const [selected, setSelected] = useState<Set<number>>(new Set());
  const [removed, setRemoved] = useState<number | null>(null);

  const handleImported = () => {
    setVersion((v) => v + 1);
    void onChanged();
  };
  const select = (ids: number[], on: boolean) =>
    setSelected((prev) => {
      const next = new Set(prev);
      ids.forEach((id) => (on ? next.add(id) : next.delete(id)));
      return next;
    });
  const handleDeleted = (n: number) => {
    setSelected(new Set());
    setRemoved(n);
    handleImported();
  };

  return (
    <>
      <Tabs label="Data" tabs={TABS} active={tab} onChange={setTab} />
      {tab === "quality" ? (
        <QualityView version={stats.total} />
      ) : tab === "filter" ? (
        <FilterView version={stats.total} onApplied={() => void onChanged()} />
      ) : tab === "export" ? (
        <ExportView stats={stats} onChanged={() => void onChanged()} />
      ) : tab === "duplicates" ? (
        <DuplicatesView version={stats.total} onMerged={() => void onChanged()} />
      ) : (
        <div className="screen">
          <section className="screen-main">
            {!isDesktop() ? (
              <div className="notice" style={{ margin: "12px 16px 0" }}>
                Anda membuka pratinjau di browser. Pencarian dan penyimpanan hanya berjalan di aplikasi desktop.
              </div>
            ) : null}
            <div className="import-panel">
              <div style={{ display: "flex", alignItems: "baseline", gap: 10, flexWrap: "wrap" }}>
                <strong>Sumber impor</strong>
                <span className="muted">{formatInt(stats.total)} dokumen unik di koleksi</span>
              </div>
              <SearchForm onImported={handleImported} />
              <FileImport onImported={handleImported} />
              <OriginSummary origins={stats.origins} total={stats.total} />
              <OriginRemove origins={stats.origins} onDeleted={handleDeleted} />
            </div>
            <div className="toolbar">
              <label htmlFor="doc-filter" className="sr-only">Saring dokumen</label>
              <input
                id="doc-filter"
                className="input"
                type="search"
                maxLength={200}
                placeholder="Saring judul, penulis, kata kunci"
                style={{ width: "min(280px, 100%)" }}
                value={filter}
                onChange={(e) => {
                  setFilter(e.target.value);
                  setPage(0);
                }}
              />
              <span className="spacer muted">
                Menampilkan <span className="num">{formatInt(papers.data.items.length)}</span> dari{" "}
                <span className="num">{formatInt(papers.data.total)}</span>
              </span>
            </div>
            {removed !== null ? (
              <div className="notice removed-note" role="status">
                {`${formatInt(removed)} dokumen dihapus dari koleksi.`}
                <button type="button" className="btn" onClick={() => setRemoved(null)}>Tutup</button>
              </div>
            ) : null}
            <DeleteBar selected={selected} onClear={() => setSelected(new Set())} onDeleted={handleDeleted} />
            <DocumentTable data={papers.data} page={page} loading={papers.loading} error={papers.error} onPage={setPage}
              selected={selected} onSelect={select} />
          </section>
          <MetadataPanel stats={stats} error={statsError} />
        </div>
      )}
    </>
  );
}
