import { useEffect, useState } from "react";
import { isDesktop } from "../../lib/api";
import { appInfo, type AppInfo } from "../../lib/apiApp";
import { formatDecimal, formatInt } from "../../lib/format";
import { t } from "../../i18n";

const size = (bytes: number) => (bytes >= 1048576 ? `${formatDecimal(bytes / 1048576)} MB` : `${formatDecimal(bytes / 1024)} KB`);

/** Version and storage facts, with a copyable summary for support requests. */
export function AboutPanel() {
  const [info, setInfo] = useState<AppInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [copied, setCopied] = useState<string | null>(null);
  useEffect(() => {
    if (isDesktop()) appInfo().then(setInfo).catch((e) => setError(e instanceof Error ? e.message : String(e)));
  }, []);

  const copy = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopied(t("Tersalin"));
    } catch {
      setCopied(t("Gagal menyalin; pilih teks lalu salin manual."));
    }
  };
  const summary = info
    ? t("Celah {version} ({os} {arch}), SQLite {sqlite}, {n} dokumen, basis data {size}", { version: info.version, os: info.os, arch: info.arch, sqlite: info.sqlite, n: info.documents, size: size(info.databaseBytes) })
    : "";

  return (
    <section className="pnl key-panel">
      <div className="ph">{t("Tentang Celah")}</div>
      <div className="about-body">
        <p className="muted">
          {t("Celah memetakan bidang riset secara bibliometrik dan mencari celah penelitian yang didukung dokumen. Semua koleksi, review, dan pengaturan disimpan di komputer ini; kunci API disimpan di keychain sistem.")}
        </p>
        {!isDesktop() ? <div className="notice">{t("Informasi versi dan penyimpanan hanya tersedia di aplikasi desktop.")}</div> : null}
        {error ? <p className="refs-error" role="alert">{error}</p> : null}
        {info ? (
          <>
            <dl className="words-facts about-facts">
              <dt>{t("Versi")}</dt><dd className="num">{info.version}</dd>
              <dt>{t("Sistem")}</dt><dd className="num">{`${info.os} ${info.arch}`}</dd>
              <dt>SQLite</dt><dd className="num">{info.sqlite}</dd>
              <dt>{t("Dokumen di proyek ini")}</dt><dd className="num">{formatInt(info.documents)}</dd>
              <dt>{t("Ukuran basis data")}</dt><dd className="num">{size(info.databaseBytes)}</dd>
              <dt>{t("Berkas basis data")}</dt><dd className="num select">{info.database}</dd>
              <dt>{t("Folder data")}</dt><dd className="num select about-path">{info.dataDir}</dd>
            </dl>
            <div className="about-actions">
              <button type="button" className="btn" onClick={() => void copy(info.dataDir)}>{t("Salin lokasi folder")}</button>
              <button type="button" className="btn" onClick={() => void copy(summary)}>{t("Salin info untuk laporan masalah")}</button>
              {copied ? <span className="muted" role="status">{copied}</span> : null}
            </div>
            <p className="muted about-hint">
              {t("Untuk memindahkan atau mencadangkan proyek, pakai Koleksi & impor, tab Ekspor & cadangan. Jangan menyalin berkas basis data saat aplikasi sedang terbuka.")}
            </p>
          </>
        ) : null}
      </div>
    </section>
  );
}
