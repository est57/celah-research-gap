import type { ReactNode } from "react";
import { ScreenMessage } from "../../components/ScreenMessage";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { useDesktopData } from "../../hooks/useDesktopData";
import { workflowStatus } from "../../lib/apiEntities";
import type { Jump } from "../../lib/commands";
import { STATE_LABEL, steps } from "../../lib/workflow";
import { Welcome } from "./Welcome";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../../dev/sampleWorkflow").then((m) => m.SAMPLE_WORKFLOW) : null;

const TABS: TabDef<"steps">[] = [{ id: "steps", label: "Langkah penelitian" }];

/** Home screen: the research workflow, where the project stands, and where to go next. */
export function WorkflowScreen({ version, onJump }: { version: number; onJump: (j: Jump) => void }) {
  const { data, sample: isSample, error, loading } = useDesktopData(workflowStatus, sample, version);
  let body: ReactNode;
  if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? "Memeriksa kemajuan proyek..." : ""}</ScreenMessage>;
  else {
    const list = steps(data);
    const next = list.findIndex((s) => s.state === "todo" || s.state === "attention");
    body = (
      <div className="year-page">
        {isSample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
        <div className="wf-head">
          <h1>Alur kerja penelitian</h1>
          <p className="muted">{next >= 0 ? `Langkah berikutnya: ${list[next].title.toLowerCase()}.` : "Semua langkah sudah berjalan. Perbarui laporan bila ada celah baru yang diterima."}</p>
        </div>
        {data.documents === 0 ? <Welcome onJump={onJump} /> : null}
        <ol className="wf-steps">
          {list.map((s, i) => (
            <li key={s.title} className="pnl wf-step" data-state={s.state} data-next={i === next}>
              <span className="wf-num" aria-hidden="true">{i + 1}</span>
              <div className="wf-body">
                <div className="wf-title">
                  <strong>{s.title}</strong>
                  <span className="wf-chip" data-state={s.state}>{STATE_LABEL[s.state]}</span>
                </div>
                <p className="muted">{s.text}</p>
                {s.facts.length ? <p className="wf-facts">{s.facts.join(" · ")}</p> : null}
                <div className="wf-actions">
                  {s.actions.map((a) => <button key={a.label} type="button" className="btn" onClick={() => onJump(a.jump)}>{a.label}</button>)}
                </div>
              </div>
            </li>
          ))}
        </ol>
      </div>
    );
  }
  return (
    <>
      <Tabs label="Alur kerja" tabs={TABS} active="steps" onChange={() => undefined} />
      {body}
    </>
  );
}
