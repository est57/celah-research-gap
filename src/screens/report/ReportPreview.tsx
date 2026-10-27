import type { Report } from "../../lib/reportTypes";

export type PreviewView = "draft" | "bibtex" | "ris";

const VIEWS: [PreviewView, string][] = [["draft", "Draf"], ["bibtex", "BibTeX"], ["ris", "RIS"]];

interface Props {
  report: Report;
  view: PreviewView;
  copyStatus: string | null;
  onView: (v: PreviewView) => void;
  onCopy: () => void;
}

export function ReportPreview({ report, view, copyStatus, onView, onCopy }: Props) {
  return (
    <section className="report-preview" aria-label="Pratinjau laporan">
      <div className="report-bar">
        <div className="opt-seg report-views" role="group" aria-label="Tampilan pratinjau">
          {VIEWS.map(([v, label]) => (
            <button key={v} type="button" aria-pressed={view === v} onClick={() => onView(v)}>{label}</button>
          ))}
        </div>
        {copyStatus ? <span className="muted report-copied" role="status">{copyStatus}</span> : null}
        <button type="button" className="btn" onClick={onCopy}>{view === "draft" ? "Salin teks" : "Salin"}</button>
      </div>
      <div className="report-scroll">
        {view === "draft" ? (
          <article className="paper">
            <h1>{report.title}</h1>
            <p>{report.intro}</p>
            {report.paragraphs.map((p, i) => (
              <section key={p.key}>
                <h2>{`${i + 1}. ${p.heading}`}</h2>
                <p>{p.text}</p>
              </section>
            ))}
            <h2>{report.referencesTitle}</h2>
            <ul className="paper-refs">
              {report.references.map((r) => <li key={r}>{r}</li>)}
            </ul>
          </article>
        ) : (
          <pre className="paper-code">{view === "bibtex" ? report.bibtex : report.ris}</pre>
        )}
      </div>
    </section>
  );
}

/** Plain text of the draft for pasting into a word processor. */
export function plainText(r: Report): string {
  const body = r.paragraphs.map((p, i) => `${i + 1}. ${p.heading}\n\n${p.text}`);
  return [r.title, r.intro, ...body, r.referencesTitle, r.references.join("\n")].join("\n\n");
}
