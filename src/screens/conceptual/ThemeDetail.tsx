import { formatDecimal, formatInt } from "../../lib/format";
import { isEmergingSignal, QUADRANT_LABEL, type Theme } from "../../lib/conceptualTypes";
import { themeColor } from "./ThematicMap";

interface Props {
  theme: Theme;
  collectionShare: number;
  recentFrom: number;
  recentTo: number;
}

function signal(t: Theme, collectionShare: number, period: string): string | null {
  const pct = (v: number) => `${formatDecimal(v * 100, 0)}%`;
  if (isEmergingSignal(t, collectionShare)) {
    return `Kemungkinan tema baru muncul: ${pct(t.recentShare)} dokumennya terbit ${period}, di atas rata-rata koleksi (${pct(collectionShare)}). Sentralitasnya rendah, artinya tema ini masih jarang terhubung dengan tema utama.`;
  }
  if (t.quadrant === "emerging") {
    return `Kemungkinan tema menurun: hanya ${pct(t.recentShare)} dokumennya terbit ${period} (rata-rata koleksi ${pct(collectionShare)}).`;
  }
  if (t.quadrant === "niche") {
    return "Tema niche: berkembang di dalam kelompoknya sendiri tetapi jarang terhubung dengan tema lain. Kaitannya dengan tema motor bisa menjadi celah riset.";
  }
  return null;
}

export function ThemeDetail({ theme, collectionShare, recentFrom, recentTo }: Props) {
  const text = signal(theme, collectionShare, `${recentFrom}-${recentTo}`);
  return (
    <div className="theme-detail">
      <div className="theme-head">
        <span className="theme-swatch" style={{ background: themeColor(theme.id) }} />
        <strong>{theme.words[0]?.label ?? theme.name}</strong>
      </div>
      <div className="muted theme-quadrant">{QUADRANT_LABEL[theme.quadrant]}</div>
      <div className="theme-stats">
        <div><div className="num">{formatInt(theme.documents)}</div><div className="muted">dokumen</div></div>
        <div><div className="num">{formatDecimal(theme.centrality, 2)}</div><div className="muted">sentralitas</div></div>
        <div><div className="num">{formatDecimal(theme.density, 2)}</div><div className="muted">densitas</div></div>
      </div>
      <ul className="theme-words">
        {theme.words.map((w) => (
          <li key={w.label}>
            <span>{w.label}</span>
            <span className="num muted">{formatInt(w.freq)}</span>
          </li>
        ))}
      </ul>
      {text ? (
        <div className="notice theme-signal">
          <strong>Sinyal celah</strong>
          <span>{text}</span>
        </div>
      ) : null}
    </div>
  );
}
