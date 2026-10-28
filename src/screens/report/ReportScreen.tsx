import { useState, type ReactNode } from "react";
import { ScreenMessage } from "../../components/ScreenMessage";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { useReport } from "../../hooks/useReport";
import { t, tk } from "../../i18n";
import { exportReport, isDesktop } from "../../lib/api";
import { DEFAULT_REPORT_PARAMS, type ExportFormat, type ReportParams } from "../../lib/reportTypes";
import type { ScreenId } from "../../nav";
import { ReportOptions } from "./ReportOptions";
import { plainText, ReportPreview, type PreviewView } from "./ReportPreview";
import { SummaryView } from "./SummaryView";

export type ReportTab = "draft" | "summary";

const TABS: TabDef<ReportTab>[] = [
  { id: "draft", label: tk("Draf research gap") },
  { id: "summary", label: tk("Draf metode & hasil") },
];

interface Props {
  version: number;
  onNavigate: (screen: ScreenId) => void;
  /** Citation style and language from the display settings. */
  defaults: Pick<ReportParams, "style" | "language">;
  initialTab?: ReportTab;
}

export function ReportScreen({ version, onNavigate, defaults, initialTab = "draft" }: Props) {
  const [tab, setTab] = useState<ReportTab>(initialTab);
  const [params, setParams] = useState<ReportParams>({ ...DEFAULT_REPORT_PARAMS, ...defaults });
  const [view, setView] = useState<PreviewView>("draft");
  const [busy, setBusy] = useState<ExportFormat | null>(null);
  const [status, setStatus] = useState<string | null>(null);
  const [copyStatus, setCopyStatus] = useState<string | null>(null);
  const { data, sample, error, loading } = useReport(version, params);

  const save = async (format: ExportFormat) => {
    setBusy(format);
    setStatus(null);
    try {
      const name = await exportReport(params, format);
      setStatus(name ? t("Tersimpan sebagai {name}", { name }) : t("Penyimpanan dibatalkan."));
    } catch (err) {
      setStatus(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(null);
    }
  };

  const copy = async () => {
    if (!data) return;
    const text = view === "draft" ? plainText(data) : view === "bibtex" ? data.bibtex : data.ris;
    try {
      await navigator.clipboard.writeText(text);
      setCopyStatus(t("Tersalin"));
    } catch {
      setCopyStatus(t("Gagal menyalin; pilih teks lalu salin manual."));
    }
  };

  let body: ReactNode;
  if (error) {
    // No accepted gap yet is the normal starting state, not an error.
    // The backend message follows the interface language (src-tauri/src/report/mod.rs).
    const none = /belum ada celah yang diterima|no accepted gap/i.test(error);
    body = (
      <ScreenMessage>
        {none ? <strong>{t("Belum ada celah yang diterima")}</strong> : null}
        <span className={none ? "muted" : undefined} role={none ? undefined : "alert"}>
          {none ? t("Draf research gap disusun dari celah yang Anda tandai Diterima di Gap Explorer. Draf metode & hasil bisa disusun sekarang.") : error}
        </span>
        <div className="report-empty-actions">
          <button type="button" className="btn btn-primary" onClick={() => onNavigate("gaps")}>{t("Buka Gap Explorer")}</button>
          {none ? <button type="button" className="btn" onClick={() => setTab("summary")}>{t("Draf metode & hasil")}</button> : null}
        </div>
      </ScreenMessage>
    );
  } else if (!data) body = <ScreenMessage>{loading ? t("Menyusun draf dari celah yang diterima...") : ""}</ScreenMessage>;
  else {
    body = (
      <div className="screen report-screen">
        <ReportOptions params={params} headings={data.paragraphs.map((p) => p.heading)} canExport={isDesktop()}
          busy={busy} status={status} onChange={(p) => { setParams(p); setCopyStatus(null); }}
          onExport={(f) => void save(f)} onManage={() => onNavigate("gaps")} />
        <div className="screen-main report-main">
          {sample ? <div className="notice report-sample">{t("Pratinjau browser: draf di halaman ini dibuat dari data contoh.")}</div> : null}
          <ReportPreview report={data} view={view} copyStatus={copyStatus}
            onView={(v) => { setView(v); setCopyStatus(null); }} onCopy={() => void copy()} />
        </div>
      </div>
    );
  }

  return (
    <>
      <Tabs label={t("Laporan")} tabs={TABS.map((x) => ({ ...x, label: t(x.label) }))} active={tab} onChange={setTab} />
      {tab === "summary" ? <SummaryView version={version} defaults={defaults} /> : body}
    </>
  );
}
