import type { Jump } from "../../lib/commands";
import { formatDecimal, formatInt } from "../../lib/format";
import { t } from "../../i18n";
import type { Opportunity } from "../../lib/opportunity";
import { themeColor } from "../conceptual/ThematicMap";
import { opStatusLabel } from "./labels";

interface Props {
  item: Opportunity;
  rank: number;
  selected: boolean;
  recentSpan: string;
  onSelect: () => void;
  onJump: (j: Jump) => void;
}

/** One theme worth considering: status, why, and where to go next. */
export function OpportunityCard({ item, rank, selected, recentSpan, onSelect, onJump }: Props) {
  const th = item.theme;
  // The theme name joins several labels with dots; searches use its main keyword only.
  const term = th.words[0]?.label ?? th.name;
  const why = item.status === "rising"
    ? t("{pct}% dokumennya terbit {span}, {pace} kali laju koleksi, dan masih jarang terhubung dengan tema utama.", {
      pct: formatInt(Math.round(th.recentShare * 100)), span: recentSpan, pace: formatDecimal(item.pace),
    })
    : item.status === "declining"
      ? t("Hanya {pct}% dokumennya terbit {span}. Bisa jadi tema yang ditinggalkan; periksa alasannya sebelum memilih.", {
        pct: formatInt(Math.round(th.recentShare * 100)), span: recentSpan,
      })
      : t("Tema yang sudah berkembang di dalam kelompoknya sendiri, tetapi jarang bersinggungan dengan tema lain.");
  return (
    <li className="op-card pnl" data-selected={selected}>
      <button type="button" className="op-head" aria-pressed={selected} onClick={onSelect}>
        <span className="num op-rank">{rank}</span>
        <span className="theme-swatch" style={{ background: themeColor(th.id) }} />
        <strong className="op-name">{th.name}</strong>
        <span className="op-status" data-status={item.status}>{opStatusLabel(item.status)}</span>
        <span className="num muted op-score">{t("skor {n}", { n: formatDecimal(item.score, 2) })}</span>
      </button>
      <p className="op-why">{why}</p>
      <p className="muted op-words">{th.words.slice(0, 6).map((w) => w.label).join(" · ")}</p>
      <div className="op-facts num muted">
        {t("{n} dokumen · sentralitas {c} · densitas {d}", { n: formatInt(th.documents), c: formatDecimal(th.centrality, 2), d: formatDecimal(th.density, 2) })}
      </div>
      <div className="op-actions">
        <button type="button" className="btn" onClick={() => onJump({ screen: "gaps", tab: "explorer" })}>{t("Kandidat celah")}</button>
        <button type="button" className="btn" onClick={() => onJump({ screen: "gaps", tab: "idea", query: term })}>{t("Uji dengan ide lain")}</button>
        <button type="button" className="btn" onClick={() => onJump({ screen: "collection", tab: "collection", query: term })}>{t("Lihat dokumen")}</button>
      </div>
    </li>
  );
}
