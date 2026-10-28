import { countryName } from "../../lib/countryNames";
import { formatInt } from "../../lib/format";
import type { CountryProd } from "../../lib/countriesTypes";
import { t } from "../../i18n";

/** Corresponding author countries split into single-country (SCP) and multi-country (MCP) papers. */
export function CorrespondingChart({ rows }: { rows: CountryProd[] }) {
  const max = Math.max(...rows.map((r) => r.articles), 1);
  return (
    <div className="corr">
      <div className="legend-row muted">
        <span><span className="swatch" style={{ background: "var(--accent)" }} />{t("SCP")}</span>
        <span><span className="swatch" style={{ background: "var(--gap)" }} />{t("MCP")}</span>
      </div>
      {rows.map((r) => (
        <div key={r.code} className="corr-row" title={t("{country}: {scp} SCP, {mcp} MCP", { country: countryName(r.code), scp: formatInt(r.scp), mcp: formatInt(r.mcp) })}>
          <span className="corr-name">{countryName(r.code)}</span>
          <span className="corr-bar">
            <span style={{ width: `${(r.scp / max) * 100}%`, background: "var(--accent)" }} />
            <span style={{ width: `${(r.mcp / max) * 100}%`, background: "var(--gap)" }} />
          </span>
          <span className="num corr-total">{formatInt(r.articles)}</span>
        </div>
      ))}
      <p className="muted bridge-note">
        {t("SCP = publikasi satu negara, MCP = publikasi multi-negara, menurut negara penulis korespondensi. Untuk sumber yang tidak menandai penulis korespondensi, negara penulis pertama yang dipakai.")}
      </p>
    </div>
  );
}
