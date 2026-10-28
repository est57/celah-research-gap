import { docTypeLabel, formatDecimal, formatInt } from "../../lib/format";
import type { MainInfo } from "../../lib/overviewTypes";
import { t } from "../../i18n";

type Row = [label: string, value: string, strong?: boolean];

function groups(i: MainInfo): [string, Row[]][] {
  const opt = (v: number | null, f: (n: number) => string) => (v === null ? "-" : f(v));
  const span = i.yearMin !== null && i.yearMax !== null ? `${i.yearMin} - ${i.yearMax}` : "-";
  return [
    [t("Deskripsi"), [
      [t("Rentang waktu"), span],
      [t("Sumber (jurnal, prosiding)"), formatInt(i.sources)],
      [t("Dokumen"), formatInt(i.documents), true],
      [t("Laju pertumbuhan tahunan"), opt(i.growthRate, (g) => `${formatDecimal(g)}%`), true],
      [t("Rata-rata usia dokumen"), opt(i.avgAge, (a) => t("{n} th", { n: formatDecimal(a) }))],
      [t("Rata-rata sitasi per dokumen"), formatDecimal(i.avgCitations)],
      [t("Referensi unik"), formatInt(i.references)],
    ]],
    [t("Isi dokumen"), [
      ["Keywords Plus (ID)", formatInt(i.keywordsPlus)],
      [t("Kata kunci penulis (DE)"), formatInt(i.authorKeywords)],
    ]],
    [t("Penulis"), [
      [t("Penulis"), formatInt(i.authors)],
      [t("Penulis dokumen tunggal"), formatInt(i.authorsOfSingle)],
      [t("Dokumen penulis tunggal"), formatInt(i.singleAuthorDocs)],
      [t("Ko-penulis per dokumen"), opt(i.coauthorsPerDoc, (c) => formatDecimal(c))],
      [t("Ko-penulisan internasional"), opt(i.internationalCoauthorship, (s) => `${formatDecimal(s * 100)}%`), true],
    ]],
    [t("Jenis dokumen"), i.docTypes.map((d): Row => [d.docType === "lainnya" ? t("Lainnya") : t(docTypeLabel(d.docType)), formatInt(d.count)])],
  ];
}

export function MainInfoPanel({ info }: { info: MainInfo }) {
  return (
    <aside className="main-info" aria-label={t("Informasi utama")}>
      {groups(info).map(([title, rows]) => (
        <section key={title}>
          <h3>{title}</h3>
          <dl>
            {rows.map(([label, value, strong]) => (
              <div key={label} className="kv">
                <dt>{label}</dt>
                <dd className="num" data-strong={strong ? "true" : undefined}>{value}</dd>
              </div>
            ))}
          </dl>
        </section>
      ))}
      <p className="muted main-info-note">
        {t("Jumlah penulis dan ko-penulisan internasional dihitung dari dokumen yang punya daftar penulis dan negara afiliasi. Dokumen yang diimpor sebelum fitur itu ada belum ikut terhitung.")}
      </p>
    </aside>
  );
}
