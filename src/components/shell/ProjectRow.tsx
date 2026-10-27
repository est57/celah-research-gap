import { useState } from "react";
import type { ProjectInfo } from "../../lib/apiProjects";
import { formatInt } from "../../lib/format";

interface Props {
  p: ProjectInfo;
  busy: boolean;
  onOpen: () => void;
  onRename: (name: string) => Promise<boolean>;
  onDelete: () => void;
}

/** One project in the switcher: open it, rename it, or delete it after confirming. */
export function ProjectRow({ p, busy, onOpen, onRename, onDelete }: Props) {
  const [mode, setMode] = useState<"view" | "rename" | "delete">("view");
  const [name, setName] = useState(p.name);
  const deletable = !p.current && p.id !== "utama";

  if (mode === "rename") {
    return (
      <li className="proj-row">
        <form className="proj-edit" onSubmit={(e) => { e.preventDefault(); void onRename(name).then((ok) => ok && setMode("view")); }}>
          <label htmlFor={`proj-${p.id}`} className="sr-only">Nama proyek</label>
          <input id={`proj-${p.id}`} className="input" maxLength={60} value={name} autoFocus onChange={(e) => setName(e.target.value)} />
          <button type="submit" className="btn" disabled={busy || !name.trim()}>Simpan</button>
          <button type="button" className="btn" onClick={() => { setName(p.name); setMode("view"); }}>Batal</button>
        </form>
      </li>
    );
  }
  return (
    <li className="proj-row" data-current={p.current}>
      <div className="proj-info">
        <span className="proj-name">{p.name}</span>
        <span className="muted num">{`${formatInt(p.documents)} dokumen${p.current ? " · sedang dibuka" : ""}`}</span>
      </div>
      {mode === "delete" ? (
        <div className="proj-actions">
          <span className="delete-warn">Hapus beserta isinya?</span>
          <button type="button" className="btn" onClick={() => setMode("view")}>Batal</button>
          <button type="button" className="btn btn-danger" disabled={busy} onClick={onDelete}>Ya, hapus</button>
        </div>
      ) : (
        <div className="proj-actions">
          {!p.current ? <button type="button" className="btn btn-primary" disabled={busy} onClick={onOpen}>Buka</button> : null}
          <button type="button" className="btn" onClick={() => setMode("rename")}>Ganti nama</button>
          {deletable ? <button type="button" className="btn" onClick={() => setMode("delete")}>Hapus</button> : null}
        </div>
      )}
    </li>
  );
}
