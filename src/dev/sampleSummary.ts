import type { Report } from "../lib/reportTypes";

export const SAMPLE_SUMMARY: Report = {
  title: "Metode dan hasil bibliometrik",
  intro: "Draf ini disusun otomatis dari koleksi; periksa angka dan sesuaikan kalimatnya sebelum dipakai.",
  paragraphs: [
    { key: "method", heading: "Data dan metode", text: "Data dihimpun dari OpenAlex (412), Scopus (118), dan Crossref (23). Rekaman ganda antar sumber digabung (41 rekaman). Koleksi yang dianalisis berisi 553 dokumen terbitan 2015-2025. Analisis bibliometrik dilakukan dengan aplikasi Celah, mencakup analisis kinerja (produksi tahunan, sumber, penulis, dan negara) serta pemetaan sains (ko-kemunculan kata kunci, ko-sitasi, kopling bibliografis, dan spektroskopi tahun rujukan) mengikuti alur bibliometrix (Aria & Cuccurullo, 2017)." },
    { key: "performance", heading: "Kinerja publikasi", text: "Koleksi ini memuat 553 dokumen dari 214 sumber, ditulis oleh 2.140 penulis dengan rata-rata 18,4 sitasi per dokumen. Produksi tumbuh rata-rata 21,3% per tahun. Sumber paling produktif adalah Diagnostics (31), IEEE Access (27), dan Scientific Reports (22). Penulis paling produktif adalah Contoh, A. (12), Peneliti, B. (9), dan Penulis, C. (8). Kontribusi terbesar datang dari India (142), Tiongkok (118), dan Amerika Serikat (96), dengan 24,6% dokumen melibatkan kolaborasi internasional." },
    { key: "influence", heading: "Dokumen berpengaruh dan akar sejarah", text: "Dokumen paling berpengaruh dalam koleksi adalah Contoh et al. (2019) dengan 312 sitasi, Peneliti & Penulis (2020) dengan 240 sitasi, dan Penulis (2021) dengan 198 sitasi. Spektroskopi tahun rujukan (Marx et al., 2014) menunjukkan akar sejarah bidang ini pada tahun 1998 (Lecun 1998), 2006 (Davis 2006), dan 2012 (Krizhevsky 2012)." },
  ],
  referencesTitle: "Daftar pustaka",
  references: [
    "Aria, M., & Cuccurullo, C. (2017). bibliometrix: An R-tool for comprehensive science mapping analysis. Journal of Informetrics. https://doi.org/10.1016/j.joi.2017.08.007",
    "Contoh, A., Peneliti, B., & Penulis, C. (2019). Judul contoh dokumen paling disitasi. Jurnal Contoh.",
  ],
  bibtex: "@article{aria2017,\n  title = {bibliometrix: An R-tool for comprehensive science mapping analysis}\n}",
  ris: "TY  - JOUR\nTI  - bibliometrix: An R-tool for comprehensive science mapping analysis\nER  - ",
  markdown: "",
};
