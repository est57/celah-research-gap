import type { ScreenId } from "../nav";
import { t, tk } from "../i18n";

/** Where a search result leads: a screen, optionally a tab in it and a prefilled search. */
export interface Jump {
  screen: ScreenId;
  tab?: string;
  query?: string;
}

export interface Command {
  /** Indonesian key; render with t(). */
  label: string;
  /** Where it lives, shown under the label. */
  place: string;
  keywords: string;
  jump: Jump;
}

const c = (label: string, place: string, keywords: string, screen: ScreenId, tab?: string): Command => ({ label, place, keywords, jump: { screen, tab } });

export const COMMANDS: Command[] = [
  c(tk("Alur kerja"), tk("Beranda"), "mulai langkah panduan kemajuan", "workflow"),
  c(tk("Koleksi & impor"), tk("Data"), "cari tambah impor openalex scopus wos pubmed", "collection", "collection"),
  c(tk("Kualitas metadata"), tk("Koleksi"), "field kosong kelengkapan", "collection", "quality"),
  c(tk("Duplikat"), tk("Koleksi"), "gabung preprint", "collection", "duplicates"),
  c(tk("Filter analisis"), tk("Koleksi"), "batasi tahun jenis dokumen sumber", "collection", "filter"),
  c(tk("Ekspor & cadangan"), tk("Koleksi"), "csv bibtex ris zotero backup database pulihkan restore", "collection", "export"),
  c(tk("Informasi utama"), tk("Ringkasan"), "ringkasan statistik", "overview", "main"),
  c(tk("Produksi tahunan"), tk("Ringkasan"), "pertumbuhan tahun", "overview", "annual"),
  c(tk("Sitasi per tahun"), tk("Ringkasan"), "rata-rata sitasi", "overview", "citations"),
  c(tk("Plot tiga bidang"), tk("Ringkasan"), "sankey", "overview", "threefield"),
  c(tk("Dokumen paling disitasi"), tk("Ringkasan"), "sitasi global lokal", "overview", "cited"),
  c(tk("Ikhtisar sumber"), tk("Sumber"), "sumber relevan hukum bradford zona jurnal", "sources", "overview"),
  c(tk("Tabel sumber"), tk("Sumber"), "dampak jurnal indeks h dinamika", "sources", "sources"),
  c(tk("Kuartil jurnal (SJR)"), tk("Sumber"), "q1 q2 q3 q4 scimago sjr peringkat jurnal kuartil impor", "sources", "quartiles"),
  c(tk("Ikhtisar penulis"), tk("Penulis"), "hukum lotka produktivitas produksi dampak lokal", "authors", "overview"),
  c(tk("Tabel penulis"), tk("Penulis"), "penulis terfraksi indeks h nama ejaan", "authors", "authors"),
  c(tk("Afiliasi"), tk("Penulis"), "institusi universitas", "authors", "affiliations"),
  c(tk("Produksi negara"), tk("Negara & kolaborasi"), "negara paling produktif sitasi mcp scp dari waktu ke waktu", "countries", "production"),
  c(tk("Kolaborasi negara"), tk("Negara & kolaborasi"), "peta dunia jaringan pasangan korespondensi", "countries", "collaboration"),
  c(tk("Peta tematik & tren"), tk("Konseptual"), "tema kuadran", "conceptual", "map"),
  c(tk("Jaringan ko-kemunculan"), tk("Konseptual"), "kata kunci jaringan", "conceptual", "network"),
  c(tk("Evolusi tematik"), tk("Konseptual"), "periode aliran tema", "conceptual", "evolution"),
  c(tk("Analisis faktorial"), tk("Konseptual"), "mca peta konseptual", "conceptual", "factorial"),
  c(tk("Analisis kata"), tk("Konseptual"), "kata paling sering frekuensi judul abstrak frasa word cloud dinamika", "conceptual", "words"),
  c(tk("Sinonim kata kunci"), tk("Konseptual"), "satukan istilah singkatan jamak ejaan", "conceptual", "synonyms"),
  c(tk("Ko-sitasi"), tk("Intelektual"), "referensi jaringan", "intellectual", "references"),
  c(tk("Rujukan teratas"), tk("Intelektual"), "rujukan paling sering dikutip local cited references belum ada di koleksi bacaan wajib", "intellectual", "toprefs"),
  c(tk("Kopling bibliografis"), tk("Intelektual"), "coupling rujukan bersama dokumen sumber penulis jaringan", "intellectual", "coupling"),
  c(tk("Kolaborasi penulis"), tk("Sosial"), "jaringan penulis", "social", "authors"),
  c(tk("Historiograf"), tk("Intelektual"), "sitasi langsung lokal rujukan", "intellectual", "historiograph"),
  c(tk("Akar sejarah (RPYS)"), tk("Intelektual"), "spektroskopi tahun rujukan klasik puncak reference publication year", "intellectual", "rpys"),
  c(tk("Tema peluang"), tk("Pencarian celah"), "tema baru muncul niche arah riset naik menurun", "opportunities"),
  c(tk("Gap Explorer"), tk("Pencarian celah"), "celah kandidat kombinasi jarang matriks struktural front", "gaps", "explorer"),
  c(tk("Uji ide celah"), tk("Pencarian celah"), "konsep gabungan ide sendiri cek", "gaps", "idea"),
  c(tk("Verifikasi celah"), tk("Pencarian celah"), "cek literatur terbaru", "verify"),
  c(tk("Riwayat review"), tk("Pencarian celah"), "diterima ditolak catatan", "gaps", "history"),
  c(tk("Laporan & ekspor"), tk("Keluaran"), "draf bibtex ris markdown", "report"),
  c(tk("Draf metode & hasil"), tk("Keluaran"), "metodologi hasil bibliometrik paragraf data sumber penulis negara", "report", "summary"),
  c(tk("Kunci API"), tk("Pengaturan"), "openalex semantic scholar ncbi keychain", "settings", "sources"),
  c(tk("Tampilan"), tk("Pengaturan"), "tema gelap terang ukuran angka", "settings", "display"),
  c(tk("Tentang Celah"), tk("Pengaturan"), "versi folder data lokasi basis data ukuran diagnostik", "settings", "about"),
];

/**
 * Commands whose label or keywords contain every typed word; label prefix matches first.
 * Labels and places are keys (tk), so both the Indonesian and the current-language text match.
 */
export function matchCommands(query: string, limit = 6): Command[] {
  const words = query.toLowerCase().split(/\s+/).filter(Boolean);
  if (words.length === 0) return [];
  const hits = COMMANDS.filter((cmd) => {
    const text = `${cmd.label} ${t(cmd.label)} ${cmd.place} ${t(cmd.place)} ${cmd.keywords}`.toLowerCase();
    return words.every((w) => text.includes(w));
  });
  const starts = (cmd: Command) => Number(![cmd.label, t(cmd.label)].some((l) => l.toLowerCase().startsWith(words[0])));
  return [...hits].sort((a, b) => starts(a) - starts(b)).slice(0, limit);
}
