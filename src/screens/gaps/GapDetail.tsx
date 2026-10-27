import { formatInt } from "../../lib/format";
import { KIND_INFO, type Gap } from "../../lib/gapTypes";
import type { ScreenId } from "../../nav";
import { CheckResult } from "./CheckResult";

interface Props {
  gap: Gap;
  onNavigate: (screen: ScreenId, tab?: string) => void;
}

export function GapDetail({ gap, onNavigate }: Props) {
  return (
    <article className="gap-detail">
      <header className="gap-head">
        <div className="gap-tags">
          <span className="kind-tag" style={{ background: KIND_INFO[gap.kind].color }}>{KIND_INFO[gap.kind].label}</span>
          <span className="muted">Kepercayaan <b className="ink">{gap.confidence}</b></span>
        </div>
        <h1>{gap.title}</h1>
      </header>
      <p className="gap-statement">{gap.statement}</p>

      <section className="gap-section">
        <h2>Dibangun dari sinyal analisis</h2>
        <div className="signal-table">
          {gap.signals.map((s, i) => (
            <div key={i} className="signal-row">
              <span className="signal-name">{s.analysis}</span>
              <span>{s.finding}</span>
              <button type="button" className="link-btn" onClick={() => onNavigate(s.screen, s.tab)}>Buka</button>
            </div>
          ))}
        </div>
      </section>

      <section className="gap-section">
        <h2>Bukti dari dokumen</h2>
        {gap.evidence.map((e, i) => (
          <div key={e.paperId} className="evidence">
            <span className="num muted">[{i + 1}]</span>
            <div className="evidence-body">
              {e.quote ? <q className="evidence-quote">{e.quote}</q> : <span className="evidence-title">{e.title}</span>}
              <span className="muted evidence-src">
                {e.quote ? `${e.title} · ` : ""}
                {e.authors || "-"} · {e.year ?? "-"}
                {e.source ? ` · ${e.source}` : ""} · {formatInt(e.citedBy)} sitasi
              </span>
            </div>
            <span className="role-pill" data-role={e.role}>{e.role}</span>
          </div>
        ))}
      </section>

      {gap.check ? (
        <section className="gap-section">
          <h2>Verifikasi terhadap literatur terbaru</h2>
          <CheckResult check={gap.check} limit={3} />
        </section>
      ) : null}

      <section className="gap-section">
        <h2>Saran pertanyaan riset</h2>
        {gap.questions.map((q) => (
          <div key={q} className="rq"><span className="num muted">RQ</span><span>{q}</span></div>
        ))}
        <p className="muted gap-caveat">
          Pertanyaan ini dibuat dari pola data sebagai titik awal. Periksa dan rumuskan ulang sesuai konteks riset Anda.
        </p>
      </section>
    </article>
  );
}
