import { useEffect, useRef } from "react";
import { t } from "../../i18n";

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

// Labels may be keys kept with tk() or texts already passed through t(); t() on either is safe.
export function Tabs<T extends string>({ label, tabs, active, onChange }: Props<T>) {
  const bar = useRef<HTMLDivElement>(null);
  // Keep the active tab visible when the row is wider than the window.
  useEffect(() => {
    bar.current?.querySelector('[aria-selected="true"]')?.scrollIntoView({ block: "nearest", inline: "nearest" });
  }, [active]);
  return (
    <div ref={bar} className="tabs" role="tablist" aria-label={t(label)}>
      {tabs.map((tab) => (
        <button
          key={tab.id}
          type="button"
          role="tab"
          className="tab"
          aria-selected={tab.id === active}
          disabled={tab.disabled}
          title={tab.disabled ? t("Tersedia di versi berikutnya") : undefined}
          onClick={() => onChange(tab.id)}
        >
          {t(tab.label)}
        </button>
      ))}
    </div>
  );
}
