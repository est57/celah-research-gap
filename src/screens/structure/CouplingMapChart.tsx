import { useRef } from "react";
import { useHideOverlaps } from "../../hooks/useHideOverlaps";
import { formatDecimal, formatInt } from "../../lib/format";
import { COUPLING_QUADRANT, type CouplingCluster } from "../../lib/couplingMapTypes";
import { clusterColor } from "../../lib/structureTypes";
import { t } from "../../i18n";

interface Props {
  clusters: CouplingCluster[];
  selected: number | null;
  onSelect: (id: number) => void;
}

/** Strategic diagram of coupling clusters, drawn with the thematic map's styles. */
export function CouplingMapChart({ clusters, selected, onSelect }: Props) {
  const maxSize = Math.max(...clusters.map((c) => c.size), 1);
  const plot = useRef<HTMLDivElement>(null);
  useHideOverlaps(plot, ".tmap-q, .tmap-label", [clusters, selected]);
  return (
    <div className="tmap">
      <div className="tmap-ylabel muted">{t("Dampak (sitasi ternormalisasi per tahun)")}</div>
      <div className="tmap-plot" ref={plot}>
        <div className="tmap-mid-x" />
        <div className="tmap-mid-y" />
        <span className="tmap-q" data-priority={1e9} style={{ left: 10, top: 8 }}>{t(COUPLING_QUADRANT.niche)}</span>
        <span className="tmap-q" data-priority={1e9} style={{ right: 10, top: 8 }}>{t(COUPLING_QUADRANT.motor)}</span>
        <span className="tmap-q tmap-q-gap" data-priority={1e9} style={{ left: 10, bottom: 8 }}>{t(COUPLING_QUADRANT.peripheral)}</span>
        <span className="tmap-q" data-priority={1e9} style={{ right: 10, bottom: 8 }}>{t(COUPLING_QUADRANT.basic)}</span>
        <div className="tmap-area">
          {clusters.map((c) => {
            const size = 16 + 44 * Math.sqrt(c.size / maxSize);
            const on = c.id === selected;
            return (
              <div key={c.id} className="tmap-item" style={{ left: `${c.x * 100}%`, top: `${(1 - c.y) * 100}%` }}>
                <button type="button" className="tmap-bubble" data-on={on} aria-pressed={on}
                  aria-label={t("{name}, {quadrant}, {n} dokumen, dampak {impact}", { name: c.keywords[0] ?? t("Klaster {n}", { n: c.id + 1 }), quadrant: t(COUPLING_QUADRANT[c.quadrant]), n: formatInt(c.size), impact: formatDecimal(c.impact, 2) })}
                  style={{ width: size, height: size, background: clusterColor(c.id), color: clusterColor(c.id) }}
                  onClick={() => onSelect(c.id)} />
                <div className="tmap-label" style={{ top: size / 2 + 4 }} data-priority={on ? 1e8 : c.size}>
                  <strong>{c.keywords[0] ?? t("Klaster {n}", { n: c.id + 1 })}</strong>
                  <span className="muted">{c.keywords.slice(1, 3).join(" · ")}</span>
                </div>
              </div>
            );
          })}
        </div>
      </div>
      <span />
      <div className="tmap-xlabel muted">{t("Sentralitas (keterhubungan dengan klaster lain)")}</div>
    </div>
  );
}
