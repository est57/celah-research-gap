// English texts for the conceptual area, keyed by the Indonesian source text.
import { PART2 } from "./part2";

export const DICT: Record<string, string> = {
  // Screen and tabs
  "Peta tematik & tren": "Thematic map & trends",
  "Analisis kata": "Word analysis",
  "Jaringan ko-kemunculan": "Co-occurrence network",
  "Evolusi tematik": "Thematic evolution",
  "Analisis faktorial": "Factorial analysis",
  "Sinonim kata kunci": "Keyword synonyms",
  "Struktur konseptual": "Conceptual structure",
  "Menyusun peta tematik...": "Building the thematic map...",
  "Belum ada tema yang memenuhi syarat. Coba turunkan frekuensi minimum tema, atau pilih field kata kunci lain.":
    "No themes meet the criteria yet. Try lowering the minimum theme frequency, or choose another keyword field.",
  "Pratinjau browser: angka di halaman ini adalah data contoh.": "Browser preview: the numbers on this page are sample data.",
  "Peta tematik": "Thematic map",
  "{n} tema dari {words} {field} · klik tema untuk detail": "{n} themes from {words} {field} · click a theme for details",
  "Tren topik": "Trend topics",
  "rentang Q1-Q3 tahun terbit, titik = median, ukuran = frekuensi": "Q1-Q3 range of publication years, dot = median, size = frequency",
  "Parameter dan detail tema": "Parameters and theme details",
  "Parameter": "Parameters",
  "Tema dipilih": "Selected theme",
  "Belum ada tema.": "No themes yet.",

  // Parameters form
  "Field": "Field",
  "Otomatis": "Automatic",
  "Kata kunci penulis": "Author keywords",
  "Jumlah kata": "Number of words",
  "Frek. min. tema (per 1.000 dok)": "Min. theme freq. (per 1,000 docs)",
  "Label per tema": "Labels per theme",
  "Frek. min. tren": "Min. trend freq.",
  "Keywords Plus": "Keywords Plus",
  "Louvain": "Louvain",
  "Algoritma": "Algorithm",
  "Atur ulang": "Reset",
  "Terapkan": "Apply",

  // Thematic map and theme detail
  "Densitas (tingkat perkembangan)": "Density (degree of development)",
  "Sentralitas (tingkat relevansi)": "Centrality (degree of relevance)",
  "{name}, {quadrant}, {n} dokumen": "{name}, {quadrant}, {n} documents",
  "Kemungkinan tema baru muncul: {share} dokumennya terbit {period}, di atas rata-rata koleksi ({avg}). Sentralitasnya rendah, artinya tema ini masih jarang terhubung dengan tema utama.":
    "Possibly an emerging theme: {share} of its documents were published in {period}, above the collection average ({avg}). Its centrality is low, so the theme is still rarely linked to the main themes.",
  "Kemungkinan tema menurun: hanya {share} dokumennya terbit {period} (rata-rata koleksi {avg}).":
    "Possibly a declining theme: only {share} of its documents were published in {period} (collection average {avg}).",
  "Tema niche: berkembang di dalam kelompoknya sendiri tetapi jarang terhubung dengan tema lain. Kaitannya dengan tema motor bisa menjadi celah riset.":
    "Niche theme: well developed within its own group but rarely linked to other themes. Its connection to motor themes may be a research gap.",
  "dokumen": "documents",
  "sentralitas": "centrality",
  "densitas": "density",
  "Sinyal celah": "Gap signal",
  "Belum ada kata kunci yang cukup sering untuk dihitung trennya.": "No keywords are frequent enough to compute a trend yet.",
  "{label}: Q1 {q1}, median {median}, Q3 {q3}, {n} dokumen": "{label}: Q1 {q1}, median {median}, Q3 {q3}, {n} documents",

  // Thematic evolution
  "Menyusun peta tematik tiap periode...": "Building a thematic map for each period...",
  "Belum ada tema di periode mana pun. Tambahkan dokumen yang punya kata kunci.": "No themes in any period yet. Add documents that have keywords.",
  "Pratinjau browser: aliran tema di halaman ini adalah data contoh.": "Browser preview: the theme flows on this page are sample data.",
  "{n} periode · {field} · klik tema untuk detail": "{n} periods · {field} · click a theme for details",
  "Tinggi = kemunculan kata kunci. Tebal aliran = kata kunci yang sama.": "Height = keyword occurrences. Flow width = shared keywords.",
  "Klik tema di grafik untuk melihat asal dan kelanjutannya.": "Click a theme in the chart to see where it comes from and where it leads.",
  "Aliran tema antar periode": "Theme flows between periods",
  "{n} dok": "{n} docs",
  "{pct}% inklusi: {words}": "{pct}% inclusion: {words}",
  "{name} ({n} kemunculan kata kunci)": "{name} ({n} keyword occurrences)",
  "Perubahan tema": "Theme changes",
  "pada inklusi minimum {pct}%": "at minimum inclusion {pct}%",
  "Sinyal celah: tema baru di {period}": "Gap signal: new themes in {period}",
  "Tema ini tidak berasal dari tema periode sebelumnya. Arah baru seperti ini belum banyak dikaitkan dengan literatur yang lebih mapan.":
    "These themes do not come from any theme of the previous period. New directions like these are not yet well connected to the more established literature.",
  "Baru": "New",
  "Tidak berlanjut": "Discontinued",
  "tidak ada": "none",
  "Tidak ada di atas inklusi minimum.": "None above the minimum inclusion.",
  "{period} · {quadrant} · {n} dokumen": "{period} · {quadrant} · {n} documents",
  "Berasal dari": "Comes from",
  "Berlanjut ke": "Continues into",
  "Isi 1 sampai 3 tahun berurutan, misalnya 2019, 2022. Kosongkan untuk otomatis.":
    "Enter 1 to 3 years in ascending order, for example 2019, 2022. Leave empty for automatic.",
  "Tahun potong": "Cut years",
  "otomatis": "automatic",
  "Tahun potong ikut periode sebelumnya.": "Each cut year belongs to the earlier period.",
  "Otomatis: {cuts}, agar jumlah dokumen tiap periode seimbang.": "Automatic: {cuts}, so each period has a similar number of documents.",
  "satu periode": "one period",
  "Kata per periode": "Words per period",
  "Inklusi minimum": "Minimum inclusion",
  ...PART2,
};
