# Panduan Celah

Celah adalah aplikasi desktop untuk memetakan sebuah bidang riset secara bibliometrik dan menemukan celah
penelitian yang didukung bukti dokumen. Semua data tersimpan di komputer Anda.

## 1. Mulai cepat

1. Buka **Alur kerja** (layar pertama). Layar ini menunjukkan enam langkah dan langkah berikutnya.
2. Buka **Koleksi & impor**, tulis topik riset, pilih sumber, lalu klik cari. Atau impor ekspor Scopus
   (CSV/BibTeX) atau Web of Science (TXT, *Full record and cited references*).
3. Rapikan data: gabungkan **Duplikat**, terima saran **Sinonim kata kunci**, dan satukan **Nama penulis**.
4. Jelajahi struktur bidang, lalu buka **Gap Explorer** untuk meninjau kandidat celah.
5. Tandai celah **Diterima**, verifikasi ke literatur terbaru, lalu susun laporan di **Laporan & ekspor**.

## 2. Mengumpulkan dokumen

| Sumber | Kunci API | Catatan |
| --- | --- | --- |
| OpenAlex | disarankan | Paling lengkap: abstrak, kata kunci, afiliasi, rujukan berupa ID |
| Semantic Scholar | disarankan | Tanpa kunci sering ditolak karena kuota bersama |
| PubMed | opsional | Kunci menaikkan batas 3 menjadi 10 permintaan per detik |
| Crossref | tidak | Rujukan berupa teks, berguna untuk RPYS dan rujukan teratas |
| Europe PMC, arXiv | tidak | Biomedis dan pracetak |

Kunci API diatur di **Pengaturan → Sumber data** dan disimpan di keychain sistem, bukan di file.
Koleksi yang memadai biasanya berisi ratusan sampai beberapa ribu dokumen.

**Daftar rujukan** dibutuhkan Ko-sitasi, Rujukan teratas, Akar sejarah, dan Kopling. Ekspor Scopus hanya
menyertakannya bila kolom References dicentang. Bila rujukan kosong, layar-layar itu menampilkan tombol
*Lengkapi rujukan dari Crossref*, yang mengambil daftar rujukan lewat DOI (600 dokumen sekali jalan). Mengimpor
ulang file yang sama juga melengkapi dokumen yang sudah ada tanpa menggandakannya.

**Kualitas metadata** menunjukkan kelengkapan tiap bidang dan analisis mana yang siap. Bila rujukan atau
institusi kurang, buka **Intelektual → Historiograf** atau **Penulis → Afiliasi**, lalu klik
*Lengkapi rujukan & institusi dari OpenAlex*.

## 3. Merapikan data

- **Duplikat**: pasangan rekaman yang sama dari sumber berbeda. Gabungkan agar tidak dihitung dua kali.
- **Sinonim kata kunci**: misalnya *CNN* dan *convolutional neural network*. Sinonim aktif berlaku di semua analisis.
- **Nama penulis**: ejaan berbeda untuk orang yang sama (*Rahman, M.* dan *Rahman M*).
- **Filter analisis**: batasi tahun, jenis dokumen, asal, atau sumber. Filter berlaku di semua analisis
  sampai diubah lagi.

## 4. Analisis

| Layar | Isi |
| --- | --- |
| Ringkasan | Informasi utama, produksi tahunan, sitasi per tahun, plot tiga bidang, dokumen paling disitasi |
| Sumber | Sumber paling relevan, hukum Bradford, tabel dan dinamika sumber, kuartil jurnal (SJR) |
| Penulis | Hukum Lotka, produksi dan dampak penulis, tabel penulis (dengan penyatuan nama), afiliasi |
| Negara & kolaborasi | Produksi dan dampak per negara (SCP/MCP), peta dunia dan jaringan kolaborasi negara |
| Konseptual | Peta tematik, analisis kata, ko-kemunculan, evolusi tematik, analisis faktorial, sinonim |
| Intelektual | Ko-sitasi, rujukan teratas, kopling bibliografis dan peta kopling, historiograf, akar sejarah (RPYS) |
| Sosial | Jaringan kolaborasi penulis |

Setiap menu membuka halaman sendiri; tidak ada dua menu yang menampilkan isi yang sama.

**Kuartil jurnal (SJR)**: unduh data dari scimagojr.com (*Download data*), lalu impor di Sumber, tab Kuartil (SJR).
Celah mencocokkan jurnal lewat ISSN, lalu lewat judul, dan memakai kuartil terbaik jurnal pada tahun peringkat yang
diimpor. Kuartil lalu bisa dipakai sebagai filter di Filter analisis dan muncul di Draf metode & hasil.

