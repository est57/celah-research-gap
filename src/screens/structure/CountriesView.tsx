import { useMemo, useState, type ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useCountries } from "../../hooks/useCountries";
import { countryName } from "../../lib/countryNames";
import { formatDecimal, formatInt } from "../../lib/format";
import { TopSources } from "../sources/TopSources";
import { CorrespondingChart } from "./CorrespondingChart";
import { NetworkGraph } from "./NetworkGraph";
import { ChartExport } from "../../components/ChartExport";
import { WorldMap } from "./WorldMap";

export function CountriesView({ version }: { version: number }) {
  const [focus, setFocus] = useState<number | null>(null);
  const { data, sample, error, loading } = useCountries(version);
  const nodes = useMemo(() => data?.network.nodes.map((n) => ({ ...n, label: countryName(n.label) })) ?? [], [data]);

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? "Menghitung kolaborasi negara..." : ""}</ScreenMessage>;
  else if (data.documentsWithCountry === 0 && version === 0) main = <EmptyCollection />;
  else if (data.documentsWithCountry === 0) {
    main = (
      <ScreenMessage>
        <strong>Belum ada data negara</strong>
        <span className="muted">
          Negara afiliasi disimpan untuk dokumen yang dicari atau diimpor mulai versi ini. Cari ulang atau impor
          ulang koleksi Anda untuk mengisinya.
        </span>
      </ScreenMessage>
    );
  } else {
    main = (
      <div className="structure-main">
        {sample ? <div className="notice">Pratinjau browser: angka di halaman ini adalah data contoh.</div> : null}
        <section className="pnl">
          <div className="ph">
            Peta produksi dan kolaborasi
            <span className="ph-aside">warna = jumlah dokumen, garis = kolaborasi terkuat</span>
            <ChartExport name="Peta produksi dan kolaborasi negara" />
          </div>
          <div className="pnl-body"><WorldMap production={data.production} pairs={data.pairs} /></div>
        </section>
        <section className="pnl">
          <div className="ph">
            Jaringan kolaborasi negara
            <span className="ph-aside">
              {`${formatInt(data.documentsWithCountry)} dokumen dengan negara`}
              {data.internationalShare !== null ? `, ${formatDecimal(data.internationalShare * 100)}% internasional` : ""}
            </span>
            <ChartExport name="Jaringan kolaborasi negara" />
          </div>
          <NetworkGraph nodes={nodes} edges={data.network.edges} bridge={null} focus={focus} onFocus={setFocus} />
          <div className="muted net-caption">Ukuran node = jumlah dokumen. Tebal garis = kekuatan kolaborasi. Klik node untuk menyorot kelompoknya.</div>
        </section>
        <section className="pnl">
          <div className="ph">Produksi ilmiah negara<span className="ph-aside">dokumen dengan minimal satu penulis dari negara itu</span></div>
          <div className="pnl-body">
            <TopSources items={data.production.slice(0, 12).map((c) => ({ label: countryName(c.code), count: c.count }))} plain />
          </div>
        </section>
      </div>
    );
  }

  return (
    <div className="screen">
      <section className="screen-main">{main}</section>
      <aside className="inspector" aria-label="Negara penulis korespondensi dan pasangan kolaborasi">
        <div className="ph">Negara penulis korespondensi</div>
        <div className="pnl-body">{data && data.corresponding.length ? <CorrespondingChart rows={data.corresponding.slice(0, 10)} /> : <p className="muted">Belum ada data.</p>}</div>
        <div className="ph">Pasangan kolaborasi teratas</div>
        <ul className="pair-list">
          {(data?.pairs ?? []).map((p) => (
            <li key={`${p.a}-${p.b}`}>
              <span>{countryName(p.a)} dan {countryName(p.b)}</span>
              <span className="num">{formatInt(p.count)}</span>
            </li>
          ))}
        </ul>
      </aside>
    </div>
  );
}
