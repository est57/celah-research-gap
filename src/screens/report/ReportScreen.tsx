import { useState, type ReactNode } from "react";
import { ScreenMessage } from "../../components/ScreenMessage";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { useReport } from "../../hooks/useReport";
import { exportReport, isDesktop } from "../../lib/api";
import { DEFAULT_REPORT_PARAMS, type ExportFormat, type ReportParams } from "../../lib/reportTypes";
import type { ScreenId } from "../../nav";
import { ReportOptions } from "./ReportOptions";
import { plainText, ReportPreview, type PreviewView } from "./ReportPreview";
import { SummaryView } from "./SummaryView";

export type ReportTab = "draft" | "summary";

const TABS: TabDef<ReportTab>[] = [
  { id: "draft", label: "Draf research gap" },
  { id: "summary", label: "Draf metode & hasil" },
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
      setStatus(name ? `Tersimpan sebagai ${name}` : "Penyimpanan dibatalkan.");
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
      setCopyStatus("Tersalin");
    } catch {
      setCopyStatus("Gagal menyalin; pilih teks lalu salin manual.");
    }
  };

  let body: ReactNode;
  if (error) {
    // No accepted gap yet is the normal starting state, not an error.
    const none = error.includes("belum ada celah yang diterima");
    body = (
      <ScreenMessage>
        {none ? <strong>Belum ada celah yang diterima</strong> : null}
        <span className={none ? "muted" : undefined} role={none ? undefined : "alert"}>
          {none ? "Draf research gap disusun dari celah yang Anda tandai Diterima di Gap Explorer. Draf metode & hasil bisa disusun sekarang." : error}
        </span>
        <div className="report-empty-actions">
          <button type="button" className="btn btn-primary" onClick={() => onNavigate("gaps")}>Buka Gap Explorer</button>
          {none ? <button type="button" className="btn" onClick={() => setTab("summary")}>Draf metode &amp; hasil</button> : null}
        </div>
      </ScreenMessage>
    );
  } else if (!data) body = <ScreenMessage>{loading ? "Menyusun draf dari celah yang diterima..." : ""}</ScreenMessage>;
  else {
    body = (
      <div className="screen report-screen">
        <ReportOptions params={params} headings={data.paragraphs.map((p) => p.heading)} canExport={isDesktop()}
          busy={busy} status={status} onChange={(p) => { setParams(p); setCopyStatus(null); }}
          onExport={(f) => void save(f)} onManage={() => onNavigate("gaps")} />
        <div className="screen-main report-main">
          {sample ? <div className="notice report-sample">Pratinjau browser: draf di halaman ini dibuat dari data contoh.</div> : null}
          <ReportPreview report={data} view={view} copyStatus={copyStatus}
            onView={(v) => { setView(v); setCopyStatus(null); }} onCopy={() => void copy()} />
        </div>
      </div>
    );
  }

  return (
    <>
      <Tabs label="Laporan" tabs={TABS} active={tab} onChange={setTab} />
      {tab === "summary" ? <SummaryView version={version} defaults={defaults} /> : body}
    </>
  );
}
