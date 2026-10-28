import { formatInt } from "../../lib/format";
import type { BradfordItem, ZoneSummary } from "../../lib/sourcesTypes";
import { t } from "../../i18n";

const PLOT_H = 170;

interface Props {
  ranked: BradfordItem[];
  zones: ZoneSummary[];
}

export function BradfordChart({ ranked, zones }: Props) {
  const max = Math.max(...ranked.map((r) => r.count), 1);
  const coreShown = ranked.filter((r) => r.zone === 1).length;
  const coreWidth = ranked.length ? (coreShown / ranked.length) * 100 : 0;

  return (
    <div className="bradford">
      <div className="bradford-plot" style={{ height: PLOT_H }}>
        <div className="bradford-core" style={{ width: `${coreWidth}%` }} />
        {ranked.map((r) => (
          <div
            key={r.rank}
            className="bradford-bar"
            data-zone={r.zone}
            style={{ height: `${(r.count / max) * (PLOT_H - 8)}px` }}
            title={t("{rank}. {name}: {n} dokumen", { rank: r.rank, name: r.label, n: formatInt(r.count) })}
          />
        ))}
        {/* Drawn after the bars and beside the core zone, so the first tall bar never hides it. */}
        <span className="bradford-core-label" style={coreWidth > 75 ? { left: 6 } : { left: `calc(${coreWidth}% + 6px)` }}>
          {coreWidth > 75 ? t("Zona inti") : `\u2190 ${t("Zona inti")}`}
        </span>
      </div>
      <div className="axis-row muted">
        <span>{t("peringkat 1")}</span>
        <span>{t("sumber diurutkan dari yang paling produktif")}</span>
        <span className="num">{ranked.length}</span>
      </div>
      <div className="zone-grid">
        {zones.map((z, i) => (
          <div key={i}>
            <div className="zone-title">{t("Zona {n}", { n: i + 1 })}</div>
            <div className="num muted">
              {t("{sources} sumber, {docs} dok", { sources: formatInt(z.sources), docs: formatInt(z.documents) })}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
