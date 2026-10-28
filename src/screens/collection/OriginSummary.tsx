import { formatInt } from "../../lib/format";
import type { OriginCount } from "../../lib/types";
import { t } from "../../i18n";

const COLORS: Record<string, string> = {
  OpenAlex: "#2E8B6E",
  Scopus: "#2456A6",
  "Web of Science": "#D9622B",
  PubMed: "#7A5BB0",
  "Europe PMC": "#3B8FB5",
  arXiv: "#6B7480",
  Crossref: "#B8457A",
  "Semantic Scholar": "#8C7A3E",
  RIS: "#4F7F8C",
};

interface Props {
  origins: OriginCount[];
  total: number;
}

export function OriginSummary({ origins, total }: Props) {
  if (total === 0) {
    return <div className="muted" style={{ fontSize: 12 }}>{t("Belum ada dokumen di koleksi.")}</div>;
  }
  return (
    <>
      <div className="origin-bar" role="img" aria-label={t("Proporsi dokumen per sumber impor")}>
        {origins.map((o) => (
          <div
            key={o.origin}
            style={{ width: `${(o.count / total) * 100}%`, background: COLORS[o.origin] ?? "#8a929c" }}
          />
        ))}
      </div>
      <div className="legend">
        {origins.map((o) => (
          <span key={o.origin}>
            <span className="swatch" style={{ background: COLORS[o.origin] ?? "#8a929c" }} />
            {o.origin}
            <span className="num">{formatInt(o.count)}</span>
          </span>
        ))}
      </div>
    </>
  );
}
