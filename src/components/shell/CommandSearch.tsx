import { Fragment, useEffect, useRef, useState, type KeyboardEvent } from "react";
import { useQuickSearch } from "../../hooks/useQuickSearch";
import type { Jump } from "../../lib/commands";
import { t } from "../../i18n";

const MAC = typeof navigator !== "undefined" && /mac/i.test(navigator.userAgent);

/** Title-bar search: screens, documents, authors and sources. Ctrl/Cmd+K focuses it. */
export function CommandSearch({ onJump }: { onJump: (j: Jump) => void }) {
  const [query, setQuery] = useState("");
  const [open, setOpen] = useState(false);
  const [active, setActive] = useState(0);
  const input = useRef<HTMLInputElement>(null);
  const items = useQuickSearch(query);
  const shown = open && query.trim().length > 0;

  useEffect(() => {
    const onKey = (e: globalThis.KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        input.current?.focus();
        input.current?.select();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);
  useEffect(() => setActive(0), [query, items.length]);

  const choose = (i: number) => {
    const item = items[i];
    if (!item) return;
    onJump(item.jump);
    setQuery("");
    setOpen(false);
    input.current?.blur();
  };
  const onKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "ArrowDown" || e.key === "ArrowUp") {
      e.preventDefault();
      const step = e.key === "ArrowDown" ? 1 : -1;
      setActive((a) => (items.length ? (a + step + items.length) % items.length : 0));
    } else if (e.key === "Enter") {
      e.preventDefault();
      choose(active);
    } else if (e.key === "Escape") {
      setOpen(false);
      input.current?.blur();
    }
  };

  return (
    <div className="cmd-wrap">
      <label className="cmd" htmlFor="cmd-input">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
          <circle cx="11" cy="11" r="7" />
          <path d="M20 20l-3.5-3.5" />
        </svg>
        <span className="sr-only">{t("Cari layar, dokumen, penulis, atau sumber")}</span>
        <input id="cmd-input" ref={input} type="search" role="combobox" aria-expanded={shown} aria-controls="cmd-list"
          aria-activedescendant={shown && items[active] ? `cmd-${active}` : undefined} autoComplete="off"
          placeholder={t("Cari perintah, dokumen, penulis")} maxLength={100} value={query}
          onChange={(e) => { setQuery(e.target.value); setOpen(true); }} onFocus={() => setOpen(true)}
          onBlur={() => setOpen(false)} onKeyDown={onKeyDown} />
        <kbd className="cmd-key">{MAC ? "Cmd K" : "Ctrl K"}</kbd>
      </label>
      {shown ? (
        <ul id="cmd-list" className="cmd-list" role="listbox" aria-label={t("Hasil pencarian")}>
          {items.length === 0 ? <li className="cmd-empty muted">{query.trim().length < 2 ? t("Ketik minimal 2 huruf") : t("Tidak ada yang cocok")}</li> : null}
          {items.map((it, i) => (
            <Fragment key={it.key}>
              {i === 0 || items[i - 1].group !== it.group ? <li role="presentation" className="cmd-group">{it.group}</li> : null}
              <li id={`cmd-${i}`} role="option" aria-selected={i === active} className="cmd-item"
                onMouseDown={(e) => e.preventDefault()} onMouseEnter={() => setActive(i)} onClick={() => choose(i)}>
                <span className="cmd-label">{it.label}</span>
                <span className="cmd-sub muted">{it.sub}</span>
              </li>
            </Fragment>
          ))}
        </ul>
      ) : null}
    </div>
  );
}
