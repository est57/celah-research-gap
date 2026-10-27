import { useMemo, useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { useGaps } from "../../hooks/useGaps";
import type { ScreenId } from "../../nav";
import { GapDetail } from "./GapDetail";
import { GapListPane, type KindFilter, type SortBy } from "./GapListPane";
import { ReviewPanel } from "./ReviewPanel";
import { VerifyView } from "./VerifyView";
import { HistoryView } from "./HistoryView";
import { IdeaView } from "./IdeaView";

export type GapsTab = "explorer" | "idea" | "verify" | "history";

const TABS: TabDef<GapsTab>[] = [
  { id: "explorer", label: "Gap Explorer" },
  { id: "idea", label: "Uji ide celah" },
  { id: "verify", label: "Verifikasi" },
  { id: "history", label: "Riwayat review" },
];

interface Props {
  version: number;
  initialKind: KindFilter;
  initialTab: GapsTab;
  onNavigate: (screen: ScreenId, tab?: string) => void;
}

export function GapsScreen({ version, initialKind, initialTab, onNavigate }: Props) {
  const [tab, setTab] = useState<GapsTab>(initialTab);
  const [kind, setKind] = useState<KindFilter>(initialKind);
  const [sort, setSort] = useState<SortBy>("score");
  const [selected, setSelected] = useState<string | null>(null);
  const { data, sample, error, loading, items, review, saveError, verify, verifying, reload } = useGaps(version);

  const shown = useMemo(() => {
    const list = items.filter((g) => kind === "all" || g.kind === kind);
    return sort === "unreviewed" ? [...list].sort((a, b) => Number(a.status !== "") - Number(b.status !== "")) : list;
  }, [items, kind, sort]);
  const current = shown.find((g) => g.key === selected) ?? shown[0];

  const openInExplorer = (key: string) => {
    setKind("all");
    setSelected(key);
    setTab("explorer");
  };

  let body: ReactNode;
  // The history lists stored reviews even when the current analysis has no candidates.
  if (tab === "history") body = <HistoryView version={version} onOpen={openInExplorer} onRemoved={reload} />;
  else if (tab === "idea") body = <IdeaView onOpen={openInExplorer} onChanged={reload} />;
  else if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? "Mencari kandidat celah dari semua analisis..." : ""}</ScreenMessage>;
  else if (data.documents === 0) body = <EmptyCollection />;
  else if (items.length === 0) {
    body = (
      <ScreenMessage>
        <strong>Belum ada kandidat celah</strong>
        <span className="muted">Butuh lebih banyak dokumen yang punya kata kunci. Tambahkan koleksi dari OpenAlex, PubMed, atau impor Scopus/WoS.</span>
      </ScreenMessage>
    );
  } else if (tab === "verify") {
    body = <VerifyView gaps={items} verifying={verifying} onVerify={verify} />;
  } else {
    body = (
      <div className="gaps-wrap">
        <div className="gaps-page">
          <GapListPane gaps={shown} all={items} selected={current?.key ?? null} kind={kind} sort={sort}
            onKind={setKind} onSort={setSort} onSelect={setSelected} />
          <div className="gap-main">
            {sample ? <div className="notice gap-sample">Pratinjau browser: kandidat di halaman ini adalah data contoh.</div> : null}
            {current ? <GapDetail gap={current} onNavigate={onNavigate} /> : null}
          </div>
          {current ? (
            <ReviewPanel gap={current} saveError={saveError} verifying={verifying}
              onReview={(s, n) => void review(current.key, s, n)} onVerify={() => verify([current.key]).then(() => undefined)} />
          ) : null}
        </div>
      </div>
    );
  }

  return (
    <>
      <Tabs label="Pencarian celah" tabs={TABS} active={tab} onChange={setTab} />
      {body}
    </>
  );
}
