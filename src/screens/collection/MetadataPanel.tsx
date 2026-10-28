import { formatInt, formatPct, quality } from "../../lib/format";
import type { CollectionStats } from "../../lib/types";
import { t } from "../../i18n";

interface Props {
  stats: CollectionStats;
  error: string | null;
}

export function MetadataPanel({ stats, error }: Props) {
  const weak = stats.fields.filter((f) => stats.total > 0 && f.missing / stats.total >= 0.2);

  return (
    <aside className="inspector" aria-label={t("Kelengkapan metadata")}>
      <div className="ph">
        {t("Kelengkapan metadata")}
        <span className="muted num" style={{ marginLeft: "auto", fontWeight: 400 }}>
          {t("{n} dok", { n: formatInt(stats.total) })}
        </span>
      </div>
      {error ? <div className="notice" style={{ margin: 12 }} role="alert">{error}</div> : null}
      <div className="meta-row meta-head">
        <span>{t("Tag")}</span>
        <span>{t("Field")}</span>
        <span style={{ textAlign: "right" }}>{t("Hilang")}</span>
        <span>{t("Status")}</span>
      </div>
      {stats.fields.map((f) => {
        const q = quality(f.missing, stats.total);
        return (
          <div className="meta-row" key={f.tag}>
            <span className="num" style={{ fontSize: 11, color: "var(--ink-2)" }}>{f.tag}</span>
            <span>{f.label}</span>
            <span className="num" style={{ textAlign: "right" }}>{formatPct(f.missing, stats.total)}</span>
            <span>
              <span className="status-dot" style={{ background: q.color }} />
              {q.label}
            </span>
          </div>
        );
      })}
      {weak.length ? (
        <div className="notice" style={{ margin: 12, display: "flex", flexDirection: "column", gap: 4 }}>
          <strong style={{ color: "var(--gap-ink)" }}>{t("Perlu perhatian")}</strong>
          <span>
            {t("Field {fields} banyak yang kosong. Analisis yang bergantung pada field ini akan kurang akurat.",
              { fields: weak.map((f) => f.label.toLowerCase()).join(", ") })}
          </span>
        </div>
      ) : null}
      <div style={{ marginTop: "auto", borderTop: "1px solid var(--line-soft)", padding: "10px 12px", fontSize: 12 }}>
        <strong>{t("Aturan dedupe")}</strong>
        <div className="muted" style={{ marginTop: 4 }}>
          {t("Dokumen dilewati bila ID sumber atau DOI sama, atau judul dan tahunnya sama dengan dokumen yang sudah ada.")}
        </div>
      </div>
    </aside>
  );
}
