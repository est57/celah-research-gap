import type { ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useQuartiles } from "../../hooks/useQuartiles";
import { formatDecimal, formatInt, formatPct } from "../../lib/format";
import { QUARTILE_LABELS, type QuartileSource } from "../../lib/quartileTypes";
import { EntityTable, type Column } from "./EntityTable";
import { SjrPanel } from "./SjrPanel";
import { t, tk } from "../../i18n";

const COLUMNS: Column<QuartileSource>[] = [
  { label: tk("Kuartil"), cell: (r) => <span className="q-chip" data-q={r.quartile || "none"}>{r.quartile || "-"}</span>, sort: (r) => (r.quartile ? 5 - Number(r.quartile[1]) : 0) },
  { label: "SJR", cell: (r) => (r.sjr !== null ? formatDecimal(r.sjr, 2) : "-"), sort: (r) => r.sjr ?? -1 },
  { label: tk("Dok"), cell: (r) => formatInt(r.documents), sort: (r) => r.documents },
];

/** Journal quartiles of the collection from the imported SJR list. */
export function QuartilesTab({ version, onDataChanged }: { version: number; onDataChanged: () => void }) {
  const { data, sample, error, loading } = useQuartiles(version);
  let body: ReactNode;
  if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? t("Mencocokkan kuartil jurnal...") : ""}</ScreenMessage>;
  else if (data.documents === 0) body = <EmptyCollection />;
  else {
    const max = Math.max(1, ...data.counts);
    body = (
      <>
        <SjrPanel status={data.status} onChanged={onDataChanged} />
        {data.status.journals ? (
          <>
            <section className="pnl">
              <div className="ph">{t("Sebaran dokumen per kuartil")}<span className="ph-aside">{t("{n} dokumen dianalisis", { n: formatInt(data.documents) })}</span></div>
              <ol className="q-bars">
                {data.counts.map((n, i) => (
                  <li key={QUARTILE_LABELS[i]}>
                    <span className="q-chip" data-q={i < 4 ? QUARTILE_LABELS[i] : "none"}>{t(QUARTILE_LABELS[i])}</span>
                    <span className="bar-track"><span className="bar-fill" data-q={i < 4 ? QUARTILE_LABELS[i] : "none"} style={{ width: `${(n / max) * 100}%` }} /></span>
                    <span className="num">{formatInt(n)}</span>
                    <span className="num muted">{formatPct(n, data.documents)}</span>
                  </li>
                ))}
              </ol>
            </section>
            <section className="pnl">
              <div className="ph">{t("Sumber dan kuartilnya")}<span className="ph-aside">{t("sumber paling banyak di koleksi")}</span></div>
              <EntityTable id="qsrc" label={t("Sumber")} rows={data.sources} name={(r) => r.label} columns={COLUMNS} initialSort={2} />
              <p className="muted ent-note">{t("\"-\" berarti sumber tidak ditemukan di daftar SJR, misalnya prosiding, jurnal nasional, atau ISSN yang belum tersimpan. Impor ulang ekspor Scopus untuk melengkapi ISSN dokumen lama.")}</p>
            </section>
          </>
        ) : null}
      </>
    );
  }
  return (
    <div className="year-page">
      {sample ? <div className="notice">{t("Pratinjau browser: angka di halaman ini adalah data contoh.")}</div> : null}
      {body}
    </div>
  );
}
