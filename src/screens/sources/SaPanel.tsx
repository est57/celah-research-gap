import type { ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useSourcesAuthors } from "../../hooks/useSourcesAuthors";
import type { SourcesAuthors } from "../../lib/sourcesTypes";

/** Titled panel of the source and author overviews; `wide` spans both grid columns. */
export function Panel({ title, aside, wide, children }: { title: string; aside?: string; wide?: boolean; children: ReactNode }) {
  return (
    <section className={wide ? "pnl sa-wide" : "pnl"}>
      <div className="ph">
        {title}
        {aside ? <span className="ph-aside">{aside}</span> : null}
      </div>
      <div className="pnl-body">{children}</div>
    </section>
  );
}

/** Loads the shared source and author figures and renders `content` once they are there; `stack` uses one column. */
export function SaOverview({ version, content, stack }: { version: number; content: (d: SourcesAuthors) => ReactNode; stack?: boolean }) {
  const { data, sample, error, loading } = useSourcesAuthors(version);
  if (error) return <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  if (!data) return <ScreenMessage>{loading ? "Menghitung analisis sumber dan penulis..." : ""}</ScreenMessage>;
  if (data.totalSources === 0 && data.lotka.totalAuthors === 0) return <EmptyCollection />;
  return (
    <div className="sa-page">
      {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
      <div className={stack ? "sa-grid sa-stack" : "sa-grid"}>{content(data)}</div>
    </div>
  );
}
