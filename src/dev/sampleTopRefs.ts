import type { TopRefs } from "../lib/topRefTypes";

const WORKS: [string, number, number, boolean, string | null][] = [
  ["Lecun", 2015, 61, false, "10.1038/nature14539"], ["Qin", 2019, 48, true, "10.1038/s41598-019-51503-3"],
  ["He", 2016, 44, false, null], ["Lakhani", 2017, 39, true, "10.1148/radiol.2017162326"], ["Rajpurkar", 2017, 31, false, null],
  ["Jaeger", 2014, 28, false, null], ["Hwang", 2019, 25, true, null], ["Selvaraju", 2017, 22, false, null],
  ["Pasa", 2019, 19, true, null], ["Simonyan", 2015, 17, false, null], ["Murphy", 2020, 15, true, null], ["Dosovitskiy", 2021, 12, false, null],
];

export const SAMPLE_TOP_REFS: TopRefs = {
  documents: 553,
  citing: 312,
  works: 14820,
  refs: WORKS.map(([who, year, documents, inside, doi], i) => ({
    label: `${who} ${year}`,
    year,
    example: `${who.toUpperCase()} A, ${year}, JURNAL CONTOH, V${i + 1}, P${100 + i}${doi ? `, DOI ${doi}` : ""}`,
    doi,
    documents,
    inCollection: inside ? i + 1 : null,
    title: inside ? `Judul contoh dokumen di koleksi nomor ${i + 1}` : null,
  })),
  missing: WORKS.filter((w) => !w[3]).length,
};