Grafik jaringan dan historiograf bisa di-zoom dengan roda mouse dan digeser dengan menyeret. Hampir semua
grafik bisa disimpan sebagai SVG atau PNG lewat tombol di kanan atas panelnya.

**Rujukan teratas** menandai karya yang sering dirujuk tetapi belum ada di koleksi. Karya ber-DOI bisa
ditambahkan langsung dari OpenAlex; asalnya tercatat sebagai *OpenAlex (rujukan)* sehingga bisa dihapus
bersama bila tidak jadi dipakai.

## 5. Mencari celah

**Tema peluang** mengambil tema dari kuadran niche dan baru muncul pada peta tematik, menandainya *Sedang naik*,
*Menurun*, atau *Niche matang*, lalu mengurutkannya dari yang paling menjanjikan. Dari setiap tema Anda bisa langsung
menguji gabungannya dengan konsep lain atau membuka dokumennya.

**Gap Explorer** menggabungkan beberapa detektor. Setiap kandidat punya skor (kebaruan, bukti, momentum),
dokumen bukti, dan pertanyaan riset.

| Jenis | Artinya |
| --- | --- |
| Kombinasi jarang | Dua kata kunci populer yang jarang muncul bersama |
| Baru muncul | Tema muda yang tumbuh cepat tetapi belum terhubung dengan tema lain |
| Niche | Tema matang yang terpisah dari tema utama |
| Struktural | Dua kelompok topik yang terhubung jauh lebih sedikit dari perkiraan |
| Front terisolasi | Kelompok dokumen yang banyak disitasi tetapi jarang berbagi rujukan dengan front utama |
| Eksplisit | Penulis sendiri menyebut hal yang belum diteliti (*future work*, keterbatasan) |
| Ide peneliti | Dua konsep yang Anda uji sendiri di **Uji ide celah** |

Saring jenis lewat menu di atas daftar (misalnya *Kombinasi jarang* untuk matriks gap). Tandai setiap kandidat
**Diterima**, **Perlu cek**, atau **Ditolak**. Di halaman **Verifikasi**, Celah mencari paper
terbaru di luar koleksi yang mungkin sudah menjawab celah tersebut. Skor turun bila jawabannya sudah ada.

Kandidat adalah titik awal, bukan kesimpulan. Periksa dokumen buktinya sebelum memakai celah dalam tulisan.

## 6. Laporan

- **Draf research gap**: satu paragraf per celah yang diterima, dengan sitasi APA 7 atau IEEE.
- **Draf metode & hasil**: paragraf data dan metode, kinerja publikasi, serta dokumen berpengaruh dan akar
  sejarah, disusun dari angka koleksi.

Keduanya bisa disalin atau disimpan sebagai Markdown, BibTeX, atau RIS (untuk Zotero dan Mendeley).
Periksa angka dan sesuaikan kalimatnya sebelum dipakai.

## 7. Proyek, cadangan, dan privasi

- **Proyek**: setiap proyek punya basis data sendiri. Ganti atau buat proyek dari menu di sebelah logo.
- **Cadangan**: **Koleksi & impor → Ekspor & cadangan** menyimpan salinan proyek dan memulihkannya lagi.
  Jangan menyalin berkas basis data saat aplikasi terbuka.
- **Lokasi data**: lihat **Pengaturan → Tentang**. Tombol *Salin info untuk laporan masalah* menyalin versi
  dan ukuran data tanpa isi koleksi.
- **Jaringan**: Celah hanya menghubungi API riset yang tercantum di atas, lewat HTTPS. Tidak ada telemetri.

## 8. Pintasan

| Tombol | Fungsi |
| --- | --- |
| Cmd K / Ctrl K | Cari perintah, dokumen, atau penulis |
| Tab, Enter, Spasi | Pindah dan memilih, termasuk dokumen di historiograf |
| Roda mouse, seret | Zoom dan geser grafik jaringan dan historiograf |

## 9. Masalah umum

| Gejala | Penyelesaian |
| --- | --- |
| Pencarian OpenAlex atau Semantic Scholar ditolak | Kuota bersama habis; pasang kunci API sendiri |
| Ko-sitasi, RPYS, atau rujukan teratas kosong | Butuh rujukan teks: impor Scopus/WoS atau cari lewat Crossref |
| Historiograf atau kopling kosong | Lengkapi rujukan dari OpenAlex di tab Historiograf |
| Peta negara kosong | Lengkapi afiliasi di Penulis → Afiliasi |
| Kandidat celah terasa palsu | Satukan sinonim dan nama penulis, lalu hitung ulang |
