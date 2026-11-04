import { t } from "../../i18n";
import { formatInt } from "../../lib/format";
import type { Prisma } from "../../lib/screeningTypes";
import { prismaBoxes, type Box } from "./prismaBoxes";

// Drawn in fixed coordinates and scaled to the panel; the PNG/SVG export keeps the viewBox.
const W = 900;
const LEFT = 52;
const BOX_W = 390;
const RIGHT = LEFT + BOX_W + 56;
const LINE = 17;
const PAD = 12;
const GAP = 34;
// Tall enough for the rotated phase label beside a one-line box.
const MIN_H = 84;

const height = (b: Box | null) => (b ? PAD * 2 + b.lines.length * LINE : 0);

function BoxShape({ box, x, y, h }: { box: Box; x: number; y: number; h: number }) {
  return (
    <g className="prisma-box" data-kind={box.kind}>
      <rect x={x} y={y} width={BOX_W} height={h} rx={4} />
      {box.lines.map((line, i) => (
        <text key={i} x={x + PAD} y={y + PAD + LINE * i + 12} className={i === 0 ? "prisma-head" : "prisma-line"}>{line}</text>
      ))}
    </g>
  );
}

/** PRISMA 2020 flow diagram for one database search: identification, screening, inclusion. */
export function PrismaDiagram({ data }: { data: Prisma }) {
  const rows = prismaBoxes(data, (n) => formatInt(n));
  let y = 8;
  const placed = rows.map((row) => {
    // Paired boxes share one height, as in the PRISMA template.
    const h = Math.max(height(row.main), height(row.side), row.phase === 2 ? MIN_H : 0);
    const at = { ...row, y, h };
    y += h + GAP;
    return at;
  });
  const total = y - GAP + 8;
  const phases = [t("Identifikasi"), t("Skrining"), t("Disertakan")];

  return (
    <svg className="prisma" viewBox={`0 0 ${W} ${total}`} role="img" aria-label={t("Diagram alir PRISMA 2020")}>
      <defs>
        <marker id="prisma-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
          <path d="M0 0 L10 5 L0 10 z" className="prisma-arrowhead" />
        </marker>
      </defs>
      {phases.map((label, i) => {
        const group = placed.filter((r) => r.phase === i);
        if (!group.length) return null;
        const top = group[0].y;
        const bottom = group[group.length - 1].y + group[group.length - 1].h;
        return (
          <g key={label} className="prisma-phase">
            <rect x={0} y={top} width={30} height={bottom - top} rx={4} />
            <text transform={`translate(19 ${(top + bottom) / 2}) rotate(-90)`} textAnchor="middle">{label}</text>
          </g>
        );
      })}
      {placed.map((row, i) => {
        const next = placed[i + 1];
        return (
          <g key={i}>
            <BoxShape box={row.main} x={LEFT} y={row.y} h={row.h} />
            {row.side ? <BoxShape box={row.side} x={RIGHT} y={row.y} h={row.h} /> : null}
            {row.side ? <line className="prisma-link" x1={LEFT + BOX_W} y1={row.y + row.h / 2} x2={RIGHT - 2} y2={row.y + row.h / 2} markerEnd="url(#prisma-arrow)" /> : null}
            {next ? <line className="prisma-link" x1={LEFT + BOX_W / 2} y1={row.y + row.h} x2={LEFT + BOX_W / 2} y2={next.y - 2} markerEnd="url(#prisma-arrow)" /> : null}
          </g>
        );
      })}
    </svg>
  );
}
