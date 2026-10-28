import { useState } from "react";
import type { ProjectInfo } from "../../lib/apiProjects";
import { formatInt } from "../../lib/format";
import { t } from "../../i18n";

interface Props {
  p: ProjectInfo;
  busy: boolean;
  onOpen: () => void;
  onRename(name: string): Promise<boolean>;
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
          <label htmlFor={`proj-${p.id}`} className="sr-only">{t("Nama proyek")}</label>
          <input id={`proj-${p.id}`} className="input" maxLength={60} value={name} autoFocus onChange={(e) => setName(e.target.value)} />
          <button type="submit" className="btn" disabled={busy || !name.trim()}>{t("Simpan")}</button>
          <button type="button" className="btn" onClick={() => { setName(p.name); setMode("view"); }}>{t("Batal")}</button>
        </form>
      </li>
    );
  }
  return (
    <li className="proj-row" data-current={p.current}>
      <div className="proj-info">
        <span className="proj-name">{p.name}</span>
        <span className="muted num">{`${t("{n} dokumen", { n: formatInt(p.documents) })}${p.current ? ` · ${t("sedang dibuka")}` : ""}`}</span>
      </div>
      {mode === "delete" ? (
        <div className="proj-actions">
          <span className="delete-warn">{t("Hapus beserta isinya?")}</span>
          <button type="button" className="btn" onClick={() => setMode("view")}>{t("Batal")}</button>
          <button type="button" className="btn btn-danger" disabled={busy} onClick={onDelete}>{t("Ya, hapus")}</button>
        </div>
      ) : (
        <div className="proj-actions">
          {!p.current ? <button type="button" className="btn btn-primary" disabled={busy} onClick={onOpen}>{t("Buka")}</button> : null}
          <button type="button" className="btn" onClick={() => setMode("rename")}>{t("Ganti nama")}</button>
          {deletable ? <button type="button" className="btn" onClick={() => setMode("delete")}>{t("Hapus")}</button> : null}
        </div>
      )}
    </li>
  );
}
