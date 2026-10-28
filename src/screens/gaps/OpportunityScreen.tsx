import { useMemo, useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { useConceptual } from "../../hooks/useConceptual";
import { t, tk } from "../../i18n";
import type { Jump } from "../../lib/commands";
import { DEFAULT_PARAMS } from "../../lib/conceptualTypes";
import { formatInt } from "../../lib/format";
import { opportunities } from "../../lib/opportunity";
import { ThematicMap } from "../conceptual/ThematicMap";
import { OpportunityCard } from "./OpportunityCard";

const TABS: TabDef<"themes">[] = [{ id: "themes", label: tk("Tema peluang") }];

/** Themes from the niche and emerging/declining quadrants, ranked as possible research directions. */
export function OpportunityScreen({ version, onJump }: { version: number; onJump: (j: Jump) => void }) {
  const { data, sample, error, loading } = useConceptual(version, DEFAULT_PARAMS);
  const [selected, setSelected] = useState<number | null>(null);
  const list = useMemo(() => (data ? opportunities(data.map.themes, data.map.collectionRecentShare) : []), [data]);
  const current = list.find((o) => o.theme.id === selected) ?? list[0];

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? t("Mencari tema peluang...") : ""}</ScreenMessage>;
  else if (data.documents === 0) main = <EmptyCollection />;
  else if (list.length === 0) {
    main = <ScreenMessage>{t("Belum ada tema niche atau baru muncul. Tambahkan dokumen, atau ubah parameter di Konseptual, Peta tematik.")}</ScreenMessage>;
  } else {
    const span = `${data.map.recentFrom}-${data.map.recentTo}`;
    main = (
      <div className="conceptual-main">
        {sample ? <div className="notice">{t("Pratinjau browser: tema di halaman ini adalah data contoh.")}</div> : null}
        <p className="muted op-intro">
          {t("{n} dari {total} tema berada di kuadran niche atau baru muncul. Diurutkan dari yang paling menjanjikan: tumbuh lebih cepat dari koleksi, jarang terhubung dengan tema utama, dan didukung cukup dokumen.", {
            n: formatInt(list.length), total: formatInt(data.map.themes.length),
          })}
        </p>
        <ol className="op-list">
          {list.map((o, i) => (
            <OpportunityCard key={o.theme.id} item={o} rank={i + 1} selected={o.theme.id === current?.theme.id} recentSpan={span}
              onSelect={() => setSelected(o.theme.id)} onJump={onJump} />
          ))}
        </ol>
        <section className="pnl">
          <div className="ph">
            {t("Posisi di peta tematik")}
            <span className="ph-aside">{t("tema terpilih ditandai · klik tema di peta atau di daftar")}</span>
          </div>
          <div className="pnl-body op-map">
            <ThematicMap themes={data.map.themes} selected={current?.theme.id ?? null}
              collectionShare={data.map.collectionRecentShare} onSelect={setSelected} />
          </div>
          <p className="muted op-hint">
            {t("Kandidat ini titik awal, bukan kesimpulan. Baca dokumen tema yang dipilih, lalu uji gabungannya dengan konsep lain di Uji ide celah sebelum menetapkan arah riset. Peta lengkap dan parameternya ada di Konseptual.")}
          </p>
        </section>
      </div>
    );
  }

  return (
    <>
      <Tabs label={t("Tema peluang")} tabs={TABS.map((x) => ({ ...x, label: t(x.label) }))} active="themes" onChange={() => undefined} />
      <div className="screen">
        <section className="screen-main">{main}</section>
      </div>
    </>
  );
}
