import { useEffect, useRef } from "react";

export interface TabDef<T extends string> {
  id: T;
  label: string;
  disabled?: boolean;
}

interface Props<T extends string> {
  label: string;
  tabs: TabDef<T>[];
  active: T;
  onChange: (id: T) => void;
}

export function Tabs<T extends string>({ label, tabs, active, onChange }: Props<T>) {
  const bar = useRef<HTMLDivElement>(null);
  // Keep the active tab visible when the row is wider than the window.
  useEffect(() => {
    bar.current?.querySelector('[aria-selected="true"]')?.scrollIntoView({ block: "nearest", inline: "nearest" });
  }, [active]);
  return (
    <div ref={bar} className="tabs" role="tablist" aria-label={label}>
      {tabs.map((t) => (
        <button
          key={t.id}
          type="button"
          role="tab"
          className="tab"
          aria-selected={t.id === active}
          disabled={t.disabled}
          title={t.disabled ? "Tersedia di versi berikutnya" : undefined}
          onClick={() => onChange(t.id)}
        >
          {t.label}
        </button>
      ))}
    </div>
  );
}
