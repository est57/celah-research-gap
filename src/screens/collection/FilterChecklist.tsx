import { useMemo, useState } from "react";
import { formatInt } from "../../lib/format";
import type { Choice } from "../../lib/filterTypes";

interface Props {
  id: string;
  title: string;
  choices: Choice[];
  selected: string[];
  label: (value: string) => string;
  searchable?: boolean;
  onChange: (next: string[]) => void;
}

/** Checkbox list where nothing ticked means "all". */
export function FilterChecklist({ id, title, choices, selected, label, searchable, onChange }: Props) {
  const [query, setQuery] = useState("");
  const shown = useMemo(() => {
    const q = query.trim().toLowerCase();
    return q ? choices.filter((c) => label(c.value).toLowerCase().includes(q)) : choices;
  }, [choices, query, label]);
  const toggle = (v: string) => onChange(selected.includes(v) ? selected.filter((s) => s !== v) : [...selected, v]);

  return (
    <section className="pnl filter-group">
      <div className="ph">
        {title}
        <span className="ph-aside">{selected.length ? `${selected.length} dipilih` : "semua"}</span>
      </div>
      {searchable ? (
        <div className="filter-search">
          <label htmlFor={`${id}-q`} className="sr-only">{`Cari ${title.toLowerCase()}`}</label>
          <input id={`${id}-q`} className="input" type="search" maxLength={100} placeholder="Cari nama sumber"
            value={query} onChange={(e) => setQuery(e.target.value)} />
          {selected.length ? <button type="button" className="btn" onClick={() => onChange([])}>Kosongkan</button> : null}
        </div>
      ) : null}
      <ul className={searchable ? "filter-list filter-list-long" : "filter-list"}>
        {shown.map((c) => (
          <li key={c.value}>
            <label>
              <input type="checkbox" checked={selected.includes(c.value)} onChange={() => toggle(c.value)} />
              <span className={c.value ? "filter-name" : "filter-name muted"}>{label(c.value)}</span>
              <span className="num muted">{formatInt(c.count)}</span>
            </label>
          </li>
        ))}
        {shown.length === 0 ? <li className="muted filter-none">Tidak ada yang cocok.</li> : null}
      </ul>
    </section>
  );
}
