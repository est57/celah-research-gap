import { formatDecimal, formatInt } from "../../lib/format";
import { IDEA_VERDICT, type DocRef, type Idea } from "../../lib/ideaTypes";
import { DynamicsChart } from "../sources/DynamicsChart";

function Docs({ title, docs }: { title: string; docs: DocRef[] }) {
  return (
    <div className="idea-docs">
      <span className="opt-label">{title}</span>
      {docs.length === 0 ? <span className="muted">Tidak ada.</span> : null}
      <ol>
        {docs.map((d) => (
          <li key={d.id}>
            <span>{d.title}</span>
            <span className="muted">{[d.authors, d.year, `${formatInt(d.citedBy)} sitasi`].filter(Boolean).join(" · ")}</span>
          </li>
        ))}
      </ol>
    </div>
  );
}

const pct = (v: number) => `${formatDecimal(v * 100, 0)}%`;

export function IdeaResult({ idea }: { idea: Idea }) {
  const [a, b] = [idea.a[0], idea.b[0]];
  const v = IDEA_VERDICT[idea.verdict];
  return (
    <>
      <section className="pnl idea-verdict" data-verdict={idea.verdict}>
        <strong>{v.title}</strong>
        <p>
          {`${formatInt(idea.withA)} dokumen membahas "${a}", ${formatInt(idea.withB)} membahas "${b}", dan ${formatInt(idea.withBoth)} membahas keduanya. Bila keduanya tidak saling terkait, diperkirakan sekitar ${formatDecimal(idea.expected, 1)} dokumen. ${v.text}`}
        </p>
        <div className="idea-stats">
          {[["Konsep A", idea.withA], ["Konsep B", idea.withB], ["Keduanya", idea.withBoth]].map(([label, n]) => (
            <div key={label as string}><span className="num">{formatInt(n as number)}</span><span className="muted">{label}</span></div>
          ))}
          <div><span className="num">{formatDecimal(idea.expected, 1)}</span><span className="muted">diharapkan</span></div>
        </div>
        <p className="muted idea-momentum">
          {`Porsi dokumen 3 tahun terakhir: "${a}" ${pct(idea.recentA)}, "${b}" ${pct(idea.recentB)}, seluruh koleksi ${pct(idea.recentAll)}.`}
          {idea.recentA > idea.recentAll + 0.05 || idea.recentB > idea.recentAll + 0.05 ? " Setidaknya satu konsep sedang naik." : ""}
        </p>
      </section>
      {idea.years.length > 1 ? (
        <section className="pnl">
          <div className="ph">Dokumen per tahun</div>
          <div className="pnl-body">
            <DynamicsChart aria="Dokumen per tahun untuk kedua konsep dan gabungannya"
              data={{ years: idea.years, dynamics: [{ label: a, cumulative: idea.perYear[0] }, { label: b, cumulative: idea.perYear[1] }, { label: "keduanya", cumulative: idea.perYear[2] }] }} />
          </div>
        </section>
      ) : null}
      <section className="pnl idea-lists">
        <Docs title="Menggabungkan keduanya" docs={idea.bothDocs} />
        <Docs title={`Hanya "${a}" (paling disitasi)`} docs={idea.aOnly} />
        <Docs title={`Hanya "${b}" (paling disitasi)`} docs={idea.bOnly} />
      </section>
    </>
  );
}
