import { NAV, type ScreenId } from "../../nav";
import { formatInt } from "../../lib/format";
import { t } from "../../i18n";

interface Props {
  active: ScreenId;
  total: number;
  open: boolean;
  onSelect: (id: ScreenId) => void;
  onClose: () => void;
}

export function NavSidebar({ active, total, open, onSelect, onClose }: Props) {
  return (
    <>
      <button
        type="button"
        className="nav-backdrop"
        data-open={open}
        aria-label={t("Tutup menu")}
        tabIndex={-1}
        onClick={onClose}
      />
      <nav id="nav" className="nav" data-open={open} aria-label={t("Navigasi analisis")}>
        {NAV.map((section) => (
          <div key={section.title}>
            <h2>{t(section.title)}</h2>
            {section.items.map((item) => (
              <button
                key={item.id}
                type="button"
                className="nav-item"
                style={{ width: "100%" }}
                aria-current={item.id === active ? "page" : undefined}
                onClick={() => onSelect(item.id)}
              >
                {t(item.label)}
                {item.showsTotal ? <span className="count">{formatInt(total)}</span> : null}
              </button>
            ))}
          </div>
        ))}
      </nav>
    </>
  );
}
