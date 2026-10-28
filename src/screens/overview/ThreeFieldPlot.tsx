import { useMemo } from "react";
import { useWidth } from "../../hooks/useWidth";
import { formatInt } from "../../lib/format";
import { layoutSankey, SANKEY_NODE_W } from "../../lib/sankey";
import type { ThreeField } from "../../lib/overviewTypes";
import { countryName } from "../../lib/countryNames";
import { t } from "../../i18n";

const PALETTE = ["#2456A6", "#2E8B6E", "#7A5BB0", "#B8457A", "#8C7A3E", "#3B8FB5", "#D9A21B", "#6B7480"];

function shorten(text: string, max: number): string {
  return text.length > max ? `${text.slice(0, max - 1)}...` : text;
}

export function ThreeFieldPlot({ data, height = 300 }: { data: ThreeField; height?: number }) {
  const { ref, width } = useWidth<HTMLDivElement>(800);
  const labelW = Math.min(170, Math.max(90, width * 0.2));
  const maxChars = Math.floor(labelW / 6.5);
  const layout = useMemo(() => layoutSankey(data, width, height, labelW), [data, width, height, labelW]);
  const name = (column: number, label: string) => (data.kinds[column] === "countries" ? countryName(label) : label);

  if (layout.nodes.length === 0) {
    return (
      <div ref={ref} className="chart-empty">
        {t("Belum cukup data {fields} yang saling terhubung untuk plot ini.", { fields: data.labels.map((l) => t(l).toLowerCase()).join(", ") })}
      </div>
    );
  }

  return (
    <div ref={ref} className="three-field">
      <div className="three-field-heads muted">
        {data.labels.map((l) => <span key={l}>{t(l)}</span>)}
      </div>
      <svg width={width} height={height} role="img" aria-label={t("Aliran dokumen: {fields}", { fields: data.labels.map((l) => t(l)).join(", ") })}>
        {layout.ribbons.map((r, i) => (
          <path key={i} d={r.d} style={{ fill: r.stage === 0 ? PALETTE[r.source % PALETTE.length] : "var(--neutral-bar)" }}
            fillOpacity={r.stage === 0 ? 0.35 : 0.45}>
            <title>{t("{n} dokumen", { n: formatInt(r.value) })}</title>
          </path>
        ))}
        {layout.nodes.map((n, i) => {
          const right = n.column === 0;
          const tx = right ? n.x - 6 : n.x + SANKEY_NODE_W + 5;
          return (
            <g key={i}>
              <rect x={n.x} y={n.y} width={SANKEY_NODE_W} height={Math.max(n.h, 1)} style={{ fill: "var(--ink)" }} />
              <text x={tx} y={n.y + n.h / 2} dy="0.35em" textAnchor={right ? "end" : "start"} className="sankey-label">
                <title>{`${name(n.column, n.label)}: ${formatInt(n.value)}`}</title>
                {shorten(name(n.column, n.label), maxChars)}
              </text>
            </g>
          );
        })}
      </svg>
    </div>
  );
}
