import { t } from "../../i18n";
import type { SavedIdea } from "../../lib/apiEntities";

interface Props {
  ideas: SavedIdea[];
  onOpen: (key: string) => void;
  onRemove: (key: string) => void;
}

/** Ideas already in Gap Explorer, with links to them. */
export function SavedIdeas({ ideas, onOpen, onRemove }: Props) {
  if (ideas.length === 0) return null;
  return (
    <section className="pnl">
      <div className="ph">{t("Ide tersimpan")}<span className="ph-aside">{t("tampil di Gap Explorer sebagai Ide peneliti")}</span></div>
      <ul className="saved-ideas">
        {ideas.map((i) => (
          <li key={i.key}>
            <span><strong>{i.a}</strong><span className="muted">{` ${t("dengan")} `}</span><strong>{i.b}</strong></span>
            <span className="saved-actions">
              <button type="button" className="btn" onClick={() => onOpen(i.key)}>{t("Buka di Explorer")}</button>
              <button type="button" className="btn" onClick={() => onRemove(i.key)}>{t("Hapus")}</button>
            </span>
          </li>
        ))}
      </ul>
    </section>
  );
}
