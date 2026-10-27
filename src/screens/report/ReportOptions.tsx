import type { CitationStyle, ExportFormat, ReportLanguage, ReportParams } from "../../lib/reportTypes";

interface Props {
  params: ReportParams;
  headings: string[];
  canExport: boolean;
  busy: ExportFormat | null;
  status: string | null;
  onChange: (p: ReportParams) => void;
  onExport: (f: ExportFormat) => void;
  onManage: () => void;
}

export function Choice<T extends string>({ label, value, options, onPick }: { label: string; value: T; options: [T, string][]; onPick: (v: T) => void }) {
  return (
    <div className="opt-group" role="group" aria-label={label}>
      <span className="opt-label">{label}</span>
      <div className="opt-seg">
        {options.map(([v, text]) => (
          <button key={v} type="button" aria-pressed={value === v} onClick={() => onPick(v)}>{text}</button>
        ))}
      </div>
    </div>
  );
}

export const EXPORTS: [ExportFormat, string, string][] = [
  ["markdown", "Draf Markdown", ".md"],
  ["bibtex", "BibTeX", ".bib"],
  ["ris", "RIS (Zotero, Mendeley)", ".ris"],
];

export function ReportOptions({ params, headings, canExport, busy, status, onChange, onExport, onManage }: Props) {
  return (
    <aside className="report-side" aria-label="Opsi laporan">
      <div className="ph">Opsi laporan</div>
      <div className="report-side-body">
        <Choice<CitationStyle> label="Gaya sitasi" value={params.style} options={[["apa", "APA 7"], ["ieee", "IEEE"]]}
          onPick={(style) => onChange({ ...params, style })} />
        <Choice<ReportLanguage> label="Bahasa draf" value={params.language} options={[["id", "Indonesia"], ["en", "English"]]}
          onPick={(language) => onChange({ ...params, language })} />
        <label className="opt-check">
          <input type="checkbox" checked={params.includeVerification}
            onChange={(e) => onChange({ ...params, includeVerification: e.target.checked })} />
          <span>Sertakan hasil verifikasi literatur terbaru</span>
        </label>

        <div className="opt-group">
          <span className="opt-label">{`Celah yang dimasukkan (${headings.length})`}</span>
          <ol className="opt-gaps">
            {headings.map((h, i) => <li key={`${i}-${h}`}>{h}</li>)}
          </ol>
          <button type="button" className="btn opt-link" onClick={onManage}>Kelola di Gap Explorer</button>
        </div>

        <div className="opt-group">
          <span className="opt-label">Simpan ke file</span>
          {EXPORTS.map(([f, label, ext]) => (
            <button key={f} type="button" className="btn opt-export" disabled={!canExport || busy !== null} onClick={() => onExport(f)}>
              <span>{busy === f ? "Menyimpan..." : label}</span>
              <span className="num muted">{ext}</span>
            </button>
          ))}
          {!canExport ? <span className="muted opt-hint">Ekspor file hanya tersedia di aplikasi desktop.</span> : null}
          {status ? <span className="opt-hint" role="status">{status}</span> : null}
        </div>
      </div>
    </aside>
  );
}
