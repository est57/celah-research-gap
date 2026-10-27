import { ScreenMessage } from "../../components/ScreenMessage";
import { useCitedEntities } from "../../hooks/useCitedEntities";
import { formatInt } from "../../lib/format";
import { TopSources } from "../sources/TopSources";

/** Sources or first authors most cited by the collection's references. */
export function CitedEntitiesPanel({ version, kind }: { version: number; kind: "sources" | "authors" }) {
  const { data, sample, error, loading } = useCitedEntities(version);
  if (error) return <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  if (!data) return <ScreenMessage>{loading ? "Menghitung sumber dan penulis yang dirujuk..." : ""}</ScreenMessage>;
  const list = kind === "sources" ? data.sources : data.authors;
  return (
    <>
    {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
    <section className="pnl">
      <div className="ph">
        {kind === "sources" ? "Sumber paling banyak dirujuk" : "Penulis pertama paling banyak dirujuk"}
        <span className="ph-aside">{`dari ${formatInt(data.references)} rujukan · angka = dokumen koleksi yang merujuk`}</span>
      </div>
      {list.length ? (
        <div className="pnl-body words-bars">
          <TopSources items={list.map((c) => ({ label: c.label, count: c.documents }))} plain={kind === "authors"} wide />
        </div>
      ) : (
        <p className="muted rpys-empty">Belum ada rujukan dengan format yang bisa dibaca (ekspor Scopus, WoS, atau Crossref).</p>
      )}
      <p className="muted ent-note">
        Sumber dibaca dari teks rujukan, jadi singkatan seperti "J DIGIT IMAGING" dan nama lengkap jurnal dihitung terpisah.
        Penulis lembaga, misalnya World Health Organization, tampil dengan kata pertamanya.
      </p>
    </section>
    </>
  );
}
