import { formatDecimal, formatInt } from "../../lib/format";
import type { LotkaRow } from "../../lib/sourcesTypes";
import { t } from "../../i18n";

const PLOT_H = 160;
const GROUPS = 5;

interface Group {
  label: string;
  observed: number;
  theoretical: number;
}

/** Classic Lotka share of authors with n documents: C / n^2, C = 6 / pi^2 (shares sum to 1). */
const classic = (n: number) => 6 / Math.PI ** 2 / n ** 2;

/** Folds rows into 1, 2, 3, 4 and "5+" documents per author. */
function group(rows: LotkaRow[]): Group[] {
  const out: Group[] = Array.from({ length: GROUPS }, (_, i) => {
    const n = i + 1;
    return { label: n === GROUPS ? `${GROUPS}+` : String(n), observed: 0, theoretical: classic(n) };
  });
  // The last group takes the whole theoretical tail (n >= GROUPS).
  const head = out.slice(0, -1).reduce((sum, g) => sum + g.theoretical, 0);
  out[GROUPS - 1].theoretical = 1 - head;
  for (const r of rows) {
    out[Math.min(r.documents, GROUPS) - 1].observed += r.observed;
  }
  return out;
}

interface Props {
  rows: LotkaRow[];
  totalAuthors: number;
  beta: number | null;
}

export function LotkaChart({ rows, totalAuthors, beta }: Props) {
  const groups = group(rows);
  const max = Math.max(...groups.flatMap((g) => [g.observed, g.theoretical]), 0.01);
  const pct = (v: number) => `${formatDecimal(v * 100)}%`;

  return (
    <div className="lotka">
      <div className="lotka-plot" style={{ height: PLOT_H }}>
        {groups.map((g) => {
          const bar = (g.observed / max) * (PLOT_H - 20);
          const theory = (g.theoretical / max) * (PLOT_H - 20);
          return (
            <div key={g.label} className="lotka-col" title={t("{n} dokumen: {pct} penulis", { n: g.label, pct: pct(g.observed) })}>
              <div className="lotka-bar" style={{ height: bar }} />
              <div className="lotka-theory" style={{ bottom: theory }} />
              {/* Above whichever is higher, the bar or the dashed theory line, so neither covers the number. */}
              <span className="num lotka-value" style={{ bottom: Math.max(bar, theory) + 4 }}>{pct(g.observed)}</span>
            </div>
          );
        })}
      </div>
      <div className="lotka-axis">
        {groups.map((g) => (
          <span key={g.label} className="num muted">{g.label}</span>
        ))}
      </div>
      <div className="legend-row muted">
        <span><span className="swatch" style={{ background: "var(--accent)" }} />{t("teramati")}</span>
        <span><span className="dash" />{t("teoretis (beta = 2)")}</span>
        <span className="legend-right">{t("dokumen per penulis")}</span>
      </div>
      <p className="muted chart-note">
        {t("{n} penulis.", { n: formatInt(totalAuthors) })}
        {beta !== null ? ` ${t("Beta terukur pada koleksi ini {beta}.", { beta: formatDecimal(beta, 2) })}` : ""}
        {` ${t("Beta di atas 2 berarti produktivitas lebih terpusat pada sedikit penulis.")}`}
      </p>
    </div>
  );
}
