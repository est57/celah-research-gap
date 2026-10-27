import { formatInt } from "../../lib/format";
import { isEmergingSignal, QUADRANT_LABEL, type Theme } from "../../lib/conceptualTypes";

const COLORS = ["#2456A6", "#2E8B6E", "#8C7A3E", "#7A5BB0", "#B8457A", "#D9622B", "#3B8FB5", "#6B7480", "#4F6D2E", "#9C4F2E"];

interface Props {
  themes: Theme[];
  selected: number | null;
  collectionShare: number;
  onSelect: (id: number) => void;
}

export function themeColor(id: number): string {
  return COLORS[id % COLORS.length];
}

export function ThematicMap({ themes, selected, collectionShare, onSelect }: Props) {
  const maxFreq = Math.max(...themes.map((t) => t.frequency), 1);
  return (
    <div className="tmap">
      <div className="tmap-ylabel muted">Densitas (tingkat perkembangan)</div>
      <div className="tmap-plot">
        <div className="tmap-mid-x" />
        <div className="tmap-mid-y" />
        <span className="tmap-q" style={{ left: 10, top: 8 }}>{QUADRANT_LABEL.niche}</span>
        <span className="tmap-q" style={{ right: 10, top: 8 }}>{QUADRANT_LABEL.motor}</span>
        <span className="tmap-q tmap-q-gap" style={{ left: 10, bottom: 8 }}>{QUADRANT_LABEL.emerging}</span>
        <span className="tmap-q" style={{ right: 10, bottom: 8 }}>{QUADRANT_LABEL.basic}</span>
        <div className="tmap-area">
          {themes.map((t) => {
            const size = 16 + 44 * Math.sqrt(t.frequency / maxFreq);
            const on = t.id === selected;
            const style = { left: `${t.x * 100}%`, top: `${(1 - t.y) * 100}%` };
            return (
              <div key={t.id} className="tmap-item" style={style}>
                <button
                  type="button"
                  className="tmap-bubble"
                  data-on={on}
                  data-signal={isEmergingSignal(t, collectionShare)}
                  aria-pressed={on}
                  aria-label={`${t.name}, ${QUADRANT_LABEL[t.quadrant]}, ${formatInt(t.documents)} dokumen`}
                  style={{ width: size, height: size, background: themeColor(t.id), color: themeColor(t.id) }}
                  onClick={() => onSelect(t.id)}
                />
                <div className="tmap-label" style={{ top: size / 2 + 4 }}>
                  {t.words.slice(0, 1).map((w) => (
                    <strong key={w.label}>{w.label}</strong>
                  ))}
                  <span className="muted">{t.words.slice(1, 3).map((w) => w.label).join(" · ")}</span>
                </div>
              </div>
            );
          })}
        </div>
      </div>
      <span />
      <div className="tmap-xlabel muted">Sentralitas (tingkat relevansi)</div>
    </div>
  );
}
