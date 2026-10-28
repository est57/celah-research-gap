import { formatDecimal, formatInt } from "../../lib/format";
import { t } from "../../i18n";
import type { DocRef, Idea } from "../../lib/ideaTypes";
import { DynamicsChart } from "../sources/DynamicsChart";
import { ideaVerdict } from "./labels";

function Docs({ title, docs }: { title: string; docs: DocRef[] }) {
  return (
    <div className="idea-docs">
      <span className="opt-label">{title}</span>
      {docs.length === 0 ? <span className="muted">{t("Tidak ada.")}</span> : null}
      <ol>
        {docs.map((d) => (
          <li key={d.id}>
            <span>{d.title}</span>
            <span className="muted">{[d.authors, d.year, t("{n} sitasi", { n: formatInt(d.citedBy) })].filter(Boolean).join(" · ")}</span>
          </li>
        ))}
      </ol>
    </div>
  );
}

const pct = (v: number) => `${formatDecimal(v * 100, 0)}%`;

export function IdeaResult({ idea }: { idea: Idea }) {
  const [a, b] = [idea.a[0], idea.b[0]];
  const v = ideaVerdict(idea.verdict);
  return (
    <>
      <section className="pnl idea-verdict" data-verdict={idea.verdict}>
        <strong>{v.title}</strong>
        <p>
          {t("{na} dokumen membahas \"{a}\", {nb} membahas \"{b}\", dan {both} membahas keduanya. Bila keduanya tidak saling terkait, diperkirakan sekitar {expected} dokumen. {verdict}", {
            na: formatInt(idea.withA), a, nb: formatInt(idea.withB), b, both: formatInt(idea.withBoth),
            expected: formatDecimal(idea.expected, 1), verdict: v.text,
          })}
        </p>
        <div className="idea-stats">
          {[[t("Konsep A"), idea.withA], [t("Konsep B"), idea.withB], [t("Keduanya"), idea.withBoth]].map(([label, n]) => (
            <div key={label as string}><span className="num">{formatInt(n as number)}</span><span className="muted">{label}</span></div>
          ))}
          <div><span className="num">{formatDecimal(idea.expected, 1)}</span><span className="muted">{t("diharapkan")}</span></div>
        </div>
        <p className="muted idea-momentum">
          {t("Porsi dokumen 3 tahun terakhir: \"{a}\" {pa}, \"{b}\" {pb}, seluruh koleksi {pall}.", {
            a, pa: pct(idea.recentA), b, pb: pct(idea.recentB), pall: pct(idea.recentAll),
          })}
          {idea.recentA > idea.recentAll + 0.05 || idea.recentB > idea.recentAll + 0.05 ? ` ${t("Setidaknya satu konsep sedang naik.")}` : ""}
        </p>
      </section>
      {idea.years.length > 1 ? (
        <section className="pnl">
          <div className="ph">{t("Dokumen per tahun")}</div>
          <div className="pnl-body">
            <DynamicsChart aria={t("Dokumen per tahun untuk kedua konsep dan gabungannya")}
              data={{ years: idea.years, dynamics: [{ label: a, cumulative: idea.perYear[0] }, { label: b, cumulative: idea.perYear[1] }, { label: t("keduanya"), cumulative: idea.perYear[2] }] }} />
          </div>
        </section>
      ) : null}
      <section className="pnl idea-lists">
        <Docs title={t("Menggabungkan keduanya")} docs={idea.bothDocs} />
        <Docs title={t("Hanya \"{term}\" (paling disitasi)", { term: a })} docs={idea.aOnly} />
        <Docs title={t("Hanya \"{term}\" (paling disitasi)", { term: b })} docs={idea.bOnly} />
      </section>
    </>
  );
}
