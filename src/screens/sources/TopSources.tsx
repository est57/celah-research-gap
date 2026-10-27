import { formatInt } from "../../lib/format";
import type { NamedCount } from "../../lib/sourcesTypes";

/** Horizontal bar list. Source titles are italic by convention; pass `plain` for other names. */
export function TopSources({ items, plain }: { items: NamedCount[]; plain?: boolean }) {
  const max = Math.max(...items.map((s) => s.count), 1);
  return (
    <ol className="bar-list">
      {items.map((s) => (
        <li key={s.label}>
          <span className="bar-label" data-plain={plain} title={s.label}>{s.label}</span>
          <span className="bar-track">
            <span className="bar-fill" style={{ width: `${(s.count / max) * 100}%` }} />
          </span>
          <span className="num bar-value">{formatInt(s.count)}</span>
        </li>
      ))}
    </ol>
  );
}
