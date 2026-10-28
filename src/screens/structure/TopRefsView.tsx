import { useState, type ReactNode } from "react";
import { NoReferences } from "../../components/NoReferences";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useAddWorks } from "../../hooks/useAddWorks";
import { useTopRefs } from "../../hooks/useTopRefs";
import { formatInt } from "../../lib/format";
import { CitedEntitiesPanel } from "./CitedEntitiesPanel";
import { TopRefList } from "./TopRefList";
import { t } from "../../i18n";

const TOPS = [50, 100, 200, 500];

/** Most cited references of the collection, and which of them the collection lacks. */
export function TopRefsView({ version, onDataChanged }: { version: number; onDataChanged: () => void }) {
  const [top, setTop] = useState(100);
  const [show, setShow] = useState<"works" | "sources" | "authors">("works");
  const { data, sample, error, loading } = useTopRefs(version, top);
  const adder = useAddWorks(onDataChanged);
  const addable = data?.refs.filter((r) => r.inCollection === null && r.doi).map((r) => r.doi as string) ?? [];

  let main: ReactNode;
  if (show !== "works") main = <div className="conceptual-main"><CitedEntitiesPanel version={version} kind={show} onDataChanged={onDataChanged} /></div>;
  else if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? t("Menghitung rujukan teratas...") : ""}</ScreenMessage>;
  else if (data.documents === 0) main = <EmptyCollection />;
  else if (data.refs.length === 0) main = <NoReferences analysis={t("Rujukan teratas")} onDone={onDataChanged} />;
  else {
    main = (
      <div className="conceptual-main">
        {sample ? <div className="notice">{t("Pratinjau browser: angka di halaman ini adalah data contoh.")}</div> : null}
        <section className="pnl">
          <div className="ph">
            {t("Rujukan paling sering dikutip")}
            <span className="ph-aside">{t("{n} teratas · angka = dokumen koleksi yang merujuk", { n: formatInt(data.refs.length) })}</span>
          </div>
          <div className="tr-add">
            <button type="button" className="btn" disabled={adder.busy || addable.length === 0} onClick={() => void adder.add(addable)}>
              {adder.busy ? t("Menambahkan...") : t("Tambahkan semua yang belum ada ({n} ber-DOI)", { n: formatInt(addable.length) })}
            </button>
            {adder.message ? <span className={adder.message.error ? "tr-msg tr-msg-error" : "tr-msg"} role="status">{adder.message.text}</span> : null}
          </div>
          <TopRefList refs={data.refs} busy={adder.busy} onAdd={(d) => void adder.add(d)} />
        </section>
      </div>
    );
  }

  return (
    <div className="screen">
      <section className="screen-main">{main}</section>
      <aside className="inspector" aria-label={t("Parameter rujukan teratas")}>
        <div className="ph">{t("Parameter")}</div>
        <div className="params">
          <div className="param-row">
            <label htmlFor="tr-show">{t("Tampilkan")}</label>
            <select id="tr-show" className="input" value={show} onChange={(e) => setShow(e.target.value as typeof show)}>
              <option value="works">{t("Karya")}</option>
              <option value="sources">{t("Sumber yang dirujuk")}</option>
              <option value="authors">{t("Penulis yang dirujuk")}</option>
            </select>
          </div>
          <div className="param-row">
            <label htmlFor="tr-top">{t("Jumlah rujukan")}</label>
            <select id="tr-top" className="input" value={top} onChange={(e) => setTop(Number(e.target.value))}>
              {TOPS.map((n) => <option key={n} value={n}>{n}</option>)}
            </select>
          </div>
        </div>
        <div className="ph">{t("Ringkasan")}</div>
        {data ? (
          <dl className="words-facts">
            <dt>{t("Dokumen dengan rujukan teks")}</dt>
            <dd className="num">{t("{n} dari {total}", { n: formatInt(data.citing), total: formatInt(data.documents) })}</dd>
            <dt>{t("Karya berbeda yang dirujuk")}</dt>
            <dd className="num">{formatInt(data.works)}</dd>
            <dt>{t("Belum ada di koleksi (dari daftar)")}</dt>
            <dd className="num tr-missing">{formatInt(data.missing)}</dd>
          </dl>
        ) : null}
        <p className="muted words-hint">
          {t("Karya yang sering dirujuk tetapi belum ada di koleksi biasanya rujukan inti bidang ini: baca dan tambahkan ke koleksi supaya tinjauan pustaka dan analisis celah tidak melewatkannya. Status koleksi dicari di seluruh koleksi, termasuk dokumen yang sedang tersaring. Tombol tambah mengambil data lengkap dari OpenAlex lewat DOI; karya tanpa DOI bisa dicari manual di Koleksi & impor. Karya tambahan diberi asal \"OpenAlex (rujukan)\", jadi bisa dihapus bersama dari Koleksi bila tidak jadi dipakai.")}
        </p>
      </aside>
    </div>
  );
}
