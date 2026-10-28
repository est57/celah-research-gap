import { formatInt } from "../../lib/format";
import type { Jump } from "../../lib/commands";
import { CommandSearch } from "./CommandSearch";
import { ProjectSwitcher } from "./ProjectSwitcher";
import { t } from "../../i18n";

interface Props {
  onToggleNav: () => void;
  navOpen: boolean;
  onRefresh: () => void;
  busy: boolean;
  onExport: () => void;
  /** Shown while a collection filter is active. */
  filter: { analyzed: number; total: number } | null;
  onFilter: () => void;
  onJump: (j: Jump) => void;
  /** Another project was opened; every screen reloads. */
  onProjectSwitched: () => void;
}

function Logo() {
  return (
    <svg width="18" height="18" viewBox="0 0 28 28" aria-hidden="true">
      <path d="M12 4a10 10 0 0 0 0 20" fill="none" stroke="#2456A6" strokeWidth="3.5" strokeLinecap="round" />
      <path d="M16 4a10 10 0 0 1 0 20" fill="none" stroke="#D9622B" strokeWidth="3.5" strokeLinecap="round" />
    </svg>
  );
}

export function TitleBar({ onToggleNav, navOpen, onRefresh, busy, onExport, filter, onFilter, onJump, onProjectSwitched }: Props) {
  return (
    <header className="titlebar">
      <button
        type="button"
        className="menu-btn"
        aria-label={t("Buka menu analisis")}
        aria-expanded={navOpen}
        aria-controls="nav"
        onClick={onToggleNav}
      >
        <svg width="14" height="14" viewBox="0 0 14 14" aria-hidden="true">
          <path d="M1 3h12M1 7h12M1 11h12" stroke="currentColor" strokeWidth="1.5" />
        </svg>
      </button>
      <div className="brand">
        <Logo />
        Celah
      </div>
      <span className="sep project" />
      <ProjectSwitcher onSwitched={onProjectSwitched} />
      <CommandSearch onJump={onJump} />
      {filter ? (
        <button type="button" className="btn filter-chip" onClick={onFilter} title={t("Analisis memakai koleksi yang difilter. Klik untuk mengubah.")}>
          {t("Filter aktif")}
          <span className="num">{`${formatInt(filter.analyzed)}/${formatInt(filter.total)}`}</span>
        </button>
      ) : null}
      <button type="button" className="btn" onClick={onRefresh} disabled={busy}>
        {t("Perbarui data")}
      </button>
      <button type="button" className="btn btn-primary" onClick={onExport}>
        {t("Ekspor laporan")}
      </button>
    </header>
  );
}
