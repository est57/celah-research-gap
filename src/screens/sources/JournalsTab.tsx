import type { ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useJournalTargets } from "../../hooks/useJournalTargets";
import type { Jump } from "../../lib/commands";
import { formatInt } from "../../lib/format";
import { JournalTable } from "./JournalTable";
import { t } from "../../i18n";

interface Props {
  version: number;
  onJump: (j: Jump) => void;
  onOpenSjr: () => void;
}

/** Target journals: journals of the collection that publish work on the accepted gaps. */
export function JournalsTab({ version, onJump, onOpenSjr }: Props) {
  const { data, sample, error, loading } = useJournalTargets(version);
  let body: ReactNode;
  if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? t("Mencari jurnal yang cocok dengan celah...") : ""}</ScreenMessage>;
  else if (data.documents === 0) body = <EmptyCollection />;
  else {
    const openGaps = () => onJump({ screen: "gaps", tab: "explorer" });
    body = (
      <>
        <section className="pnl">
          <div className="ph">
            {data.fallback ? t("Semua kandidat celah") : t("Celah yang diterima")}
            <span className="ph-aside">{t("{n} celah dipakai", { n: formatInt(data.gaps.length) })}</span>
          </div>
          <div className="jr-body">
            <p className="jr-text">{t("Jurnal di koleksi Anda yang menerbitkan dokumen tentang celah ini, terutama yang terbit baru-baru ini, adalah tujuan pengiriman yang masuk akal. Dokumen dihitung cocok bila judul, abstrak, atau kata kuncinya memuat istilah celah.")}</p>
            {data.fallback ? (
              <div className="notice jr-notice">
                <span>{t("Belum ada celah yang Anda terima di Gap Explorer, jadi semua kandidat celah (kecuali yang ditolak) dipakai. Terima celah yang Anda kerjakan agar rekomendasi lebih tepat.")}</span>
                <button type="button" className="btn" onClick={openGaps}>{t("Buka Gap Explorer")}</button>
              </div>
            ) : (
              <ul className="jr-used">{data.gaps.map((g) => <li key={g}>{g}</li>)}</ul>
            )}
          </div>
        </section>
        <section className="pnl">
          <div className="ph">
            {t("Peringkat jurnal tujuan")}
            <span className="ph-aside">{data.recentFrom !== null ? t("terbaru = {year} atau sesudahnya", { year: data.recentFrom }) : ""}</span>
          </div>
          {data.journals.length ? <JournalTable rows={data.journals} /> : (
            <p className="muted ent-empty">{t("Belum ada jurnal di koleksi yang memuat istilah celah ini.")}</p>
          )}
          <p className="muted ent-note">{t("Skor = (dokumen cocok + dokumen cocok terbaru) x bonus kuartil (Q1 1,2; Q2 1,1). Jurnal tanpa dokumen cocok tidak ditampilkan. Periksa ruang lingkup dan kebijakan setiap jurnal sebelum mengirim.")}</p>
          {data.status.journals ? null : (
            <div className="jr-hint">
              <span className="muted">{t("Kuartil tampil \"-\" karena daftar SJR belum diimpor.")}</span>
              <button type="button" className="btn" onClick={onOpenSjr}>{t("Buka tab Kuartil (SJR)")}</button>
            </div>
          )}
        </section>
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
