// English texts for the gaps area, keyed by the Indonesian source text.
import { IDEAS } from "./ideas";
import { LABELS } from "./labels";

const EXPLORER: Record<string, string> = {
  "Kandidat celah": "Gap candidates",
  "Riwayat review": "Review history",
  "Pencarian celah": "Gap finding",
  "Mencari kandidat celah dari semua analisis...": "Finding gap candidates across all analyses...",
  "Belum ada kandidat celah": "No gap candidates yet",
  "Butuh lebih banyak dokumen yang punya kata kunci. Tambahkan koleksi dari OpenAlex, PubMed, atau impor Scopus/WoS.":
    "More documents with keywords are needed. Add to the collection from OpenAlex, PubMed, or import from Scopus/WoS.",
  "Pratinjau browser: kandidat di halaman ini adalah data contoh.": "Browser preview: the candidates on this page are sample data.",

  // Gap list
  "Daftar kandidat celah": "Gap candidate list",
  "Jenis celah": "Gap type",
  "Semua jenis ({n})": "All types ({n})",
  "Urutan": "Sort order",
  "Skor tertinggi": "Highest score",
  "Belum direview dulu": "Unreviewed first",
  "Tidak ada kandidat untuk filter ini.": "No candidates for this filter.",
  "{shown} dari {all} ditampilkan · {accepted} diterima · {rejected} ditolak":
    "{shown} of {all} shown · {accepted} accepted · {rejected} rejected",

  // Gap detail
  "Kepercayaan": "Confidence",
  "RQ": "RQ",
  "Dibangun dari sinyal analisis": "Built from analysis signals",
  "Buka": "Open",
  "Bukti dari dokumen": "Evidence from documents",
  "{n} sitasi": "{n} citations",
  "Verifikasi terhadap literatur terbaru": "Verification against recent literature",
  "Saran pertanyaan riset": "Suggested research questions",
  "Pertanyaan ini dibuat dari pola data sebagai titik awal. Periksa dan rumuskan ulang sesuai konteks riset Anda.":
    "These questions are generated from data patterns as a starting point. Check them and rephrase them for your research context.",

  // Check result
  "{m} dari {n} paper baru memuat semua kata kunci · dicek {time}": "{m} of {n} new papers contain all keywords · checked {time}",
  "Kueri:": "Query:",
  "cocok": "match",
  "terkait": "related",
  "Tidak ada paper baru di luar koleksi untuk kueri ini.": "No new papers outside the collection for this query.",

  // Review panel
  "Skor dan review": "Score and review",
  "Skor celah": "Gap score",
  "Kebaruan: seberapa jarang dibahas. Bukti: banyaknya data pendukung. Momentum: seberapa baru dokumennya dibanding koleksi.":
    "Novelty: how rarely it is discussed. Evidence: how much supporting data there is. Momentum: how recent its documents are compared with the collection.",
  "Review": "Review",
  "Status review": "Review status",
  "Catatan": "Note",
  "mis. cek ketersediaan dataset lokal": "e.g. check whether a local dataset is available",
  "Simpan catatan": "Save note",
  "Verifikasi": "Verification",
  "Cari paper 2 tahun terakhir di luar koleksi yang mungkin sudah menjawab celah ini.":
    "Search for papers from the last 2 years outside the collection that may already answer this gap.",
  "Memverifikasi...": "Verifying...",
  "Cek ulang sekarang": "Check again now",
  "Verifikasi sekarang": "Verify now",

  // Review history
  "Memuat riwayat review...": "Loading review history...",
  "Belum ada review": "No reviews yet",
  "Tandai celah sebagai Diterima, Perlu cek, atau Ditolak di Gap Explorer; keputusannya tersimpan di sini.":
    "Mark gaps as Accepted, Needs check, or Rejected in Gap Explorer; your decisions are kept here.",
  "Tidak ada review dengan status ini.": "No reviews with this status.",
  "Pratinjau browser: review di halaman ini adalah data contoh.": "Browser preview: the reviews on this page are sample data.",
  "Saring menurut status": "Filter by status",
  "Semua": "All",
  "{n} celah tidak muncul di analisis saat ini karena filter atau koleksi berubah; keputusannya tetap tersimpan.":
    "{n} gaps do not appear in the current analysis because the filter or collection changed; their decisions are still kept.",
  "diperbarui {date}": "updated {date}",
  "skor {n}": "score {n}",
  "Belum diverifikasi": "Not verified yet",
  "Tidak muncul di analisis saat ini": "Not in the current analysis",
  "Batal": "Cancel",
  "Hapus": "Remove",
  "Celah ini tidak muncul dengan filter dan koleksi saat ini": "This gap does not appear with the current filter and collection",
  "Buka di Explorer": "Open in Explorer",
  "Hapus review": "Delete review",
};

export const DICT: Record<string, string> = { ...EXPLORER, ...IDEAS, ...LABELS };
