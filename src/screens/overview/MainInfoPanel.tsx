import { docTypeLabel, formatDecimal, formatInt } from "../../lib/format";
import type { MainInfo } from "../../lib/overviewTypes";

type Row = [label: string, value: string, strong?: boolean];

function groups(i: MainInfo): [string, Row[]][] {
  const opt = (v: number | null, f: (n: number) => string) => (v === null ? "-" : f(v));
  const span = i.yearMin !== null && i.yearMax !== null ? `${i.yearMin} - ${i.yearMax}` : "-";
  return [
    ["Deskripsi", [
      ["Rentang waktu", span],
      ["Sumber (jurnal, prosiding)", formatInt(i.sources)],
      ["Dokumen", formatInt(i.documents), true],
      ["Laju pertumbuhan tahunan", opt(i.growthRate, (g) => `${formatDecimal(g)}%`), true],
      ["Rata-rata usia dokumen", opt(i.avgAge, (a) => `${formatDecimal(a)} th`)],
      ["Rata-rata sitasi per dokumen", formatDecimal(i.avgCitations)],
      ["Referensi unik", formatInt(i.references)],
    ]],
    ["Isi dokumen", [
      ["Keywords Plus (ID)", formatInt(i.keywordsPlus)],
      ["Kata kunci penulis (DE)", formatInt(i.authorKeywords)],
    ]],
    ["Penulis", [
      ["Penulis", formatInt(i.authors)],
      ["Penulis dokumen tunggal", formatInt(i.authorsOfSingle)],
      ["Dokumen penulis tunggal", formatInt(i.singleAuthorDocs)],
      ["Ko-penulis per dokumen", opt(i.coauthorsPerDoc, (c) => formatDecimal(c))],
      ["Ko-penulisan internasional", opt(i.internationalCoauthorship, (s) => `${formatDecimal(s * 100)}%`), true],
    ]],
    ["Jenis dokumen", i.docTypes.map((t): Row => [t.docType === "lainnya" ? "Lainnya" : docTypeLabel(t.docType), formatInt(t.count)])],
  ];
}

export function MainInfoPanel({ info }: { info: MainInfo }) {
  return (
    <aside className="main-info" aria-label="Informasi utama">
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
        Jumlah penulis dan ko-penulisan internasional dihitung dari dokumen yang punya daftar penulis dan negara
        afiliasi. Dokumen yang diimpor sebelum fitur itu ada belum ikut terhitung.
      </p>
    </aside>
  );
}
