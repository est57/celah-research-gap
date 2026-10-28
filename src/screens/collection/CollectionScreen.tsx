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
import { NumText } from "./NumText";
import { t, tk } from "../../i18n";

export type CollectionTab = "collection" | "quality" | "duplicates" | "filter" | "export";
type TabId = CollectionTab;

const TABS: TabDef<TabId>[] = [
  { id: "collection", label: tk("Koleksi") },
  { id: "quality", label: tk("Kualitas metadata") },
  { id: "duplicates", label: tk("Duplikat") },
  { id: "filter", label: tk("Filter analisis") },
  { id: "export", label: tk("Ekspor & cadangan") },
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
      <Tabs label="Data" tabs={TABS.map((d) => ({ ...d, label: t(d.label) }))} active={tab} onChange={setTab} />
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
                {t("Anda membuka pratinjau di browser. Pencarian dan penyimpanan hanya berjalan di aplikasi desktop.")}
              </div>
            ) : null}
            <div className="import-panel">
              <div style={{ display: "flex", alignItems: "baseline", gap: 10, flexWrap: "wrap" }}>
                <strong>{t("Sumber impor")}</strong>
                <span className="muted">{t("{n} dokumen unik di koleksi", { n: formatInt(stats.total) })}</span>
              </div>
              <SearchForm onImported={handleImported} />
              <FileImport onImported={handleImported} />
              <OriginSummary origins={stats.origins} total={stats.total} />
              <OriginRemove origins={stats.origins} onDeleted={handleDeleted} />
            </div>
            <div className="toolbar">
              <label htmlFor="doc-filter" className="sr-only">{t("Saring dokumen")}</label>
              <input
                id="doc-filter"
                className="input"
                type="search"
                maxLength={200}
                placeholder={t("Saring judul, penulis, kata kunci")}
                style={{ width: "min(280px, 100%)" }}
                value={filter}
                onChange={(e) => {
                  setFilter(e.target.value);
                  setPage(0);
                }}
              />
              <span className="spacer muted">
                <NumText text={t("Menampilkan {shown} dari {total}")}
                  vars={{ shown: formatInt(papers.data.items.length), total: formatInt(papers.data.total) }} />
              </span>
            </div>
            {removed !== null ? (
              <div className="notice removed-note" role="status">
                {t("{n} dokumen dihapus dari koleksi.", { n: formatInt(removed) })}
                <button type="button" className="btn" onClick={() => setRemoved(null)}>{t("Tutup")}</button>
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
