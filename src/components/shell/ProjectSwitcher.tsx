import { useEffect, useRef, useState, type FormEvent } from "react";
import { useProjects } from "../../hooks/useProjects";
import { isDesktop } from "../../lib/api";
import { ProjectRow } from "./ProjectRow";

/** Title-bar project menu: each project has its own collection, reviews, synonyms and ideas. */
export function ProjectSwitcher({ onSwitched }: { onSwitched: () => void }) {
  const [open, setOpen] = useState(false);
  const [name, setName] = useState("");
  const box = useRef<HTMLDivElement>(null);
  const { projects, current, busy, error, refresh, create, rename, remove, open: openProject } = useProjects(() => {
    setOpen(false);
    onSwitched();
  });

  useEffect(() => {
    if (!open) return;
    void refresh();
    const onDown = (e: MouseEvent) => { if (box.current && !box.current.contains(e.target as Node)) setOpen(false); };
    const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") setOpen(false); };
    document.addEventListener("mousedown", onDown);
    document.addEventListener("keydown", onKey);
    return () => {
      document.removeEventListener("mousedown", onDown);
      document.removeEventListener("keydown", onKey);
    };
    // Refresh counts each time the menu opens.
  }, [open]);

  const submit = (e: FormEvent) => {
    e.preventDefault();
    if (name.trim()) void create(name).then((ok) => ok && setName(""));
  };

  return (
    <div className="proj" ref={box}>
      <button type="button" className="proj-button" aria-haspopup="dialog" aria-expanded={open} onClick={() => setOpen(!open)} title="Ganti proyek">
        <span className="proj-current">{current?.name ?? "Proyek"}</span>
        <svg width="10" height="10" viewBox="0 0 10 10" aria-hidden="true"><path d="M2 3.5l3 3 3-3" fill="none" stroke="currentColor" strokeWidth="1.4" /></svg>
      </button>
      {open ? (
        <div className="proj-menu" role="dialog" aria-label="Proyek">
          <p className="muted proj-intro">Tiap proyek punya koleksi, review, sinonim, dan ide sendiri. Kunci API dipakai bersama.</p>
          {!isDesktop() ? <p className="muted proj-intro">Proyek hanya bisa dikelola di aplikasi desktop.</p> : null}
          <ul className="proj-list">
            {projects.map((p) => (
              <ProjectRow key={p.id} p={p} busy={busy} onOpen={() => void openProject(p.id)}
                onRename={(n) => rename(p.id, n)} onDelete={() => void remove(p.id)} />
            ))}
          </ul>
          <form className="proj-new" onSubmit={submit}>
            <label htmlFor="proj-new" className="sr-only">Nama proyek baru</label>
            <input id="proj-new" className="input" maxLength={60} placeholder="Nama proyek baru" value={name} onChange={(e) => setName(e.target.value)} />
            <button type="submit" className="btn" disabled={busy || !name.trim() || !isDesktop()}>Buat</button>
          </form>
          {error ? <p className="delete-error proj-intro" role="alert">{error}</p> : null}
        </div>
      ) : null}
    </div>
  );
}
