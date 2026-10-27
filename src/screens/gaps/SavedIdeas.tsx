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
      <div className="ph">Ide tersimpan<span className="ph-aside">tampil di Gap Explorer sebagai Ide peneliti</span></div>
      <ul className="saved-ideas">
        {ideas.map((i) => (
          <li key={i.key}>
            <span><strong>{i.a}</strong><span className="muted"> dengan </span><strong>{i.b}</strong></span>
            <span className="saved-actions">
              <button type="button" className="btn" onClick={() => onOpen(i.key)}>Buka di Explorer</button>
              <button type="button" className="btn" onClick={() => onRemove(i.key)}>Hapus</button>
            </span>
          </li>
        ))}
      </ul>
    </section>
  );
}
