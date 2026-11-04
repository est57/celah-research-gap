import { useState } from "react";
import { ChartExport } from "../../components/ChartExport";
import { ScreenMessage } from "../../components/ScreenMessage";
import { usePrisma } from "../../hooks/usePrisma";
import { t } from "../../i18n";
import { formatInt } from "../../lib/format";
import type { Prisma } from "../../lib/screeningTypes";
import type { ScreenId } from "../../nav";
import { PrismaDiagram } from "./PrismaDiagram";

/** A methods sentence with the same figures as the diagram, ready to paste. */
function methodsText(p: Prisma): string {
  const f = (n: number) => formatInt(n);
  const sources = p.sources.filter((s) => s.label).map((s) => `${s.label} (${f(s.n)})`).join(", ");
  const filter = p.filterActive ? t(" Sebanyak {n} rekaman lagi dikecualikan oleh kriteria kelayakan.", { n: f(p.filteredOut) }) : "";
  return t("Pencarian menghasilkan {identified} rekaman dari {sources}. Setelah {dup} duplikat dan {other} rekaman lain dihapus, {screened} rekaman disaring berdasarkan judul dan abstrak, dan {excluded} di antaranya dikecualikan.{filter} Sebanyak {included} studi disertakan dalam analisis bibliometrik.", {
    identified: f(p.identified), sources: sources || "-", dup: f(p.duplicates), other: f(p.removedOther),
    screened: f(p.screened), excluded: f(p.excluded), filter, included: f(p.included),
  });
}

/** Laporan > Diagram PRISMA: the PRISMA 2020 flow from the import log and screening decisions. */
export function PrismaView({ version, onNavigate }: { version: number; onNavigate: (screen: ScreenId) => void }) {
  const { data, sample, error, loading } = usePrisma(version);
  const [copied, setCopied] = useState<string | null>(null);
  if (error) return <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  if (!data) return <ScreenMessage>{loading ? t("Menghitung alur PRISMA...") : ""}</ScreenMessage>;
  const text = methodsText(data);
  const copy = async () => {
    try {
      await navigator.clipboard.writeText(text);
      setCopied(t("Tersalin"));
    } catch {
      setCopied(t("Gagal menyalin; pilih teks lalu salin manual."));
    }
  };

  return (
    <div className="dup-page prisma-page">
      {sample ? <div className="notice">{t("Pratinjau browser: angka di halaman ini adalah data contoh.")}</div> : null}
      <section className="pnl">
        <div className="ph">
          {t("Diagram alir PRISMA 2020")}
          <ChartExport name={t("Diagram PRISMA")} />
        </div>
        <div className="prisma-wrap"><PrismaDiagram data={data} /></div>
      </section>
      {data.unscreened > 0 ? (
        <div className="notice prisma-note">
          <span>{t("{n} dokumen yang dianalisis belum disaring. Saring dulu agar diagram mencerminkan proses seleksi yang lengkap.", { n: formatInt(data.unscreened) })}</span>
          <button type="button" className="btn" onClick={() => onNavigate("collection")}>{t("Buka Skrining")}</button>
        </div>
      ) : null}
      <section className="pnl">
        <div className="ph">{t("Kalimat untuk bagian metode")}</div>
        <div className="prisma-text">
          <p>{text}</p>
          <div className="about-actions">
            <button type="button" className="btn" onClick={() => void copy()}>{t("Salin kalimat")}</button>
            {copied ? <span className="muted" role="status">{copied}</span> : null}
          </div>
          <p className="muted prisma-hint">
            {t("Angka identifikasi berasal dari catatan impor dan pencarian. Untuk koleksi yang dibuat sebelum fitur ini ada, jumlahnya diambil dari dokumen yang tersimpan, sehingga duplikat yang dilewati saat impor lama tidak terhitung.")}
          </p>
        </div>
      </section>
    </div>
  );
}
