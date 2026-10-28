import type { Jump } from "../../lib/commands";

const START: { title: string; text: string; label: string; jump: Jump; primary?: boolean }[] = [
  { title: "Cari dokumen", text: "Tulis topik riset Anda, pilih sumber (OpenAlex, PubMed, Crossref, dan lainnya), lalu simpan hasilnya ke koleksi.",
    label: "Mulai mencari", jump: { screen: "collection", tab: "collection" }, primary: true },
  { title: "Atau impor file", text: "Sudah punya ekspor Scopus, Web of Science, PubMed, atau RIS dari Zotero, Mendeley, dan EndNote? Impor dari layar yang sama; rujukannya ikut terbaca bila ada.",
    label: "Impor ekspor", jump: { screen: "collection", tab: "collection" } },
  { title: "Kunci API (opsional)", text: "Kunci OpenAlex, Semantic Scholar, atau NCBI menaikkan kuota pencarian. Kunci disimpan di keychain sistem, bukan di file.",
    label: "Atur kunci API", jump: { screen: "settings", tab: "sources" } },
];

/** First-run guide, shown while the project has no documents. */
export function Welcome({ onJump }: { onJump: (j: Jump) => void }) {
  return (
    <section className="pnl wf-welcome" aria-labelledby="wf-welcome-title">
      <h2 id="wf-welcome-title">Selamat datang di Celah</h2>
      <p className="muted">
        Celah membantu Anda memetakan sebuah bidang riset secara bibliometrik, lalu menemukan celah penelitian yang didukung
        bukti dokumen. Semua data tersimpan di komputer Anda. Mulai dengan mengisi koleksi:
      </p>
      <div className="wf-start">
        {START.map((s) => (
          <div key={s.title} className="wf-start-item">
            <strong>{s.title}</strong>
            <p className="muted">{s.text}</p>
            <button type="button" className={s.primary ? "btn btn-primary" : "btn"} onClick={() => onJump(s.jump)}>{s.label}</button>
          </div>
        ))}
      </div>
    </section>
  );
}
