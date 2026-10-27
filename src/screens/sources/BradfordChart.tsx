import { formatInt } from "../../lib/format";
import type { BradfordItem, ZoneSummary } from "../../lib/sourcesTypes";

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
        <div className="bradford-core" style={{ width: `${coreWidth}%` }}>
          <span>Zona inti</span>
        </div>
        {ranked.map((r) => (
          <div
            key={r.rank}
            className="bradford-bar"
            data-zone={r.zone}
            style={{ height: `${(r.count / max) * (PLOT_H - 8)}px` }}
            title={`${r.rank}. ${r.label}: ${formatInt(r.count)} dokumen`}
          />
        ))}
      </div>
      <div className="axis-row muted">
        <span>peringkat 1</span>
        <span>sumber diurutkan dari yang paling produktif</span>
        <span className="num">{ranked.length}</span>
      </div>
      <div className="zone-grid">
        {zones.map((z, i) => (
          <div key={i}>
            <div className="zone-title">Zona {i + 1}</div>
            <div className="num muted">
              {formatInt(z.sources)} sumber, {formatInt(z.documents)} dok
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
