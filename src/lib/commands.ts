import type { ScreenId } from "../nav";

/** Where a search result leads: a screen, optionally a tab in it and a prefilled search. */
export interface Jump {
  screen: ScreenId;
  tab?: string;
  query?: string;
}

export interface Command {
  label: string;
  /** Where it lives, shown under the label. */
  place: string;
  keywords: string;
  jump: Jump;
}

const c = (label: string, place: string, keywords: string, screen: ScreenId, tab?: string): Command => ({ label, place, keywords, jump: { screen, tab } });

export const COMMANDS: Command[] = [
  c("Alur kerja", "Beranda", "mulai langkah panduan kemajuan", "workflow"),
  c("Koleksi & impor", "Data", "cari tambah impor openalex scopus wos pubmed", "collection", "collection"),
  c("Kualitas metadata", "Koleksi", "field kosong kelengkapan", "collection", "quality"),
  c("Duplikat", "Koleksi", "gabung preprint", "collection", "duplicates"),
  c("Filter analisis", "Koleksi", "batasi tahun jenis dokumen sumber", "collection", "filter"),
  c("Ekspor & cadangan", "Koleksi", "csv bibtex ris zotero backup database pulihkan restore", "collection", "export"),
  c("Informasi utama", "Ringkasan", "ringkasan statistik", "overview", "main"),
  c("Produksi tahunan", "Ringkasan", "pertumbuhan tahun", "overview", "annual"),
  c("Sitasi per tahun", "Ringkasan", "rata-rata sitasi", "overview", "citations"),
  c("Plot tiga bidang", "Ringkasan", "sankey", "overview", "threefield"),
  c("Dokumen paling disitasi", "Ringkasan", "sitasi global lokal", "overview", "cited"),
  c("Hukum Bradford dan Lotka", "Sumber & penulis", "ikhtisar produktivitas", "sources", "overview"),
  c("Dampak sumber", "Sumber & penulis", "jurnal indeks h zona", "sources", "sources"),
  c("Tabel penulis", "Sumber & penulis", "penulis terfraksi indeks h", "authors", "authors"),
  c("Afiliasi", "Sumber & penulis", "institusi universitas", "sources", "affiliations"),
  c("Negara paling produktif", "Sumber & penulis", "negara sitasi mcp scp produksi dari waktu ke waktu", "sources", "countries"),
  c("Negara & kolaborasi", "Struktur", "scp mcp negara", "countries", "countries"),
  c("Peta tematik & tren", "Konseptual", "tema kuadran", "conceptual", "map"),
  c("Jaringan ko-kemunculan", "Konseptual", "kata kunci jaringan", "conceptual", "network"),
  c("Evolusi tematik", "Konseptual", "periode aliran tema", "conceptual", "evolution"),
  c("Analisis faktorial", "Konseptual", "mca peta konseptual", "conceptual", "factorial"),
  c("Analisis kata", "Konseptual", "kata paling sering frekuensi judul abstrak frasa word cloud dinamika", "conceptual", "words"),
  c("Sinonim kata kunci", "Konseptual", "satukan istilah singkatan jamak ejaan", "conceptual", "synonyms"),
  c("Ko-sitasi", "Intelektual", "referensi jaringan", "intellectual", "references"),
  c("Rujukan teratas", "Intelektual", "rujukan paling sering dikutip local cited references belum ada di koleksi bacaan wajib", "intellectual", "toprefs"),
  c("Kopling bibliografis", "Intelektual", "coupling rujukan bersama dokumen sumber penulis jaringan", "intellectual", "coupling"),
  c("Kolaborasi penulis", "Sosial", "jaringan penulis", "social", "authors"),
  c("Historiograf", "Intelektual", "sitasi langsung lokal rujukan", "intellectual", "historiograph"),
  c("Akar sejarah (RPYS)", "Intelektual", "spektroskopi tahun rujukan klasik puncak reference publication year", "intellectual", "rpys"),
  c("Gap Explorer", "Pencarian celah", "celah kandidat", "gaps", "explorer"),
  c("Uji ide celah", "Pencarian celah", "konsep gabungan ide sendiri cek", "gaps", "idea"),
  c("Matriks gap", "Pencarian celah", "kombinasi jarang", "matrix", "explorer"),
  c("Verifikasi celah", "Pencarian celah", "cek literatur terbaru", "verify", "verify"),
  c("Riwayat review", "Pencarian celah", "diterima ditolak catatan", "gaps", "history"),
  c("Laporan & ekspor", "Keluaran", "draf bibtex ris markdown", "report"),
  c("Draf metode & hasil", "Keluaran", "metodologi hasil bibliometrik paragraf data sumber penulis negara", "report", "summary"),
  c("Kunci API", "Pengaturan", "openalex semantic scholar ncbi keychain", "settings", "sources"),
  c("Tampilan", "Pengaturan", "tema gelap terang ukuran angka", "settings", "display"),
  c("Tentang Celah", "Pengaturan", "versi folder data lokasi basis data ukuran diagnostik", "settings", "about"),
];

/** Commands whose label or keywords contain every typed word; label prefix matches first. */
export function matchCommands(query: string, limit = 6): Command[] {
  const words = query.toLowerCase().split(/\s+/).filter(Boolean);
  if (words.length === 0) return [];
  const hits = COMMANDS.filter((cmd) => {
    const text = `${cmd.label} ${cmd.place} ${cmd.keywords}`.toLowerCase();
    return words.every((w) => text.includes(w));
  });
  const starts = (cmd: Command) => Number(!cmd.label.toLowerCase().startsWith(words[0]));
  return [...hits].sort((a, b) => starts(a) - starts(b)).slice(0, limit);
}
