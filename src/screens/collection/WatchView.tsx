import { useWatches } from "../../hooks/useWatches";
import { t } from "../../i18n";
import { WatchCard } from "./WatchCard";

/** Koleksi > Pantau topik: saved searches re-run to catch papers published since. */
export function WatchView({ onChanged }: { onChanged: () => void }) {
  const { watches, error, running, run, remove, sample } = useWatches(onChanged);

  return (
    <div className="dup-page watch-page">
      {sample ? <div className="notice">{t("Pratinjau browser: pantauan di halaman ini adalah data contoh.")}</div> : null}
      <section className="pnl">
        <div className="ph">{t("Pantau topik")}</div>
        <p className="muted scr-intro">
          {t("Pantauan adalah pencarian yang disimpan. Jalankan lagi kapan saja: dokumen yang sudah ada di koleksi dilewati, jadi yang ditambahkan adalah terbitan baru sejak terakhir dicek. Buat pantauan dari tombol Simpan sebagai pantauan di tab Koleksi.")}
        </p>
      </section>
      {error ? <div className="notice" role="alert">{error}</div> : null}
      {watches && watches.length === 0 ? (
        <p className="muted scr-empty">{t("Belum ada pantauan. Isi pencarian di tab Koleksi, lalu klik Simpan sebagai pantauan.")}</p>
      ) : null}
      <ul className="dup-list">
        {(watches ?? []).map((w) => (
          <WatchCard key={w.id} watch={w} running={running === w.id} onRun={() => run(w.id)} onDelete={() => void remove(w.id)} />
        ))}
      </ul>
    </div>
  );
}
