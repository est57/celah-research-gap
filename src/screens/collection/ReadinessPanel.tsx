import { formatInt, formatPct } from "../../lib/format";
import { READINESS_LABEL, readiness, type MetadataQuality } from "../../lib/qualityTypes";
import { t, tk } from "../../i18n";

const HINTS: Record<string, string> = {
  conceptual: tk("Butuh kata kunci penulis atau Keywords Plus. Ekspor Scopus, Web of Science, dan PubMed (MeSH) biasanya paling lengkap."),
  intellectual: tk("Butuh teks daftar referensi. OpenAlex dan Semantic Scholar hanya memberi jumlahnya, jadi impor ekspor Scopus/WoS yang menyertakan referensi, atau tambahkan dari Crossref dan PubMed."),
  historiograph: tk("Butuh daftar rujukan. Buka Intelektual > Historiograf lalu jalankan Lengkapi rujukan & institusi dari OpenAlex untuk dokumen yang punya ID OpenAlex atau DOI."),
  social: tk("Hanya dokumen dengan lebih dari satu penulis yang membentuk jaringan kolaborasi."),
  countries: tk("Negara dibaca dari afiliasi penulis. Dokumen tanpa afiliasi tidak ikut dalam analisis SCP/MCP."),
  explicit: tk("Kalimat celah dicari di abstrak. Crossref sering tanpa abstrak; PubMed, Europe PMC, dan arXiv umumnya lengkap."),
  trends: tk("Dokumen tanpa tahun terbit tidak masuk grafik tahunan dan skor momentum."),
};

/** The import source whose documents best support analysis `i`, when the collection mixes sources. */
function bestOrigin(q: MetadataQuality, i: number): string | null {
  const candidates = q.origins.filter((o) => o.count >= 5);
  if (candidates.length < 2) return null;
  const best = candidates.reduce((a, b) => (b.usable[i] / b.count > a.usable[i] / a.count ? b : a));
  return t("Paling lengkap dari {origin} ({pct} dokumennya dapat dipakai).", { origin: best.origin, pct: formatPct(best.usable[i], best.count) });
}

export function ReadinessPanel({ quality }: { quality: MetadataQuality }) {
  return (
    <section className="pnl">
      <div className="ph">
        {t("Kesiapan analisis")}
        <span className="ph-aside">{t("dokumen yang dapat dipakai tiap analisis")}</span>
      </div>
      <ul className="ready-list">
        {quality.analyses.map((a, i) => {
          const usable = quality.origins.reduce((s, o) => s + o.usable[i], 0);
          const level = readiness(usable, quality.total);
          const share = quality.total > 0 ? (usable / quality.total) * 100 : 0;
          return (
            <li key={a.key} className="ready-row">
              <div className="ready-head">
                <strong>{a.label}</strong>
                <span className="num muted ready-tags">{a.needs.join(" / ")}</span>
                <span className="ready-chip" data-level={level}>{READINESS_LABEL[level]}</span>
              </div>
              <div className="ready-meter">
                <div className="ready-bar" aria-hidden="true">
                  <span data-level={level} style={{ width: `${share}%` }} />
                </div>
                <span className="num">{t("{n} dari {total}", { n: formatInt(usable), total: formatInt(quality.total) })}</span>
              </div>
              {level !== "ready" ? (
                <p className="ready-hint">
                  {HINTS[a.key] ? t(HINTS[a.key]) : null} {bestOrigin(quality, i)}
                </p>
              ) : null}
            </li>
          );
        })}
      </ul>
    </section>
  );
}
