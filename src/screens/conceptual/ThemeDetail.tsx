import { formatDecimal, formatInt } from "../../lib/format";
import { isEmergingSignal, QUADRANT_LABEL, type Theme } from "../../lib/conceptualTypes";
import { themeColor } from "./ThematicMap";
import { t } from "../../i18n";

interface Props {
  theme: Theme;
  collectionShare: number;
  recentFrom: number;
  recentTo: number;
}

function signal(th: Theme, collectionShare: number, period: string): string | null {
  const pct = (v: number) => `${formatDecimal(v * 100, 0)}%`;
  if (isEmergingSignal(th, collectionShare)) {
    return t("Kemungkinan tema baru muncul: {share} dokumennya terbit {period}, di atas rata-rata koleksi ({avg}). Sentralitasnya rendah, artinya tema ini masih jarang terhubung dengan tema utama.", { share: pct(th.recentShare), period, avg: pct(collectionShare) });
  }
  if (th.quadrant === "emerging") {
    return t("Kemungkinan tema menurun: hanya {share} dokumennya terbit {period} (rata-rata koleksi {avg}).", { share: pct(th.recentShare), period, avg: pct(collectionShare) });
  }
  if (th.quadrant === "niche") {
    return t("Tema niche: berkembang di dalam kelompoknya sendiri tetapi jarang terhubung dengan tema lain. Kaitannya dengan tema motor bisa menjadi celah riset.");
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
      <div className="muted theme-quadrant">{t(QUADRANT_LABEL[theme.quadrant])}</div>
      <div className="theme-stats">
        <div><div className="num">{formatInt(theme.documents)}</div><div className="muted">{t("dokumen")}</div></div>
        <div><div className="num">{formatDecimal(theme.centrality, 2)}</div><div className="muted">{t("sentralitas")}</div></div>
        <div><div className="num">{formatDecimal(theme.density, 2)}</div><div className="muted">{t("densitas")}</div></div>
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
          <strong>{t("Sinyal celah")}</strong>
          <span>{text}</span>
        </div>
      ) : null}
    </div>
  );
}
