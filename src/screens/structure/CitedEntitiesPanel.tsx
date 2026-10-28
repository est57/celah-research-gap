import { NoReferences } from "../../components/NoReferences";
import { ScreenMessage } from "../../components/ScreenMessage";
import { useCitedEntities } from "../../hooks/useCitedEntities";
import { formatInt } from "../../lib/format";
import { TopSources } from "../sources/TopSources";
import { t } from "../../i18n";

/** Sources or first authors most cited by the collection's references. */
export function CitedEntitiesPanel({ version, kind, onDataChanged }: { version: number; kind: "sources" | "authors"; onDataChanged?: () => void }) {
  const { data, sample, error, loading } = useCitedEntities(version);
  if (error) return <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  if (!data) return <ScreenMessage>{loading ? t("Menghitung sumber dan penulis yang dirujuk...") : ""}</ScreenMessage>;
  if (data.references === 0) return <NoReferences analysis={kind === "sources" ? t("Sumber yang dirujuk") : t("Penulis yang dirujuk")} onDone={onDataChanged} />;
  const list = kind === "sources" ? data.sources : data.authors;
  return (
    <>
    {sample ? <div className="notice">{t("Pratinjau browser: angka di halaman ini adalah data contoh.")}</div> : null}
    <section className="pnl">
      <div className="ph">
        {kind === "sources" ? t("Sumber paling banyak dirujuk") : t("Penulis pertama paling banyak dirujuk")}
        <span className="ph-aside">{t("dari {n} rujukan · angka = dokumen koleksi yang merujuk", { n: formatInt(data.references) })}</span>
      </div>
      {list.length ? (
        <div className="pnl-body words-bars">
          <TopSources items={list.map((c) => ({ label: c.label, count: c.documents }))} plain={kind === "authors"} wide />
        </div>
      ) : (
        <p className="muted rpys-empty">{t("Belum ada rujukan dengan format yang bisa dibaca (ekspor Scopus, WoS, atau Crossref).")}</p>
      )}
      <p className="muted ent-note">
        {t("Sumber dibaca dari teks rujukan, jadi singkatan seperti \"J DIGIT IMAGING\" dan nama lengkap jurnal dihitung terpisah. Penulis lembaga, misalnya World Health Organization, tampil dengan kata pertamanya.")}
      </p>
    </section>
    </>
  );
}
