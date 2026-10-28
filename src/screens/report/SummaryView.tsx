import { useState, type ReactNode } from "react";
import { ScreenMessage } from "../../components/ScreenMessage";
import { useSummary } from "../../hooks/useSummary";
import { t } from "../../i18n";
import { isDesktop } from "../../lib/api";
import { exportSummary, type SummaryParams } from "../../lib/apiReport";
import type { CitationStyle, ExportFormat, ReportLanguage } from "../../lib/reportTypes";
import { Choice, EXPORTS } from "./ReportOptions";
import { plainText, ReportPreview, type PreviewView } from "./ReportPreview";

/** Draft methods and results paragraphs built from the collection's figures. */
export function SummaryView({ version, defaults }: { version: number; defaults: SummaryParams }) {
  const [params, setParams] = useState<SummaryParams>(defaults);
  const [view, setView] = useState<PreviewView>("draft");
  const [busy, setBusy] = useState<ExportFormat | null>(null);
  const [status, setStatus] = useState<string | null>(null);
  const [copyStatus, setCopyStatus] = useState<string | null>(null);
  const { data, sample, error, loading } = useSummary(version, params);
  const canExport = isDesktop();

  const save = async (format: ExportFormat) => {
    setBusy(format);
    setStatus(null);
    try {
      const name = await exportSummary(params, format);
      setStatus(name ? t("Tersimpan sebagai {name}", { name }) : t("Penyimpanan dibatalkan."));
    } catch (err) {
      setStatus(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(null);
    }
  };

  const copy = async () => {
    if (!data) return;
    try {
      await navigator.clipboard.writeText(view === "draft" ? plainText(data) : view === "bibtex" ? data.bibtex : data.ris);
      setCopyStatus(t("Tersalin"));
    } catch {
      setCopyStatus(t("Gagal menyalin; pilih teks lalu salin manual."));
    }
  };

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? t("Menyusun draf metode dan hasil...") : ""}</ScreenMessage>;
  else {
    main = (
      <>
        {sample ? <div className="notice report-sample">{t("Pratinjau browser: draf di halaman ini dibuat dari data contoh.")}</div> : null}
        <ReportPreview report={data} view={view} copyStatus={copyStatus}
          onView={(v) => { setView(v); setCopyStatus(null); }} onCopy={() => void copy()} />
      </>
    );
  }

  return (
    <div className="screen report-screen">
      <aside className="report-side" aria-label={t("Opsi draf metode dan hasil")}>
        <div className="ph">{t("Opsi draf")}</div>
        <div className="report-side-body">
          <Choice<CitationStyle> label={t("Gaya sitasi")} value={params.style} options={[["apa", "APA 7"], ["ieee", "IEEE"]]}
            onPick={(style) => { setParams({ ...params, style }); setCopyStatus(null); }} />
          <Choice<ReportLanguage> label={t("Bahasa draf")} value={params.language} options={[["id", "Indonesia"], ["en", "English"]]}
            onPick={(language) => { setParams({ ...params, language }); setCopyStatus(null); }} />
          <p className="muted opt-hint">
            {t("Angka diambil dari koleksi yang sedang dianalisis, termasuk filter dan sinonim yang aktif. Bagian metode mengutip bibliometrix, dan bagian akar sejarah mengutip metode RPYS.")}
          </p>
          <div className="opt-group">
            <span className="opt-label">{t("Simpan ke file")}</span>
            {EXPORTS.map(([f, label, ext]) => (
              <button key={f} type="button" className="btn opt-export" disabled={!canExport || busy !== null || !data} onClick={() => void save(f)}>
                <span>{busy === f ? t("Menyimpan...") : t(label)}</span>
                <span className="num muted">{ext}</span>
              </button>
            ))}
            {!canExport ? <span className="muted opt-hint">{t("Ekspor file hanya tersedia di aplikasi desktop.")}</span> : null}
            {status ? <span className="opt-hint" role="status">{status}</span> : null}
          </div>
        </div>
      </aside>
      <div className="screen-main report-main">{main}</div>
    </div>
  );
}
