import type { Jump } from "../../lib/commands";
import { formatDecimal, formatInt } from "../../lib/format";
import { STATUS_LABEL, type Opportunity } from "../../lib/opportunity";
import { themeColor } from "../conceptual/ThematicMap";

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
  const t = item.theme;
  // The theme name joins several labels with dots; searches use its main keyword only.
  const term = t.words[0]?.label ?? t.name;
  const why = item.status === "rising"
    ? `${formatInt(Math.round(t.recentShare * 100))}% dokumennya terbit ${recentSpan}, ${formatDecimal(item.pace)} kali laju koleksi, dan masih jarang terhubung dengan tema utama.`
    : item.status === "declining"
      ? `Hanya ${formatInt(Math.round(t.recentShare * 100))}% dokumennya terbit ${recentSpan}. Bisa jadi tema yang ditinggalkan; periksa alasannya sebelum memilih.`
      : "Tema yang sudah berkembang di dalam kelompoknya sendiri, tetapi jarang bersinggungan dengan tema lain.";
  return (
    <li className="op-card pnl" data-selected={selected}>
      <button type="button" className="op-head" aria-pressed={selected} onClick={onSelect}>
        <span className="num op-rank">{rank}</span>
        <span className="theme-swatch" style={{ background: themeColor(t.id) }} />
        <strong className="op-name">{t.name}</strong>
        <span className="op-status" data-status={item.status}>{STATUS_LABEL[item.status]}</span>
        <span className="num muted op-score">{`skor ${formatDecimal(item.score, 2)}`}</span>
      </button>
      <p className="op-why">{why}</p>
      <p className="muted op-words">{t.words.slice(0, 6).map((w) => w.label).join(" · ")}</p>
      <div className="op-facts num muted">
        {`${formatInt(t.documents)} dokumen · sentralitas ${formatDecimal(t.centrality, 2)} · densitas ${formatDecimal(t.density, 2)}`}
      </div>
      <div className="op-actions">
        <button type="button" className="btn" onClick={() => onJump({ screen: "gaps", tab: "explorer" })}>Kandidat celah</button>
        <button type="button" className="btn" onClick={() => onJump({ screen: "gaps", tab: "idea", query: term })}>Uji dengan ide lain</button>
        <button type="button" className="btn" onClick={() => onJump({ screen: "collection", tab: "collection", query: term })}>Lihat dokumen</button>
      </div>
    </li>
  );
}
