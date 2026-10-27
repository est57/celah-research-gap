# Celah

Aplikasi desktop (Tauri v2) untuk analisis bibliometrik dan pencarian celah penelitian yang didukung bukti
dokumen. Frontend React + TypeScript di `src/`, backend Rust dan SQLite di `src-tauri/`.

Panduan pengguna: [docs/PANDUAN.md](docs/PANDUAN.md).

## Pengembangan

```bash
npm ci
npm run tauri dev
```

| Perintah | Fungsi |
| --- | --- |
| `npm run dev` | Pratinjau browser dengan data contoh (tanpa backend) |
| `npm run tauri dev` | Aplikasi desktop lengkap |
| `npm run check` | Cek tipe TypeScript |
| `npm test` | Tes logika frontend (Vitest) |
| `cargo test --manifest-path src-tauri/Cargo.toml` | Tes backend |
| `cargo test --manifest-path src-tauri/Cargo.toml -- --ignored` | Tes dengan API sungguhan dan uji skala |
| `npm run tauri build` | Installer untuk sistem saat ini |
| `node scripts/gen-world.mjs` | Membuat ulang data peta dunia di `src/data/world.json` |

Linux butuh `libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libdbus-1-dev`.

## Rilis

Workflow `.github/workflows/build.yml` menjalankan pemeriksaan di setiap push dan pull request. Tag `v*`
juga membuat installer macOS (Apple Silicon dan Intel), Windows, dan Linux sebagai draf rilis. Untuk tanda
tangan macOS, isi secret `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY`,
`APPLE_ID`, `APPLE_PASSWORD`, dan `APPLE_TEAM_ID`.

## Aturan kode

Lihat [CLAUDE.md](CLAUDE.md): tanpa emoji, file pendek (di bawah 150 baris), SQL berparameter, validasi
semua masukan `invoke()`, HTTP hanya ke API riset yang diizinkan, dan tampilan responsif.

## Lisensi

MIT. Lihat [LICENSE](LICENSE).
