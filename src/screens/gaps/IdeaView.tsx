import { useState, type FormEvent } from "react";
import { isDesktop } from "../../lib/api";
import { useIdea } from "../../hooks/useIdea";
import { CheckResult } from "./CheckResult";
import { IdeaResult } from "./IdeaResult";
import { SavedIdeas } from "./SavedIdeas";
import { useSavedIdeas } from "../../hooks/useSavedIdeas";

/** A researcher's own gap idea: two concepts tested against the collection and recent literature. */
interface Props {
  /** Opens a gap in Gap Explorer. */
  onOpen: (key: string) => void;
  /** Gap candidates changed (an idea was saved or removed). */
  onChanged: () => void;
}

export function IdeaView({ onOpen, onChanged }: Props) {
  const [a, setA] = useState("");
  const [b, setB] = useState("");
  const [saved, setSaved] = useState(false);
  const { idea, check, busy, error, test, verify } = useIdea();
  const store = useSavedIdeas(onChanged);
  const submit = (e: FormEvent) => {
    e.preventDefault();
    setSaved(false);
    if (a.trim() && b.trim()) void test(a, b);
  };
  const now = new Date().toISOString().slice(0, 19).replace("T", " ");

  return (
    <div className="year-page">
      <section className="pnl">
        <div className="ph">Uji ide celah</div>
        <form className="idea-form" onSubmit={submit}>
          <p className="muted">
            Tulis dua konsep yang ingin Anda gabungkan. Pisahkan sebutan lain dengan koma, misalnya
            "vision transformer, ViT". Filter analisis dan sinonim kata kunci ikut berlaku.
          </p>
          <div className="idea-inputs">
            <label htmlFor="idea-a">Konsep A</label>
            <input id="idea-a" className="input" maxLength={300} placeholder="vision transformer, ViT" value={a} onChange={(e) => setA(e.target.value)} />
            <label htmlFor="idea-b">Konsep B</label>
            <input id="idea-b" className="input" maxLength={300} placeholder="sputum smear" value={b} onChange={(e) => setB(e.target.value)} />
            <button type="submit" className="btn btn-primary" disabled={busy !== null || !a.trim() || !b.trim()}>{busy === "test" ? "Menghitung..." : "Uji"}</button>
          </div>
        </form>
        {error ? <p className="delete-error ent-note" role="alert">{error}</p> : null}
      </section>
      {idea ? <IdeaResult idea={idea} /> : null}
      {idea && idea.verdict !== "kurang-data" ? (
        <div className="idea-save">
          <span className="muted">Simpan agar ide ini tampil di Gap Explorer, bisa direview, diverifikasi, dan masuk ke laporan.</span>
          <button type="button" className="btn btn-primary" disabled={saved || !isDesktop()}
            onClick={() => void store.save(a, b).then((ok) => setSaved(ok))}>
            {saved ? "Tersimpan di Gap Explorer" : "Simpan ke Gap Explorer"}
          </button>
        </div>
      ) : null}
      {store.error ? <div className="notice" role="alert">{store.error}</div> : null}
      {idea ? (
        <section className="pnl">
          <div className="ph">
            Literatur terbaru di luar koleksi
            <span className="ph-aside">OpenAlex, 2 tahun terakhir</span>
          </div>
          <div className="pnl-body">
            {check ? (
              <CheckResult check={{ ...check, checkedAt: now }} limit={8} />
            ) : (
              <div className="idea-check">
                <span className="muted">Pastikan celah ini belum dijawab oleh paper yang belum ada di koleksi Anda.</span>
                <button type="button" className="btn" disabled={busy !== null || !isDesktop()} onClick={() => void verify(a, b)}>
                  {busy === "check" ? "Mencari..." : "Cek literatur terbaru"}
                </button>
              </div>
            )}
          </div>
        </section>
      ) : null}
      <SavedIdeas ideas={store.ideas} onOpen={onOpen} onRemove={(k) => void store.remove(k)} />
    </div>
  );
}
