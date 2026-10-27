import { useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useAddWorks } from "../../hooks/useAddWorks";
import { useTopRefs } from "../../hooks/useTopRefs";
import { formatInt } from "../../lib/format";
import { CitedEntitiesPanel } from "./CitedEntitiesPanel";
import { TopRefList } from "./TopRefList";

const TOPS = [50, 100, 200, 500];

/** Most cited references of the collection, and which of them the collection lacks. */
export function TopRefsView({ version, onDataChanged }: { version: number; onDataChanged: () => void }) {
  const [top, setTop] = useState(100);
  const [show, setShow] = useState<"works" | "sources" | "authors">("works");
  const { data, sample, error, loading } = useTopRefs(version, top);
  const adder = useAddWorks(onDataChanged);
  const addable = data?.refs.filter((r) => r.inCollection === null && r.doi).map((r) => r.doi as string) ?? [];

  let main: ReactNode;
  if (show !== "works") main = <div className="conceptual-main"><CitedEntitiesPanel version={version} kind={show} /></div>;
  else if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? "Menghitung rujukan teratas..." : ""}</ScreenMessage>;
  else if (data.documents === 0) main = <EmptyCollection />;
  else if (data.refs.length === 0) {
    main = (
      <ScreenMessage>
        Belum ada rujukan bertahun. Daftar ini membaca rujukan dalam bentuk teks, misalnya dari impor Scopus atau Web of
        Science, atau dari pencarian Crossref. Rujukan dari OpenAlex hanya berupa ID tanpa tahun terbit.
      </ScreenMessage>
    );
  } else {
    main = (
      <div className="conceptual-main">
        {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
        <section className="pnl">
          <div className="ph">
            Rujukan paling sering dikutip
            <span className="ph-aside">{`${formatInt(data.refs.length)} teratas · angka = dokumen koleksi yang merujuk`}</span>
          </div>
          <div className="tr-add">
            <button type="button" className="btn" disabled={adder.busy || addable.length === 0} onClick={() => void adder.add(addable)}>
              {adder.busy ? "Menambahkan..." : `Tambahkan semua yang belum ada (${formatInt(addable.length)} ber-DOI)`}
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
      <aside className="inspector" aria-label="Parameter rujukan teratas">
        <div className="ph">Parameter</div>
        <div className="params">
          <div className="param-row">
            <label htmlFor="tr-show">Tampilkan</label>
            <select id="tr-show" className="input" value={show} onChange={(e) => setShow(e.target.value as typeof show)}>
              <option value="works">Karya</option>
              <option value="sources">Sumber yang dirujuk</option>
              <option value="authors">Penulis yang dirujuk</option>
            </select>
          </div>
          <div className="param-row">
            <label htmlFor="tr-top">Jumlah rujukan</label>
            <select id="tr-top" className="input" value={top} onChange={(e) => setTop(Number(e.target.value))}>
              {TOPS.map((n) => <option key={n} value={n}>{n}</option>)}
            </select>
          </div>
        </div>
        <div className="ph">Ringkasan</div>
        {data ? (
          <dl className="words-facts">
            <dt>Dokumen dengan rujukan teks</dt>
            <dd className="num">{`${formatInt(data.citing)} dari ${formatInt(data.documents)}`}</dd>
            <dt>Karya berbeda yang dirujuk</dt>
            <dd className="num">{formatInt(data.works)}</dd>
            <dt>Belum ada di koleksi (dari daftar)</dt>
            <dd className="num tr-missing">{formatInt(data.missing)}</dd>
          </dl>
        ) : null}
        <p className="muted words-hint">
          Karya yang sering dirujuk tetapi belum ada di koleksi biasanya rujukan inti bidang ini: baca dan tambahkan ke
          koleksi supaya tinjauan pustaka dan analisis celah tidak melewatkannya. Status koleksi dicari di seluruh koleksi,
          termasuk dokumen yang sedang tersaring. Tombol tambah mengambil data lengkap dari OpenAlex lewat DOI; karya
          tanpa DOI bisa dicari manual di Koleksi & impor. Karya tambahan diberi asal "OpenAlex (rujukan)", jadi bisa
          dihapus bersama dari Koleksi bila tidak jadi dipakai.
        </p>
      </aside>
    </div>
  );
}
