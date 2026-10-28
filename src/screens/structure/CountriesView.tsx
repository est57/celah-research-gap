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
import { t } from "../../i18n";

export function CountriesView({ version }: { version: number }) {
  const [focus, setFocus] = useState<number | null>(null);
  const { data, sample, error, loading } = useCountries(version);
  const nodes = useMemo(() => data?.network.nodes.map((n) => ({ ...n, label: countryName(n.label) })) ?? [], [data]);

  let main: ReactNode;
  if (error) main = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) main = <ScreenMessage>{loading ? t("Menghitung kolaborasi negara...") : ""}</ScreenMessage>;
  else if (data.documentsWithCountry === 0 && version === 0) main = <EmptyCollection />;
  else if (data.documentsWithCountry === 0) {
    main = (
      <ScreenMessage>
        <strong>{t("Belum ada data negara")}</strong>
        <span className="muted">
          {t("Negara afiliasi disimpan untuk dokumen yang dicari atau diimpor mulai versi ini. Cari ulang atau impor ulang koleksi Anda untuk mengisinya.")}
        </span>
      </ScreenMessage>
    );
  } else {
    main = (
      <div className="structure-main">
        {sample ? <div className="notice">{t("Pratinjau browser: angka di halaman ini adalah data contoh.")}</div> : null}
        <section className="pnl">
          <div className="ph">
            {t("Peta produksi dan kolaborasi")}
            <span className="ph-aside">{t("warna = jumlah dokumen, garis = kolaborasi terkuat")}</span>
            <ChartExport name={t("Peta produksi dan kolaborasi negara")} />
          </div>
          <div className="pnl-body"><WorldMap production={data.production} pairs={data.pairs} /></div>
        </section>
        <section className="pnl">
          <div className="ph">
            {t("Jaringan kolaborasi negara")}
            <span className="ph-aside">
              {data.internationalShare !== null
                ? t("{n} dokumen dengan negara, {share}% internasional", { n: formatInt(data.documentsWithCountry), share: formatDecimal(data.internationalShare * 100) })
                : t("{n} dokumen dengan negara", { n: formatInt(data.documentsWithCountry) })}
            </span>
            <ChartExport name={t("Jaringan kolaborasi negara")} />
          </div>
          <NetworkGraph nodes={nodes} edges={data.network.edges} bridge={null} focus={focus} onFocus={setFocus} />
          <div className="muted net-caption">{t("Ukuran node = jumlah dokumen. Tebal garis = kekuatan kolaborasi. Klik node untuk menyorot kelompoknya.")}</div>
        </section>
        <section className="pnl">
          <div className="ph">{t("Produksi ilmiah negara")}<span className="ph-aside">{t("dokumen dengan minimal satu penulis dari negara itu")}</span></div>
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
      <aside className="inspector" aria-label={t("Negara penulis korespondensi dan pasangan kolaborasi")}>
        <div className="ph">{t("Negara penulis korespondensi")}</div>
        <div className="pnl-body">{data && data.corresponding.length ? <CorrespondingChart rows={data.corresponding.slice(0, 10)} /> : <p className="muted">{t("Belum ada data.")}</p>}</div>
        <div className="ph">{t("Pasangan kolaborasi teratas")}</div>
        <ul className="pair-list">
          {(data?.pairs ?? []).map((p) => (
            <li key={`${p.a}-${p.b}`}>
              <span>{t("{a} dan {b}", { a: countryName(p.a), b: countryName(p.b) })}</span>
              <span className="num">{formatInt(p.count)}</span>
            </li>
          ))}
        </ul>
      </aside>
    </div>
  );
}
