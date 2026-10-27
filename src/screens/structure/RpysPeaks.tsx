import { useEffect } from "react";
import { formatInt } from "../../lib/format";
import type { RpysPeak } from "../../lib/rpysTypes";

interface Props {
  peaks: RpysPeak[];
  selected: number | null;
  onSelect: (year: number) => void;
}

/** Peak years in time order, each with the works cited most from that year. */
export function RpysPeaks({ peaks, selected, onSelect }: Props) {
  const ordered = [...peaks].sort((a, b) => a.year - b.year);
  useEffect(() => {
    document.querySelector('.rpys-peaks > li[data-selected="true"]')?.scrollIntoView({ block: "nearest" });
  }, [selected]);
  return (
    <ol className="rpys-peaks">
      {ordered.map((p) => (
        <li key={p.year} data-selected={p.year === selected}>
          <button type="button" className="rpys-peak-head" aria-pressed={p.year === selected} onClick={() => onSelect(p.year)}>
            <span className="num rpys-year">{p.year}</span>
            <span className="muted">{`${formatInt(p.citations)} sitasi · +${formatInt(p.deviation)} di atas median`}</span>
          </button>
          <ul className="rpys-refs">
            {p.refs.map((r, i) => (
              <li key={i}>
                <span className="rpys-ref-label">{r.label}</span>
                <span className="num muted">{`${formatInt(r.documents)} dok.`}</span>
                <span className="rpys-ref-example select">{r.example}</span>
              </li>
            ))}
          </ul>
        </li>
      ))}
    </ol>
  );
}
