// English texts for the idea checker, opportunity themes and verification screens.
export const IDEAS: Record<string, string> = {
  "Uji ide celah": "Test a gap idea",
  "Tulis dua konsep yang ingin Anda gabungkan. Pisahkan sebutan lain dengan koma, misalnya \"vision transformer, ViT\". Filter analisis dan sinonim kata kunci ikut berlaku.":
    "Enter two concepts you want to combine. Separate alternative terms with commas, for example \"vision transformer, ViT\". The analysis filter and keyword synonyms apply.",
  "Konsep A": "Concept A",
  "Konsep B": "Concept B",
  "Keduanya": "Both",
  "keduanya": "both",
  "Menghitung...": "Counting...",
  "Uji": "Test",
  "Simpan agar ide ini tampil di Gap Explorer, bisa direview, diverifikasi, dan masuk ke laporan.":
    "Save it so this idea shows up in Gap Explorer, where it can be reviewed, verified and included in the report.",
  "Tersimpan di Gap Explorer": "Saved to Gap Explorer",
  "Simpan ke Gap Explorer": "Save to Gap Explorer",
  "Literatur terbaru di luar koleksi": "Recent literature outside the collection",
  "OpenAlex, 2 tahun terakhir": "OpenAlex, last 2 years",
  "Pastikan celah ini belum dijawab oleh paper yang belum ada di koleksi Anda.":
    "Make sure this gap has not been answered by papers that are not in your collection.",
  "Mencari...": "Searching...",
  "Cek literatur terbaru": "Check recent literature",
  "Tidak ada.": "None.",
  "{na} dokumen membahas \"{a}\", {nb} membahas \"{b}\", dan {both} membahas keduanya. Bila keduanya tidak saling terkait, diperkirakan sekitar {expected} dokumen. {verdict}":
    "{na} documents discuss \"{a}\", {nb} discuss \"{b}\", and {both} discuss both. If the two were unrelated, about {expected} documents would be expected. {verdict}",
  "diharapkan": "expected",
  "Porsi dokumen 3 tahun terakhir: \"{a}\" {pa}, \"{b}\" {pb}, seluruh koleksi {pall}.":
    "Share of documents from the last 3 years: \"{a}\" {pa}, \"{b}\" {pb}, whole collection {pall}.",
  "Setidaknya satu konsep sedang naik.": "At least one concept is rising.",
  "Dokumen per tahun": "Documents per year",
  "Dokumen per tahun untuk kedua konsep dan gabungannya": "Documents per year for both concepts and their combination",
  "Menggabungkan keduanya": "Combining both",
  "Hanya \"{term}\" (paling disitasi)": "Only \"{term}\" (most cited)",
  "Ide tersimpan": "Saved ideas",
  "tampil di Gap Explorer sebagai Ide peneliti": "shown in Gap Explorer as Researcher idea",
  "dengan": "with",

  // Opportunity themes
  "Tema peluang": "Opportunity themes",
  "Mencari tema peluang...": "Finding opportunity themes...",
  "Belum ada tema niche atau baru muncul. Tambahkan dokumen, atau ubah parameter di Konseptual, Peta tematik.":
    "No niche or emerging themes yet. Add documents, or change the parameters in Conceptual, Thematic map.",
  "Pratinjau browser: tema di halaman ini adalah data contoh.": "Browser preview: the themes on this page are sample data.",
  "{n} dari {total} tema berada di kuadran niche atau baru muncul. Diurutkan dari yang paling menjanjikan: tumbuh lebih cepat dari koleksi, jarang terhubung dengan tema utama, dan didukung cukup dokumen.":
    "{n} of {total} themes fall in the niche or emerging quadrant. Sorted from most promising: growing faster than the collection, rarely linked to the main themes, and backed by enough documents.",
  "Posisi di peta tematik": "Position on the thematic map",
  "tema terpilih ditandai · klik tema di peta atau di daftar": "selected theme highlighted · click a theme on the map or in the list",
  "Kandidat ini titik awal, bukan kesimpulan. Baca dokumen tema yang dipilih, lalu uji gabungannya dengan konsep lain di Uji ide celah sebelum menetapkan arah riset. Peta lengkap dan parameternya ada di Konseptual.":
    "These candidates are a starting point, not a conclusion. Read the documents of the selected theme, then test its combination with other concepts in Test a gap idea before settling on a research direction. The full map and its parameters are in Conceptual.",
  "{pct}% dokumennya terbit {span}, {pace} kali laju koleksi, dan masih jarang terhubung dengan tema utama.":
    "{pct}% of its documents were published in {span}, {pace} times the collection's pace, and it is still rarely linked to the main themes.",
  "Hanya {pct}% dokumennya terbit {span}. Bisa jadi tema yang ditinggalkan; periksa alasannya sebelum memilih.":
    "Only {pct}% of its documents were published in {span}. It may be an abandoned theme; check why before choosing it.",
  "Tema yang sudah berkembang di dalam kelompoknya sendiri, tetapi jarang bersinggungan dengan tema lain.":
    "A theme well developed within its own cluster, but rarely intersecting with other themes.",
  "{n} dokumen · sentralitas {c} · densitas {d}": "{n} documents · centrality {c} · density {d}",
  "Uji dengan ide lain": "Test with another idea",
  "Lihat dokumen": "View documents",

  // Verification
  "Verifikasi literatur terbaru": "Recent literature verification",
  "Memuat kandidat celah...": "Loading gap candidates...",
  "Belum ada kandidat celah untuk diverifikasi. Tinjau dan terima kandidat di Gap Explorer dulu.":
    "No gap candidates to verify yet. Review and accept candidates in Gap Explorer first.",
  "{ok} celah dicek, {failed} gagal: {error}": "{ok} gaps checked, {failed} failed: {error}",
  "{n} celah selesai dicek terhadap literatur terbaru.": "{n} gaps checked against recent literature.",
  "Verifikasi celah yang diterima": "Verify accepted gaps",
  "Untuk setiap celah, Celah mencari paper OpenAlex 2 tahun terakhir yang belum ada di koleksi. Paper yang judul atau abstraknya memuat semua kata kunci celah ditandai \"cocok\". Tiga atau lebih paper cocok berarti celah itu mungkin sudah mulai dijawab, dan skornya diturunkan.":
    "For each gap, Celah searches OpenAlex for papers from the last 2 years that are not in the collection. Papers whose title or abstract contains all of the gap's keywords are marked \"match\". Three or more matching papers mean the gap may already be getting answered, and its score is lowered.",
  "Verifikasi semua ({n})": "Verify all ({n})",
  "Pratinjau browser: verifikasi hanya berjalan di aplikasi desktop.": "Browser preview: verification only runs in the desktop app.",
  "Belum ada celah yang diterima. Tandai celah sebagai \"Diterima\" di Gap Explorer untuk memverifikasinya.":
    "No accepted gaps yet. Mark gaps as \"Accepted\" in Gap Explorer to verify them.",
  "Cek ulang": "Check again",
  "Belum pernah diverifikasi.": "Never verified.",
};
